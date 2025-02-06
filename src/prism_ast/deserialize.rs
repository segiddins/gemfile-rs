use std::ascii;
use std::io::Read;

use ruby_prism;

#[derive(Debug)]
struct Program {
    source: String,
    header: Header,
    root: NodeRef,
    nodes: Vec<Node>,
    constants: Vec<Constant>,
    content_pool: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub(super) kind: u8,
    pub(super) identifier: u32,
    pub(super) location: Location,
    pub(super) flags: u32,
    pub(super) node_kind: super::generated::NodeKind,
}

impl Program {
    pub fn parse(source: String) -> anyhow::Result<Program> {
        let parse_result = ruby_prism::parse(source.as_bytes());
        let serialized = parse_result.serialize();
        drop(parse_result);
        let state = State::default();
        let mut stream = winnow::Stateful {
            input: winnow::Partial::new(Bytes::new(&serialized)),
            state,
        };
        parse_program
            .parse_next(&mut stream)
            .map_err(|error| anyhow::anyhow!("failed to parse program: {:#}", error))
    }
}

use winnow::binary::length_repeat;
use winnow::binary::length_take;
use winnow::binary::u32;
use winnow::binary::u8;
use winnow::combinator::empty;
use winnow::combinator::eof;
use winnow::combinator::fail;
use winnow::combinator::repeat;
use winnow::combinator::repeat_till;
use winnow::combinator::seq;
use winnow::combinator::todo;
use winnow::token::take_until;
use winnow::Bytes;
use winnow::ModalResult;
use winnow::Parser;

use crate::evaluator::parse;
use crate::prism_ast::generated::parse_node;

pub(super) type Stream<'i> = winnow::Stateful<winnow::Partial<&'i Bytes>, State>;

#[derive(Debug)]
struct Comment {}

#[derive(Debug)]
struct MagicComment {}

#[derive(Debug, Clone)]
pub struct Location {
    start: u32,
    length: u32,
}

#[derive(Debug)]
struct Error {
    r#type: u32,
    message: String,
    location: Location,
    level: u8,
}

#[derive(Debug, Clone)]
pub struct Integer {}

#[derive(Debug)]
struct Warning {
    r#type: u32,
    message: String,
    location: Location,
    level: u8,
}

#[derive(Debug, Clone)]
pub struct NodeRef(u32);

#[derive(Debug, Clone)]
pub struct ConstantRef(u32);

#[derive(Debug)]
struct Header {
    prism_major: u8,
    prism_minor: u8,
    prism_patch: u8,
    only_semantics_serialized: bool,
    encoding_name: String,
    start_line: i32,
    newline_offsets: Vec<u32>,
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
        .context(winnow::error::StrContext::Label("string"))
        .parse_next(input)
}

pub(super) fn parse_string_field(input: &mut Stream) -> ModalResult<String> {
    winnow::combinator::dispatch!( winnow::token::any;
      1 => (parse_varuint, parse_varuint).value("".to_string()), //
      2 => parse_string,
      _ => fail,
    )
    .parse_next(input)
}

// A variable-length unsigned integer with the value fitting in `uint32_t` using between 1 and 5 bytes, using the [LEB128](https://en.wikipedia.org/wiki/LEB128) encoding.
pub(super) fn parse_varuint(input: &mut Stream) -> ModalResult<u32> {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    loop {
        let byte = winnow::binary::u8.parse_next(input)?;
        result |= ((byte & 0b0111_1111) as u32) << shift;
        if byte < 0b1000_0000 {
            break;
        }
        shift += 7;
    }

    Ok(result)
}

#[derive(Debug, Clone)]
pub enum Constant {
    Owned(u32, u32),
    Shared(u32, u32),
}

#[derive(Debug, Clone, Default)]
pub struct State {
    nodes: Vec<Node>,
    constants: Vec<Constant>,
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
    winnow::combinator::alt((
        winnow::binary::u8.verify(|value| *value == 0).map(|_| None),
        parse_node.map(Some),
    ))
    .parse_next(input)
}

pub(super) fn parse_integer(input: &mut Stream) -> ModalResult<Integer> {
    todo.parse_next(input)
}

pub(super) fn parse_constant(input: &mut Stream) -> ModalResult<ConstantRef> {
    parse_varuint.map(ConstantRef).parse_next(input)
}

pub(super) fn parse_optional_constant(input: &mut Stream) -> ModalResult<Option<ConstantRef>> {
    winnow::combinator::alt((
        winnow::binary::u8.verify(|value| *value == 0).map(|_| None),
        parse_constant.map(Some),
    ))
    .parse_next(input)
}

pub(super) fn parse_bool(input: &mut Stream) -> ModalResult<bool> {
    winnow::binary::u8.map(|value| value == 1).parse_next(input)
}

pub(super) fn parse_comment(input: &mut Stream) -> ModalResult<Comment> {
    todo!()
}
pub(super) fn parse_double(input: &mut Stream) -> ModalResult<f64> {
    todo!()
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
    (
        u32(winnow::binary::Endianness::Native),
        u32(winnow::binary::Endianness::Native),
    )
        .map(|(offset, length)| Constant::Owned(offset, length))
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
        prism_major: winnow::binary::u8,
        prism_minor: winnow::binary::u8,
        prism_patch: winnow::binary::u8,
        only_semantics_serialized: parse_bool,
        encoding_name: parse_string,
        start_line: parse_varsint,
        newline_offsets: length_repeat(parse_varuint, parse_varuint),
        comments: length_repeat(parse_varuint, parse_comment),
        magic_comments: length_repeat(parse_varuint, todo),
        end_keyword: parse_optional_location,
        errors: length_repeat(parse_varuint, parse_error),
        warnings: length_repeat(parse_varuint, parse_error.map(|e| Warning {
            r#type: e.r#type,
            message: e.message,
            location: e.location,
            level: e.level,
        })),
        content_pool_offset: winnow::binary::u32(winnow::binary::Endianness::Native),
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
    program.constants =
        repeat(program.header.content_pool_size as usize, parse_content).parse_next(input)?;
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
      0 => empty.value(None),
      1 => parse_location.map(Some),
      _ => fail,
    )
    .parse_next(input)
}

#[cfg(test)]
fn input(bytes: &[u8]) -> Stream {
    use winnow::stream::StreamIsPartial;

    let input = Bytes::new(bytes);
    let state = State::default();
    let mut stream = winnow::Stateful {
        input: winnow::Partial::new(input),
        state,
    };
    let _ = stream.complete();
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
                root: NodeRef(
                    1,
                ),
                nodes: [
                    Node {
                        kind: 140,
                        identifier: 1,
                        location: Location {
                            start: 0,
                            length: 0,
                        },
                        flags: 0,
                        node_kind: StatementsNode(
                            StatementsNode {
                                body: [],
                            },
                        ),
                    },
                    Node {
                        kind: 121,
                        identifier: 2,
                        location: Location {
                            start: 0,
                            length: 0,
                        },
                        flags: 0,
                        node_kind: ProgramNode(
                            ProgramNode {
                                locals: [],
                                statements: NodeRef(
                                    0,
                                ),
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
                            location: Location {
                                start: 24,
                                length: 11,
                            },
                            level: 1,
                        },
                    ],
                    content_pool_offset: 268,
                    content_pool_size: 3,
                },
                root: NodeRef(
                    13,
                ),
                nodes: [
                    Node {
                        kind: 141,
                        identifier: 3,
                        location: Location {
                            start: 4,
                            length: 5,
                        },
                        flags: 0,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 4,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 5,
                                    length: 3,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 8,
                                        length: 1,
                                    },
                                ),
                                unescaped: "",
                            },
                        ),
                    },
                    Node {
                        kind: 98,
                        identifier: 4,
                        location: Location {
                            start: 0,
                            length: 9,
                        },
                        flags: 1,
                        node_kind: LocalVariableWriteNode(
                            LocalVariableWriteNode {
                                name: ConstantRef(
                                    1,
                                ),
                                depth: 0,
                                name_loc: Location {
                                    start: 0,
                                    length: 1,
                                },
                                value: NodeRef(
                                    0,
                                ),
                                operator_loc: Location {
                                    start: 2,
                                    length: 1,
                                },
                            },
                        ),
                    },
                    Node {
                        kind: 141,
                        identifier: 6,
                        location: Location {
                            start: 14,
                            length: 9,
                        },
                        flags: 0,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 14,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 15,
                                    length: 7,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 22,
                                        length: 1,
                                    },
                                ),
                                unescaped: "\nbar\t",
                            },
                        ),
                    },
                    Node {
                        kind: 98,
                        identifier: 7,
                        location: Location {
                            start: 10,
                            length: 13,
                        },
                        flags: 1,
                        node_kind: LocalVariableWriteNode(
                            LocalVariableWriteNode {
                                name: ConstantRef(
                                    1,
                                ),
                                depth: 0,
                                name_loc: Location {
                                    start: 10,
                                    length: 1,
                                },
                                value: NodeRef(
                                    2,
                                ),
                                operator_loc: Location {
                                    start: 12,
                                    length: 1,
                                },
                            },
                        ),
                    },
                    Node {
                        kind: 141,
                        identifier: 8,
                        location: Location {
                            start: 24,
                            length: 11,
                        },
                        flags: 5,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 24,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 25,
                                    length: 9,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 34,
                                        length: 1,
                                    },
                                ),
                                unescaped: "😀",
                            },
                        ),
                    },
                    Node {
                        kind: 141,
                        identifier: 10,
                        location: Location {
                            start: 44,
                            length: 22,
                        },
                        flags: 0,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 44,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 45,
                                    length: 20,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 65,
                                        length: 1,
                                    },
                                ),
                                unescaped: "",
                            },
                        ),
                    },
                    Node {
                        kind: 5,
                        identifier: 11,
                        location: Location {
                            start: 44,
                            length: 22,
                        },
                        flags: 0,
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                arguments: [
                                    NodeRef(
                                        5,
                                    ),
                                ],
                            },
                        ),
                    },
                    Node {
                        kind: 19,
                        identifier: 9,
                        location: Location {
                            start: 37,
                            length: 29,
                        },
                        flags: 33,
                        node_kind: CallNode(
                            CallNode {
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(
                                    2,
                                ),
                                message_loc: Some(
                                    Location {
                                        start: 37,
                                        length: 6,
                                    },
                                ),
                                opening_loc: None,
                                arguments: Some(
                                    NodeRef(
                                        6,
                                    ),
                                ),
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        kind: 141,
                        identifier: 13,
                        location: Location {
                            start: 72,
                            length: 7,
                        },
                        flags: 0,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 72,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 73,
                                    length: 5,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 78,
                                        length: 1,
                                    },
                                ),
                                unescaped: "",
                            },
                        ),
                    },
                    Node {
                        kind: 141,
                        identifier: 15,
                        location: Location {
                            start: 81,
                            length: 7,
                        },
                        flags: 0,
                        node_kind: StringNode(
                            StringNode {
                                opening_loc: Some(
                                    Location {
                                        start: 81,
                                        length: 1,
                                    },
                                ),
                                content_loc: Location {
                                    start: 82,
                                    length: 5,
                                },
                                closing_loc: Some(
                                    Location {
                                        start: 87,
                                        length: 1,
                                    },
                                ),
                                unescaped: "",
                            },
                        ),
                    },
                    Node {
                        kind: 5,
                        identifier: 14,
                        location: Location {
                            start: 72,
                            length: 16,
                        },
                        flags: 0,
                        node_kind: ArgumentsNode(
                            ArgumentsNode {
                                arguments: [
                                    NodeRef(
                                        8,
                                    ),
                                    NodeRef(
                                        9,
                                    ),
                                ],
                            },
                        ),
                    },
                    Node {
                        kind: 19,
                        identifier: 12,
                        location: Location {
                            start: 68,
                            length: 20,
                        },
                        flags: 33,
                        node_kind: CallNode(
                            CallNode {
                                receiver: None,
                                call_operator_loc: None,
                                name: ConstantRef(
                                    3,
                                ),
                                message_loc: Some(
                                    Location {
                                        start: 68,
                                        length: 3,
                                    },
                                ),
                                opening_loc: None,
                                arguments: Some(
                                    NodeRef(
                                        10,
                                    ),
                                ),
                                closing_loc: None,
                                block: None,
                            },
                        ),
                    },
                    Node {
                        kind: 140,
                        identifier: 1,
                        location: Location {
                            start: 0,
                            length: 88,
                        },
                        flags: 0,
                        node_kind: StatementsNode(
                            StatementsNode {
                                body: [
                                    NodeRef(
                                        1,
                                    ),
                                    NodeRef(
                                        3,
                                    ),
                                    NodeRef(
                                        4,
                                    ),
                                    NodeRef(
                                        7,
                                    ),
                                    NodeRef(
                                        11,
                                    ),
                                ],
                            },
                        ),
                    },
                    Node {
                        kind: 121,
                        identifier: 16,
                        location: Location {
                            start: 0,
                            length: 88,
                        },
                        flags: 0,
                        node_kind: ProgramNode(
                            ProgramNode {
                                locals: [
                                    ConstantRef(
                                        1,
                                    ),
                                ],
                                statements: NodeRef(
                                    12,
                                ),
                            },
                        ),
                    },
                ],
                constants: [
                    Owned(
                        0,
                        1,
                    ),
                    Owned(
                        37,
                        6,
                    ),
                    Owned(
                        68,
                        3,
                    ),
                ],
                content_pool: [],
            },
        )
    "#]]
    .assert_debug_eq(&program);
}
