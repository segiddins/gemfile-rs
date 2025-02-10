use std::fmt::{self, Debug};
use std::io;

use winnow::{
    binary::{self, length_repeat, length_take, u32, u8, Endianness},
    combinator::{alt, empty, eof, fail, repeat, seq, trace},
    error::StrContext,
    token::take_until,
    Bytes, ModalResult, Parser,
};

use crate::prism_ast::generated::parse_node;

use ruby_prism;

#[derive(Debug)]
pub struct Program {
    source: String,
    pub header: Header,
    root: NodeRef,
    nodes: Vec<Node>,
    constants: Vec<Constant>,
    pub content_pool: Vec<u8>,
}

#[enumflags2::bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultNodeFlags {
    NEWLINE = 1,
    STATIC_LITERAL = 2,
}

#[derive(Debug, Clone)]
pub struct Node {
    // pub(super) kind: u8,
    pub(super) identifier: u32,
    pub(super) location: Location,
    pub(super) node_kind: super::generated::NodeKind,
}

impl Program {
    pub fn parse(source: String) -> anyhow::Result<Program> {
        let parse_result = ruby_prism::parse(source.as_bytes());
        let serialized = parse_result.serialize();
        drop(parse_result);
        let mut state = State::default();
        let stream = winnow::Stateful {
            input: Bytes::new(&serialized),
            state: &mut state,
        };
        parse_program
            .complete_err()
            .parse(stream)
            .map_err(|error| {
                anyhow::anyhow!(
                    "failed to parse program: {:?} at {:?}:\n{error:?}",
                    error.inner().context().collect::<Vec<_>>(),
                    error.offset(),
                    // Bytes::new(&serialized[error.offset()..]),
                )
            })
            .map(|(header, root, constants, content_pool)| Program {
                source,
                header,
                root,
                nodes: state.nodes,
                constants,
                content_pool,
            })
    }

    pub fn node(&self, node_ref: &NodeRef) -> &Node {
        &self.nodes[node_ref.0 as usize]
    }

    pub fn constant(&self, constant_ref: &ConstantRef) -> &Constant {
        &self.constants[constant_ref.0 as usize - 1]
    }

    pub fn root(&self) -> &Node {
        self.node(&self.root)
    }

    pub fn source(&self, location: &Location) -> &str {
        &self.source[location.start as usize..(location.start + location.length) as usize]
    }

    pub fn line_column(&self, cursor: u32) -> (i32, u32) {
        let start_line = self.header.start_line;
        debug_assert!(cursor >= self.header.newline_offsets[0]);
        let offset = (cursor - self.header.newline_offsets[0]) as i64;

        let mut left = 0i64;
        let mut right = self.header.newline_offsets.len() as i64 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_offset = self.header.newline_offsets[mid as usize] as i64;

            if mid_offset == offset {
                return (mid as i32 + start_line, 0);
            } else if mid_offset < offset {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        (
            (left as i32) + start_line - 1,
            (offset as u32 - self.header.newline_offsets[left as usize - 1]),
        )
    }

    pub fn string<'b>(&'_ self, string: &'b StringField) -> String {
        match string {
            StringField::Shared(offset, length) => {
                // TODO: escape ruby chars like # before @
                self.source[*offset as usize..(offset + length) as usize].to_string()
            }
            StringField::Owned(s) => s.to_string(),
        }
    }
}

pub(super) type Stream<'i> = winnow::Stateful<&'i Bytes, &'i mut State>;

#[derive(Debug)]
struct Comment {
    r#type: u8,
    location: Location,
}

#[derive(Debug)]
struct MagicComment {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub start: u32,
    pub length: u32,
}

impl Location {
    pub fn snapshot(&self, program: &Program, f: &mut dyn io::Write) -> io::Result<()> {
        let offsets = &program.header.newline_offsets;
        let start_line = offsets
            .iter()
            .position(|&offset| offset >= self.start)
            .unwrap_or(offsets.len());

        if self.length == 0 {
            let line = start_line + 1;
            let col = self.start - program.header.newline_offsets[start_line];
            return write!(f, "({line},{col})-({line},{col})",);
        }

        let end_line = offsets[start_line..]
            .iter()
            .rposition(|&offset| offset < self.start + self.length)
            .unwrap_or(0)
            + start_line;

        let end_line_offset = match end_line {
            0 => 0,
            _ => offsets[end_line - 1],
        };

        write!(
            f,
            "({},{})-({},{})",
            start_line + 1,
            self.start - program.header.newline_offsets[start_line],
            end_line + 1,
            (self.start + self.length - end_line_offset - 1)
        )
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Location {{ start: {}, length: {} }}",
            self.start, self.length,
        )
    }
}

#[derive(Debug)]
struct Error {
    r#type: u32,
    message: String,
    location: Location,
    level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    is_negative: bool,
    // least significant word first
    words: Vec<u32>,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.words.is_empty() || self.words.len() == 1 && self.words[0] == 0 {
            return write!(f, "0");
        }

        if self.is_negative {
            write!(f, "-")?;
        }

        fmt::Display::fmt(&num_bigint::BigUint::from_slice(&self.words), f)
    }
}

#[derive(Debug)]
struct Warning {
    r#type: u32,
    message: String,
    location: Location,
    level: u8,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeRef(u32);

impl Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeRef({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ConstantRef(u32);

impl Debug for ConstantRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConstantRef({})", self.0)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum StringField {
    Shared(u32, u32),
    Owned(String),
}

impl Debug for StringField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StringField::Shared(offset, length) => {
                write!(f, "Shared({offset}, {length})")
            }
            StringField::Owned(s) => {
                write!(f, "Owned({s:?})")
            }
        }
    }
}

#[derive(Debug)]
pub struct Header {
    prism_major: u8,
    prism_minor: u8,
    prism_patch: u8,
    only_semantics_serialized: bool,
    encoding_name: String,
    start_line: i32,
    pub newline_offsets: Vec<u32>,
    comments: Vec<Comment>,
    magic_comments: Vec<MagicComment>,
    end_keyword: Option<Location>,
    errors: Vec<Error>,
    warnings: Vec<Warning>,
    content_pool_offset: u32,
    content_pool_size: u32,
}

pub(super) fn parse_string(input: &mut Stream) -> ModalResult<String> {
    length_take(parse_varuint)
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .context(StrContext::Label("string"))
        .parse_next(input)
}

pub(super) fn parse_string_field(input: &mut Stream) -> ModalResult<StringField> {
    winnow::combinator::dispatch!( winnow::token::any;
      1 => (parse_varuint, parse_varuint).map(|(o, l)| StringField::Shared(o, l)), //
      2 => parse_string.map(StringField::Owned),
      _ => fail,
    )
    .parse_next(input)
}

// A variable-length unsigned integer with the value fitting in `uint32_t` using between 1 and 5 bytes, using the [LEB128](https://en.wikipedia.org/wiki/LEB128) encoding.
pub(super) fn parse_varuint(i: &mut Stream) -> ModalResult<u32> {
    trace("parse_varuint", move |input: &mut _| {
        let mut result: u32 = 0;
        let mut shift: u32 = 0;
        loop {
            let byte = u8.parse_next(input)?;
            result |= ((byte & 0b0111_1111) as u32) << shift;
            if byte < 0b1000_0000 {
                break;
            }
            shift += 7;
        }

        Ok(result)
    })
    .parse_next(i)
}

#[derive(Clone)]
pub enum Constant {
    Owned(u32, u32),
    Shared(u32, u32),
}

impl Debug for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Owned(offset, length) => {
                write!(f, "Owned({}, {})", offset, length)
            }
            Constant::Shared(offset, length) => {
                write!(f, "Shared({}, {})", offset, length)
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    nodes: Vec<Node>,
}

impl State {
    pub fn add_node(&mut self, node: Node) -> NodeRef {
        let identifier = self.nodes.len() as u32;
        self.nodes.push(node);
        NodeRef(identifier)
    }
}

fn parse_varsint(input: &mut Stream) -> ModalResult<i32> {
    parse_varuint
        .map(|value| {
            let sign = value & 1;
            let value = value >> 1;
            if sign == 1 {
                -(value as i32)
            } else {
                value as i32
            }
        })
        .parse_next(input)
}

pub(super) fn parse_optional_node(input: &mut Stream) -> ModalResult<Option<NodeRef>> {
    alt((
        u8.verify(|value| *value == 0).map(|_| None),
        parse_node.map(Some),
    ))
    .parse_next(input)
}

pub(super) fn parse_integer(input: &mut Stream) -> ModalResult<Integer> {
    seq![Integer {
        is_negative: parse_bool,
        words: length_repeat(parse_varuint, parse_varuint),
    }]
    .context(StrContext::Label("integer"))
    .parse_next(input)
}

pub(super) fn parse_constant(input: &mut Stream) -> ModalResult<ConstantRef> {
    parse_varuint.map(ConstantRef).parse_next(input)
}

pub(super) fn parse_optional_constant(input: &mut Stream) -> ModalResult<Option<ConstantRef>> {
    alt((
        u8.verify(|value| *value == 0).map(|_| None),
        parse_constant.map(Some),
    ))
    .parse_next(input)
}

pub(super) fn parse_bool(input: &mut Stream) -> ModalResult<bool> {
    u8.map(|value| value == 1).parse_next(input)
}

fn parse_comment(input: &mut Stream) -> ModalResult<Comment> {
    seq![Comment {
        r#type: u8,
        location: parse_location,
    }]
    .parse_next(input)
}

fn parse_magic_comment(input: &mut Stream) -> ModalResult<MagicComment> {
    seq![MagicComment {
        _: parse_location,
        _: parse_location,
    }]
    .parse_next(input)
}
pub(super) fn parse_double(input: &mut Stream) -> ModalResult<f64> {
    binary::f64(Endianness::Native).parse_next(input)
}

fn parse_content(input: &mut Stream) -> ModalResult<Constant> {
    (u32(Endianness::Native), u32(Endianness::Native))
        .map(|(offset, length)| {
            if offset & (1 << 31) == 0 {
                Constant::Owned(offset, length)
            } else {
                Constant::Shared(offset ^ (1 << 31), length)
            }
        })
        .parse_next(input)
}

fn parse_error(input: &mut Stream) -> ModalResult<Error> {
    seq![Error {
        r#type: parse_varuint,
        message: parse_string,
        location: parse_location,
        level: u8
    }]
    .parse_next(input)
}

fn parse_header(input: &mut Stream) -> ModalResult<Header> {
    seq![Header {
        _: "PRISM".context(winnow::error::StrContext::Expected("PRISM".into())),
        prism_major: u8,
        prism_minor: u8,
        prism_patch: u8,
        only_semantics_serialized: parse_bool,
        encoding_name: parse_string,
        start_line: parse_varsint,
        newline_offsets: length_repeat(parse_varuint, parse_varuint),
        comments: length_repeat(parse_varuint, parse_comment),
        magic_comments: length_repeat(parse_varuint, parse_magic_comment),
        end_keyword: parse_optional_location,
        errors: length_repeat(parse_varuint, parse_error),
        warnings: length_repeat(parse_varuint, parse_error.map(|e| Warning {
            r#type: e.r#type,
            message: e.message,
            location: e.location,
            level: e.level,
        })),
        content_pool_offset: binary::u32(Endianness::Native),
        content_pool_size: parse_varuint,
    }]
    .parse_next(input)
}

fn parse_program(input: &mut Stream) -> ModalResult<(Header, NodeRef, Vec<Constant>, Vec<u8>)> {
    let (header, root) = (parse_header, parse_node).parse_next(input)?;

    let constants = repeat(
        header.content_pool_size as usize,
        parse_content.map(|c| match c {
            Constant::Owned(_, _) => c,
            Constant::Shared(offset, length) => Constant::Shared(
                offset - header.content_pool_offset - (8 * header.content_pool_size),
                length,
            ),
        }),
    )
    .parse_next(input)?;
    let content_pool = take_until(0.., b'\0')
        .map(|s: &[u8]| s.to_vec())
        .parse_next(input)?;
    // program.constants = input.state.constants.clone();
    b'\0'.parse_next(input)?;
    eof.parse_next(input)?;
    Ok((header, root, constants, content_pool))
}

pub(super) fn parse_location(input: &mut Stream) -> ModalResult<Location> {
    seq![Location {
        start: parse_varuint,
        length: parse_varuint
    }]
    .parse_next(input)
}

pub(super) fn parse_optional_location(input: &mut Stream) -> ModalResult<Option<Location>> {
    winnow::combinator::dispatch!( winnow::token::any;
      0u8 => empty.value(None),
      1 => parse_location.map(Some),
      _ => fail,
    )
    .parse_next(input)
}
