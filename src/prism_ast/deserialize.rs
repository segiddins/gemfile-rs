use std::borrow::Cow;
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

enum NodeFieldType {
    NodeRef,
    ConstantRef,
    Location,
    String,
    Integer,
    Bool,
}

pub(super) struct LocationSnapshot<'a> {
    pub program: &'a Program,
    pub location: &'a Location,
}

impl Debug for LocationSnapshot<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        eprintln!(
            "loc: {:?}, offsets: {:?}",
            self.location, self.program.header.newline_offsets
        );

        let offsets = &self.program.header.newline_offsets;
        let start_line = offsets
            .iter()
            .position(|&offset| offset >= self.location.start)
            .unwrap_or(offsets.len());

        if self.location.length == 0 {
            let line = start_line + 1;
            let col = self.location.start - self.program.header.newline_offsets[start_line];
            return write!(f, "({line},{col})-({line},{col})",);
        }

        let end_line = offsets[start_line..]
            .iter()
            .rposition(|&offset| offset < self.location.start + self.location.length)
            .unwrap_or(0)
            + start_line;

        let end_line_offset = match end_line {
            0 => 0,
            _ => offsets[end_line - 1],
        };

        eprintln!(
            "loc: {:?}, start_line: {}, end_line: {}, end_line_offset: {}",
            self.location, start_line, end_line, end_line_offset
        );

        write!(
            f,
            "({},{})-({},{})",
            start_line + 1,
            self.location.start - self.program.header.newline_offsets[start_line],
            end_line + 1,
            (self.location.start + self.location.length - end_line_offset - 1)
        )
    }
}

impl Program {
    pub fn parse(source: String) -> anyhow::Result<Program> {
        let parse_result = ruby_prism::parse(source.as_bytes());
        let serialized = parse_result.serialize();
        drop(parse_result);
        let state = State::default();
        let stream = winnow::Stateful {
            input: Bytes::new(&serialized),
            state,
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
            .map(move |mut program| {
                program.source = source;
                program
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

    pub fn string<'b>(&'_ self, string: &'b StringField) -> Cow<'b, String> {
        match string {
            StringField::Shared(offset, length) => Cow::Owned({
                // TODO: escape ruby chars like # before @
                self.source[*offset as usize..(offset + length) as usize].to_string()
            }),
            StringField::Owned(s) => Cow::Owned(s.escape_default().to_string()),
        }
    }
}

pub(super) type Stream<'i> = winnow::Stateful<&'i Bytes, State>;

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
    words: Vec<u32>,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative {
            write!(f, "-")?;
        }

        match self.words.len() {
            0 => write!(f, "0"),
            1 => write!(f, "{}", self.words[0]),
            2 => write!(
                f,
                "{}",
                ((self.words[0] as u64) << 32 | self.words[1] as u64)
            ),
            _ => write!(f, "0x{:x?}", self.words),
        }
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

#[derive(Debug)]
struct NonZeroFlagsError(u32);

impl std::error::Error for NonZeroFlagsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl fmt::Display for NonZeroFlagsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected zero flags, got {}", self.0)
    }
}

pub(super) fn zero_flags(input: &mut Stream) -> ModalResult<u32> {
    parse_varuint
        // .try_map(|f| match f {
        //     0 => Ok(f),
        //     _ => Err(NonZeroFlagsError(f)),
        // })
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

// pub(super) fn parse_option<Input, Output, Error, ParseNext>(
//     parser: ParseNext,
// ) -> impl Parser<Input, Option<Output>, Error>
// where
//     Input: winnow::stream::Stream<Token = u8> + winnow::stream::StreamIsPartial,
//     Error: winnow::error::ParserError<Input>,
//     ParseNext: Parser<Input, Output, Error>,
// {
//     winnow::combinator::alt((
//         winnow::binary::u8.verify(|value| *value == 0).map(|_| None),
//         (
//             winnow::binary::u8.verify(|value| *value == 1),
//             parser.map(Some),
//         )
//             .map(|(_, value)| value),
//         fail,
//     ))
// }

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

fn parse_program(input: &mut Stream) -> ModalResult<Program> {
    let mut program = seq![Program {
        source: empty.default_value(),
        header: parse_header,
        root: parse_node,
        nodes: empty.default_value(),
        // _: empty.span(),
        // TODO: verify at the location of the constant pool
        constants: repeat(0..0, parse_content),
        content_pool: empty.default_value(),
        // _: eof,
    }]
    .parse_next(input)?;

    program.nodes = input.state.nodes.clone();
    program.constants = repeat(
        program.header.content_pool_size as usize,
        parse_content.map(|c| match c {
            Constant::Owned(_, _) => c,
            Constant::Shared(offset, length) => Constant::Shared(
                offset
                    - program.header.content_pool_offset
                    - (8 * program.header.content_pool_size),
                length,
            ),
        }),
    )
    .parse_next(input)?;
    program.content_pool = take_until(0.., b'\0')
        .map(|s: &[u8]| s.to_vec())
        .parse_next(input)?;
    // program.constants = input.state.constants.clone();
    b'\0'.parse_next(input)?;
    eof.parse_next(input)?;
    Ok(program)
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
      x => fail.context(winnow::error::StrContext::Expected(winnow::error::StrContextValue::CharLiteral(x as char))),
    )
    .parse_next(input)
}

#[cfg(test)]
fn input(bytes: &[u8]) -> Stream {
    use winnow::stream::StreamIsPartial;

    let input = Bytes::new(bytes);
    let state = State::default();
    let mut stream = winnow::Stateful { input, state };
    stream.complete();
    stream
}

#[test]
fn test_parse_varuint() {
    {
        let mut input = input(&[0xE5, 0x8E, 0x26]);
        let result = parse_varuint(&mut input);

        expect_test::expect![[r#"
        Ok(
            624485,
        )
    "#]]
        .assert_debug_eq(&result);
    }
    {
        let mut input = input(&[0x4D]);
        let result = parse_varuint(&mut input);

        expect_test::expect![[r#"
            Ok(
                77,
            )
        "#]]
        .assert_debug_eq(&result);
    }
}

#[test]
fn test_parse_string() {
    let mut input = input(&[0x01, b'a']);
    let result = parse_string(&mut input);

    expect_test::expect![[r#"
        Ok(
            "a",
        )
    "#]]
    .assert_debug_eq(&result);
}

#[test]
fn test_parse_empty() {
    let program = Program::parse("".to_string());

    expect_test::expect![[r#"
        Ok(
            Program {
                source: "",
                header: Header {
                    prism_major: 1,
                    prism_minor: 3,
                    prism_patch: 0,
                    only_semantics_serialized: false,
                    encoding_name: "UTF-8",
                    start_line: 1,
                    newline_offsets: [
                        0,
                    ],
                    comments: [],
                    magic_comments: [],
                    end_keyword: None,
                    errors: [],
                    warnings: [],
                    content_pool_offset: 40,
                    content_pool_size: 0,
                },
                root: NodeRef(1),
                nodes: [
                    Node {
                        identifier: 1,
                        location: Location { start: 0, length: 0 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [],
                            },
                        ),
                    },
                    Node {
                        identifier: 2,
                        location: Location { start: 0, length: 0 },
                        node_kind: ProgramNode(
                            ProgramNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                locals: [],
                                statements: NodeRef(0),
                            },
                        ),
                    },
                ],
                constants: [],
                content_pool: [],
            },
        )
    "#]]
    .assert_debug_eq(&program);
}

#[test]
fn test_parse_gemfile() {
    let program = Program::parse(
        r#"_ = "foo"
_ = "\nbar\t"
"\u{1F600}"

source "https://rubygems.org"

gem "prism", "1.3.0"
    "#
        .to_string(),
    );

    expect_test::expect![[r#"
        Ok(
            Program {
                source: "_ = \"foo\"\n_ = \"\\nbar\\t\"\n\"\\u{1F600}\"\n\nsource \"https://rubygems.org\"\n\ngem \"prism\", \"1.3.0\"\n    ",
                header: Header {
                    prism_major: 1,
                    prism_minor: 3,
                    prism_patch: 0,
                    only_semantics_serialized: false,
                    encoding_name: "UTF-8",
                    start_line: 1,
                    newline_offsets: [
                        0,
                        10,
                        24,
                        36,
                        37,
                        67,
                        68,
                        89,
                    ],
                    comments: [],
                    magic_comments: [],
                    end_keyword: None,
                    errors: [],
                    warnings: [
                        Warning {
                            type: 318,
                            message: "possibly useless use of a literal in void context",
                            location: Location { start: 24, length: 11 },
                            level: 1,
                        },
                    ],
                    content_pool_offset: 268,
                    content_pool_size: 3,
                },
                root: NodeRef(13),
                nodes: [
                    Node {
                        identifier: 3,
                        location: Location { start: 4, length: 5 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b0,
                                },
                                opening_loc: Some(
                                    Location { start: 4, length: 1 },
                                ),
                                content_loc: Location { start: 5, length: 3 },
                                closing_loc: Some(
                                    Location { start: 8, length: 1 },
                                ),
                                unescaped: Shared(5, 3),
                            },
                        ),
                    },
                    Node {
                        identifier: 4,
                        location: Location { start: 0, length: 9 },
                        node_kind: LocalVariableWriteNode(
                            LocalVariableWriteNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(1),
                                depth: 0,
                                name_loc: Location { start: 0, length: 1 },
                                value: NodeRef(0),
                                operator_loc: Location { start: 2, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 6,
                        location: Location { start: 14, length: 9 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b0,
                                },
                                opening_loc: Some(
                                    Location { start: 14, length: 1 },
                                ),
                                content_loc: Location { start: 15, length: 7 },
                                closing_loc: Some(
                                    Location { start: 22, length: 1 },
                                ),
                                unescaped: Owned("\nbar\t"),
                            },
                        ),
                    },
                    Node {
                        identifier: 7,
                        location: Location { start: 10, length: 13 },
                        node_kind: LocalVariableWriteNode(
                            LocalVariableWriteNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(1),
                                depth: 0,
                                name_loc: Location { start: 10, length: 1 },
                                value: NodeRef(2),
                                operator_loc: Location { start: 12, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 8,
                        location: Location { start: 24, length: 11 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b101,
                                    flags: NEWLINE | FORCED_UTF8_ENCODING,
                                },
                                opening_loc: Some(
                                    Location { start: 24, length: 1 },
                                ),
                                content_loc: Location { start: 25, length: 9 },
                                closing_loc: Some(
                                    Location { start: 34, length: 1 },
                                ),
                                unescaped: Owned("😀"),
                            },
                        ),
                    },
                    Node {
                        identifier: 10,
                        location: Location { start: 44, length: 22 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b0,
                                },
                                opening_loc: Some(
                                    Location { start: 44, length: 1 },
                                ),
                                content_loc: Location { start: 45, length: 20 },
                                closing_loc: Some(
                                    Location { start: 65, length: 1 },
                                ),
                                unescaped: Shared(45, 20),
                            },
                        ),
                    },
                    Node {
                        identifier: 11,
                        location: Location { start: 44, length: 22 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b0,
                                },
                                arguments: [
                                    NodeRef(5),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 9,
                        location: Location { start: 37, length: 29 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b100001,
                                    flags: NEWLINE | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(2),
                                message_loc: Some(
                                    Location { start: 37, length: 6 },
                                ),
                                opening_loc: None,
                                arguments: Some(
                                    NodeRef(6),
                                ),
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 13,
                        location: Location { start: 72, length: 7 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b0,
                                },
                                opening_loc: Some(
                                    Location { start: 72, length: 1 },
                                ),
                                content_loc: Location { start: 73, length: 5 },
                                closing_loc: Some(
                                    Location { start: 78, length: 1 },
                                ),
                                unescaped: Shared(73, 5),
                            },
                        ),
                    },
                    Node {
                        identifier: 15,
                        location: Location { start: 81, length: 7 },
                        node_kind: StringNode(
                            StringNode {
                                flags: BitFlags<StringFlags> {
                                    bits: 0b0,
                                },
                                opening_loc: Some(
                                    Location { start: 81, length: 1 },
                                ),
                                content_loc: Location { start: 82, length: 5 },
                                closing_loc: Some(
                                    Location { start: 87, length: 1 },
                                ),
                                unescaped: Shared(82, 5),
                            },
                        ),
                    },
                    Node {
                        identifier: 14,
                        location: Location { start: 72, length: 16 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b0,
                                },
                                arguments: [
                                    NodeRef(8),
                                    NodeRef(9),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 12,
                        location: Location { start: 68, length: 20 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b100001,
                                    flags: NEWLINE | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(3),
                                message_loc: Some(
                                    Location { start: 68, length: 3 },
                                ),
                                opening_loc: None,
                                arguments: Some(
                                    NodeRef(10),
                                ),
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 1,
                        location: Location { start: 0, length: 88 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(1),
                                    NodeRef(3),
                                    NodeRef(4),
                                    NodeRef(7),
                                    NodeRef(11),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 16,
                        location: Location { start: 0, length: 88 },
                        node_kind: ProgramNode(
                            ProgramNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                locals: [
                                    ConstantRef(1),
                                ],
                                statements: NodeRef(12),
                            },
                        ),
                    },
                ],
                constants: [
                    Owned(0, 1),
                    Owned(37, 6),
                    Owned(68, 3),
                ],
                content_pool: [],
            },
        )
    "#]]
    .assert_debug_eq(&program);
}

#[test]
fn test_parse_def() {
    let program = Program::parse(
        r#"def f(*); a[*]; end

def f(*); a[1, *]; end

def f(*); a[*] = 1; end

def f(*); a[1, *] = 1; end

def f(*); a[*] += 1; end

def f(*); a[1, *] &&= 1; end

def f(*); rescue => a[*]; end

def f(*); rescue => a[1, *]; end
"#
        .to_string(),
    );

    expect_test::expect![[r#"
        Ok(
            Program {
                source: "def f(*); a[*]; end\n\ndef f(*); a[1, *]; end\n\ndef f(*); a[*] = 1; end\n\ndef f(*); a[1, *] = 1; end\n\ndef f(*); a[*] += 1; end\n\ndef f(*); a[1, *] &&= 1; end\n\ndef f(*); rescue => a[*]; end\n\ndef f(*); rescue => a[1, *]; end\n",
                header: Header {
                    prism_major: 1,
                    prism_minor: 3,
                    prism_patch: 0,
                    only_semantics_serialized: false,
                    encoding_name: "UTF-8",
                    start_line: 1,
                    newline_offsets: [
                        0,
                        20,
                        21,
                        44,
                        45,
                        69,
                        70,
                        97,
                        98,
                        123,
                        124,
                        153,
                        154,
                        184,
                        185,
                        218,
                    ],
                    comments: [],
                    magic_comments: [],
                    end_keyword: None,
                    errors: [],
                    warnings: [],
                    content_pool_offset: 1009,
                    content_pool_size: 5,
                },
                root: NodeRef(75),
                nodes: [
                    Node {
                        identifier: 3,
                        location: Location { start: 6, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 6, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 2,
                        location: Location { start: 6, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(0),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 5,
                        location: Location { start: 10, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 10, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 6,
                        location: Location { start: 12, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 12, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 7,
                        location: Location { start: 12, length: 1 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(3),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 8,
                        location: Location { start: 10, length: 4 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                receiver: Some(
                                    NodeRef(2),
                                ),
                                call_operator_loc: None,
                                name: ConstantRef(2),
                                message_loc: Some(
                                    Location { start: 11, length: 3 },
                                ),
                                opening_loc: Some(
                                    Location { start: 11, length: 1 },
                                ),
                                arguments: Some(
                                    NodeRef(4),
                                ),
                                closing_loc: Some(
                                    Location { start: 13, length: 1 },
                                ),
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 4,
                        location: Location { start: 10, length: 4 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(5),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 9,
                        location: Location { start: 0, length: 19 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 4, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(1),
                                ),
                                body: Some(
                                    NodeRef(6),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 0, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 5, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 7, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 16, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 11,
                        location: Location { start: 27, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 27, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 10,
                        location: Location { start: 27, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(8),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 13,
                        location: Location { start: 31, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 31, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 14,
                        location: Location { start: 33, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 16,
                        location: Location { start: 36, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 36, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 15,
                        location: Location { start: 33, length: 4 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(11),
                                    NodeRef(12),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 17,
                        location: Location { start: 31, length: 7 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                receiver: Some(
                                    NodeRef(10),
                                ),
                                call_operator_loc: None,
                                name: ConstantRef(2),
                                message_loc: Some(
                                    Location { start: 32, length: 6 },
                                ),
                                opening_loc: Some(
                                    Location { start: 32, length: 1 },
                                ),
                                arguments: Some(
                                    NodeRef(13),
                                ),
                                closing_loc: Some(
                                    Location { start: 37, length: 1 },
                                ),
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 12,
                        location: Location { start: 31, length: 7 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(14),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 18,
                        location: Location { start: 21, length: 22 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 25, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(9),
                                ),
                                body: Some(
                                    NodeRef(15),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 21, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 26, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 28, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 40, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 20,
                        location: Location { start: 51, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 51, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 19,
                        location: Location { start: 51, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(17),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 22,
                        location: Location { start: 55, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 55, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 23,
                        location: Location { start: 57, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 57, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 26,
                        location: Location { start: 62, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 24,
                        location: Location { start: 57, length: 6 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(20),
                                    NodeRef(21),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 25,
                        location: Location { start: 55, length: 8 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b10001,
                                    flags: NEWLINE | ATTRIBUTE_WRITE,
                                },
                                receiver: Some(
                                    NodeRef(19),
                                ),
                                call_operator_loc: None,
                                name: ConstantRef(4),
                                message_loc: Some(
                                    Location { start: 56, length: 3 },
                                ),
                                opening_loc: Some(
                                    Location { start: 56, length: 1 },
                                ),
                                arguments: Some(
                                    NodeRef(22),
                                ),
                                closing_loc: Some(
                                    Location { start: 58, length: 1 },
                                ),
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 21,
                        location: Location { start: 55, length: 8 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(23),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 27,
                        location: Location { start: 45, length: 23 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 49, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(18),
                                ),
                                body: Some(
                                    NodeRef(24),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 45, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 50, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 52, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 65, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 29,
                        location: Location { start: 76, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 76, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 28,
                        location: Location { start: 76, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(26),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 31,
                        location: Location { start: 80, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 80, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 32,
                        location: Location { start: 82, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 34,
                        location: Location { start: 85, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 85, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 36,
                        location: Location { start: 90, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 33,
                        location: Location { start: 82, length: 9 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(29),
                                    NodeRef(30),
                                    NodeRef(31),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 35,
                        location: Location { start: 80, length: 11 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b10001,
                                    flags: NEWLINE | ATTRIBUTE_WRITE,
                                },
                                receiver: Some(
                                    NodeRef(28),
                                ),
                                call_operator_loc: None,
                                name: ConstantRef(4),
                                message_loc: Some(
                                    Location { start: 81, length: 6 },
                                ),
                                opening_loc: Some(
                                    Location { start: 81, length: 1 },
                                ),
                                arguments: Some(
                                    NodeRef(32),
                                ),
                                closing_loc: Some(
                                    Location { start: 86, length: 1 },
                                ),
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 30,
                        location: Location { start: 80, length: 11 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(33),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 37,
                        location: Location { start: 70, length: 26 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 74, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(27),
                                ),
                                body: Some(
                                    NodeRef(34),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 70, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 75, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 77, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 93, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 39,
                        location: Location { start: 104, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 104, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 38,
                        location: Location { start: 104, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(36),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 41,
                        location: Location { start: 108, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 108, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 42,
                        location: Location { start: 110, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 110, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 43,
                        location: Location { start: 110, length: 1 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(39),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 45,
                        location: Location { start: 116, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 46,
                        location: Location { start: 108, length: 9 },
                        node_kind: IndexOperatorWriteNode(
                            IndexOperatorWriteNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                receiver: Some(
                                    NodeRef(38),
                                ),
                                call_operator_loc: None,
                                opening_loc: Location { start: 109, length: 1 },
                                arguments: Some(
                                    NodeRef(40),
                                ),
                                closing_loc: Location { start: 111, length: 1 },
                                block: None,
                                binary_operator: ConstantRef(5),
                                binary_operator_loc: Location { start: 113, length: 2 },
                                value: NodeRef(41),
                            },
                        ),
                    },
                    Node {
                        identifier: 40,
                        location: Location { start: 108, length: 9 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(42),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 47,
                        location: Location { start: 98, length: 24 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 102, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(37),
                                ),
                                body: Some(
                                    NodeRef(43),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 98, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 103, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 105, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 119, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 49,
                        location: Location { start: 130, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 130, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 48,
                        location: Location { start: 130, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(45),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 51,
                        location: Location { start: 134, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 134, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 52,
                        location: Location { start: 136, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 54,
                        location: Location { start: 139, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 139, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 53,
                        location: Location { start: 136, length: 4 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(48),
                                    NodeRef(49),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 56,
                        location: Location { start: 146, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 57,
                        location: Location { start: 134, length: 13 },
                        node_kind: IndexAndWriteNode(
                            IndexAndWriteNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                receiver: Some(
                                    NodeRef(47),
                                ),
                                call_operator_loc: None,
                                opening_loc: Location { start: 135, length: 1 },
                                arguments: Some(
                                    NodeRef(50),
                                ),
                                closing_loc: Location { start: 140, length: 1 },
                                block: None,
                                operator_loc: Location { start: 142, length: 3 },
                                value: NodeRef(51),
                            },
                        ),
                    },
                    Node {
                        identifier: 50,
                        location: Location { start: 134, length: 13 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(52),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 58,
                        location: Location { start: 124, length: 28 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 128, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(46),
                                ),
                                body: Some(
                                    NodeRef(53),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 124, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 129, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 131, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 149, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 60,
                        location: Location { start: 160, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 160, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 59,
                        location: Location { start: 160, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(55),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 63,
                        location: Location { start: 174, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 174, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 64,
                        location: Location { start: 176, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 176, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 65,
                        location: Location { start: 176, length: 1 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(58),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 67,
                        location: Location { start: 174, length: 4 },
                        node_kind: IndexTargetNode(
                            IndexTargetNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b10000,
                                    flags: ATTRIBUTE_WRITE,
                                },
                                receiver: NodeRef(57),
                                opening_loc: Location { start: 175, length: 1 },
                                arguments: Some(
                                    NodeRef(59),
                                ),
                                closing_loc: Location { start: 177, length: 1 },
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 62,
                        location: Location { start: 164, length: 14 },
                        node_kind: RescueNode(
                            RescueNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                keyword_loc: Location { start: 164, length: 6 },
                                exceptions: [],
                                operator_loc: Some(
                                    Location { start: 171, length: 2 },
                                ),
                                reference: Some(
                                    NodeRef(60),
                                ),
                                then_keyword_loc: None,
                                statements: None,
                                subsequent: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 61,
                        location: Location { start: 154, length: 29 },
                        node_kind: BeginNode(
                            BeginNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                begin_keyword_loc: None,
                                statements: None,
                                rescue_clause: Some(
                                    NodeRef(61),
                                ),
                                else_clause: None,
                                ensure_clause: None,
                                end_keyword_loc: Some(
                                    Location { start: 180, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 68,
                        location: Location { start: 154, length: 29 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 158, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(56),
                                ),
                                body: Some(
                                    NodeRef(62),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 154, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 159, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 161, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 180, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 70,
                        location: Location { start: 191, length: 1 },
                        node_kind: RestParameterNode(
                            RestParameterNode {
                                flags: BitFlags<ParameterFlags> {
                                    bits: 0b0,
                                },
                                name: None,
                                name_loc: None,
                                operator_loc: Location { start: 191, length: 1 },
                            },
                        ),
                    },
                    Node {
                        identifier: 69,
                        location: Location { start: 191, length: 1 },
                        node_kind: ParametersNode(
                            ParametersNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                requireds: [],
                                optionals: [],
                                rest: Some(
                                    NodeRef(64),
                                ),
                                posts: [],
                                keywords: [],
                                keyword_rest: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 73,
                        location: Location { start: 205, length: 1 },
                        node_kind: CallNode(
                            CallNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b101000,
                                    flags: VARIABLE_CALL | IGNORE_VISIBILITY,
                                },
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(1),
                                message_loc: Some(
                                    Location { start: 205, length: 1 },
                                ),
                                opening_loc: None,
                                arguments: None,
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 74,
                        location: Location { start: 207, length: 1 },
                        node_kind: IntegerNode(
                            IntegerNode {
                                flags: BitFlags<IntegerBaseFlags> {
                                    bits: 0b1010,
                                    flags: STATIC_LITERAL | DECIMAL,
                                },
                                value: Integer {
                                    is_negative: false,
                                    words: [
                                        1,
                                    ],
                                },
                            },
                        ),
                    },
                    Node {
                        identifier: 76,
                        location: Location { start: 210, length: 1 },
                        node_kind: SplatNode(
                            SplatNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                operator_loc: Location { start: 210, length: 1 },
                                expression: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 75,
                        location: Location { start: 207, length: 4 },
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                flags: BitFlags<ArgumentsNodeFlags> {
                                    bits: 0b100000,
                                    flags: CONTAINS_SPLAT,
                                },
                                arguments: [
                                    NodeRef(67),
                                    NodeRef(68),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 78,
                        location: Location { start: 205, length: 7 },
                        node_kind: IndexTargetNode(
                            IndexTargetNode {
                                flags: BitFlags<CallNodeFlags> {
                                    bits: 0b10000,
                                    flags: ATTRIBUTE_WRITE,
                                },
                                receiver: NodeRef(66),
                                opening_loc: Location { start: 206, length: 1 },
                                arguments: Some(
                                    NodeRef(69),
                                ),
                                closing_loc: Location { start: 211, length: 1 },
                                block: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 72,
                        location: Location { start: 195, length: 17 },
                        node_kind: RescueNode(
                            RescueNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                keyword_loc: Location { start: 195, length: 6 },
                                exceptions: [],
                                operator_loc: Some(
                                    Location { start: 202, length: 2 },
                                ),
                                reference: Some(
                                    NodeRef(70),
                                ),
                                then_keyword_loc: None,
                                statements: None,
                                subsequent: None,
                            },
                        ),
                    },
                    Node {
                        identifier: 71,
                        location: Location { start: 185, length: 32 },
                        node_kind: BeginNode(
                            BeginNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                begin_keyword_loc: None,
                                statements: None,
                                rescue_clause: Some(
                                    NodeRef(71),
                                ),
                                else_clause: None,
                                ensure_clause: None,
                                end_keyword_loc: Some(
                                    Location { start: 214, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 79,
                        location: Location { start: 185, length: 32 },
                        node_kind: DefNode(
                            DefNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b1,
                                    flags: NEWLINE,
                                },
                                name: ConstantRef(3),
                                name_loc: Location { start: 189, length: 1 },
                                receiver: None,
                                parameters: Some(
                                    NodeRef(65),
                                ),
                                body: Some(
                                    NodeRef(72),
                                ),
                                locals: [],
                                def_keyword_loc: Location { start: 185, length: 3 },
                                operator_loc: None,
                                lparen_loc: Some(
                                    Location { start: 190, length: 1 },
                                ),
                                rparen_loc: Some(
                                    Location { start: 192, length: 1 },
                                ),
                                equal_loc: None,
                                end_keyword_loc: Some(
                                    Location { start: 214, length: 3 },
                                ),
                            },
                        ),
                    },
                    Node {
                        identifier: 1,
                        location: Location { start: 0, length: 217 },
                        node_kind: StatementsNode(
                            StatementsNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                body: [
                                    NodeRef(7),
                                    NodeRef(16),
                                    NodeRef(25),
                                    NodeRef(35),
                                    NodeRef(44),
                                    NodeRef(54),
                                    NodeRef(63),
                                    NodeRef(73),
                                ],
                            },
                        ),
                    },
                    Node {
                        identifier: 80,
                        location: Location { start: 0, length: 217 },
                        node_kind: ProgramNode(
                            ProgramNode {
                                flags: BitFlags<DefaultNodeFlags> {
                                    bits: 0b0,
                                },
                                locals: [],
                                statements: NodeRef(74),
                            },
                        ),
                    },
                ],
                constants: [
                    Owned(10, 1),
                    Shared(3, 2),
                    Owned(4, 1),
                    Shared(0, 3),
                    Owned(113, 1),
                ],
                content_pool: [
                    91,
                    93,
                    61,
                    91,
                    93,
                ],
            },
        )
    "#]]
    .assert_debug_eq(&program);
}
