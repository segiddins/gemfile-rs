use std::fmt;

pub mod deserialize;
mod generated;

struct Program {
    source: String,

    expressions: Vec<Expression<ExpressionRef, LocationRef>>,

    toplevel_statements: Vec<ExpressionRef>,
}

impl Program {
    fn expression(&self, r#ref: ExpressionRef) -> &Expression<ExpressionRef, LocationRef> {
        &self.expressions[r#ref.0 as usize]
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.toplevel_statements.iter().map(|r| TreeDebug {
                program: self,
                expression: *r,
            }))
            .finish()
    }
}

struct TreeDebug<'a> {
    program: &'a Program,
    expression: ExpressionRef,
}

impl fmt::Debug for TreeDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = self.program.expression(self.expression);

        match &expr.kind {
            ExpressionKind::Integer(value) => write!(f, "Integer({})", value),
            ExpressionKind::String(value) => write!(f, "String({:?})", value),
            ExpressionKind::Call(call) => f
                .debug_struct("CallStatement")
                .field(
                    "receiver",
                    &call.receiver.map(|r| TreeDebug {
                        program: self.program,
                        expression: r,
                    }),
                )
                .field("name", &call.name)
                .field(
                    "args",
                    &call
                        .args
                        .iter()
                        .map(|r| TreeDebug {
                            program: self.program,
                            expression: *r,
                        })
                        .collect::<Vec<_>>(),
                )
                .finish(),
            ExpressionKind::Nil => write!(f, "Nil"),
            ExpressionKind::Error(err) => write!(f, "Error({:#?})", err),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ExpressionRef(u32);

#[derive(Debug, Clone, Copy)]
struct LocationRef {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct Expression<Expr, Loc> {
    kind: ExpressionKind<Expr>,
    loc: Loc,
}

#[derive(Debug)]
enum ExpressionKind<T> {
    Integer(i32),
    String(String),
    Call(CallStatement<T>),
    Nil,
    Error(anyhow::Error),
}

#[derive(Debug)]
struct CallStatement<T> {
    name: String,
    receiver: Option<T>,
    args: Vec<T>,
}

fn map_expression_node(
    node: ruby_prism::Node,
    expressions: &mut Vec<Expression<ExpressionRef, LocationRef>>,
) -> ExpressionRef {
    let kind = match node {
        ruby_prism::Node::IntegerNode { .. } => {
            let node = node.as_integer_node().unwrap();
            let value = node.value().try_into();
            match value {
                Ok(value) => ExpressionKind::Integer(value),
                Err(_) => ExpressionKind::Error(anyhow::anyhow!("integer value out of range")),
            }
        }
        ruby_prism::Node::StringNode { .. } => {
            let node = node.as_string_node().unwrap();
            ExpressionKind::String(String::from_utf8_lossy(node.unescaped()).to_string())
        }
        ruby_prism::Node::NilNode { .. } => ExpressionKind::Nil,
        ruby_prism::Node::CallNode { .. } => {
            let node = node.as_call_node().unwrap();
            ExpressionKind::Call(CallStatement {
                name: String::from_utf8_lossy(node.name().as_slice()).to_string(),
                receiver: node.receiver().map(|r| map_expression_node(r, expressions)),
                args: node
                    .arguments()
                    .map(|args| {
                        args.arguments()
                            .iter()
                            .map(|arg| map_expression_node(arg, expressions))
                            .collect()
                    })
                    .unwrap_or_default(),
            })
        }
        _ => ExpressionKind::Error(anyhow::anyhow!("unsupported node type: {:?}", node)),
    };

    let expr_ref = ExpressionRef(expressions.len() as u32);
    let prism_loc = node.location();
    expressions.push(Expression {
        kind,
        loc: LocationRef {
            start: prism_loc.start_offset() as u32,
            end: prism_loc.end_offset() as u32,
        },
    });

    expr_ref
}

fn parse(input: String) -> Program {
    let parse_result = ruby_prism::parse(input.as_bytes());
    // TODO: handle warnings + errors

    let mut expressions = Vec::new();

    let program_node = parse_result.node().as_program_node().unwrap();
    let toplevel_statements = program_node
        .statements()
        .body()
        .iter()
        .map(|statement| map_expression_node(statement, &mut expressions))
        .collect();

    drop(parse_result);

    Program {
        source: input,
        expressions,
        toplevel_statements,
    }
}

#[test]
fn test_ruby_prism_parse_serialize() {
    let input = r#""#;
    let parse_result = ruby_prism::parse(input.as_bytes());
    let serialized = parse_result.serialize();

    expect_test::expect![[r#"
        [
            80,
            82,
            73,
            83,
            77,
            1,
            3,
            0,
            0,
            5,
            85,
            84,
            70,
            45,
            56,
            2,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            40,
            0,
            0,
            0,
            0,
            121,
            2,
            0,
            0,
            0,
            0,
            140,
            1,
            0,
            0,
            0,
            0,
            0,
        ]
    "#]]
    .assert_debug_eq(&serialized);
}

#[test]
fn test_parse() {
    let program = parse("1 + 2\n'foo'\nnil\n1+1+1".to_string());

    expect_test::expect![[r#"
        [
            CallStatement {
                receiver: Some(
                    Integer(1),
                ),
                name: "+",
                args: [
                    Integer(2),
                ],
            },
            String("foo"),
            Nil,
            CallStatement {
                receiver: Some(
                    CallStatement {
                        receiver: Some(
                            Integer(1),
                        ),
                        name: "+",
                        args: [
                            Integer(1),
                        ],
                    },
                ),
                name: "+",
                args: [
                    Integer(1),
                ],
            },
        ]
    "#]]
    .assert_debug_eq(&program);
}

#[test]
fn test_parse_2() {
    let program = parse(
        r#"
source "https://rubygems.org"

version = "1.0.0"
other = ENV["GEM_VERSION"]

gem "prism-ruby", "~> 0.1.0"
gemspec path: "prism-ruby.gemspec"

if true
        gem "bundler", other || version
end

1 unless !!false
    "#
        .to_string(),
    );

    expect_test::expect![[r#"
        [
            CallStatement {
                receiver: None,
                name: "source",
                args: [
                    String("https://rubygems.org"),
                ],
            },
            Error("unsupported node type: LocalVariableWriteNode(2, 0, \"version\", StringNode(Some(\"\\\"\"), \"1.0.0\", Some(\"\\\"\"), [49, 46, 48, 46, 48]), \"=\")"),
            Error("unsupported node type: LocalVariableWriteNode(3, 0, \"other\", CallNode(Some(ConstantReadNode(4)), None, 5, Some(\"[\\\"GEM_VERSION\\\"]\"), Some(\"[\"), Some(ArgumentsNode([StringNode(Some(\"\\\"\"), \"GEM_VERSION\", Some(\"\\\"\"), [71, 69, 77, 95, 86, 69, 82, 83, 73, 79, 78])])), Some(\"]\"), None), \"=\")"),
            CallStatement {
                receiver: None,
                name: "gem",
                args: [
                    String("prism-ruby"),
                    String("~> 0.1.0"),
                ],
            },
            CallStatement {
                receiver: None,
                name: "gemspec",
                args: [
                    Error("unsupported node type: KeywordHashNode([AssocNode(SymbolNode(None, Some(\"path\"), Some(\":\"), [112, 97, 116, 104]), StringNode(Some(\"\\\"\"), \"prism-ruby.gemspec\", Some(\"\\\"\"), [112, 114, 105, 115, 109, 45, 114, 117, 98, 121, 46, 103, 101, 109, 115, 112, 101, 99]), None)])"),
                ],
            },
            Error("unsupported node type: IfNode(Some(\"if\"), TrueNode(), None, Some(StatementsNode([CallNode(None, None, 6, Some(\"gem\"), None, Some(ArgumentsNode([StringNode(Some(\"\\\"\"), \"bundler\", Some(\"\\\"\"), [98, 117, 110, 100, 108, 101, 114]), OrNode(LocalVariableReadNode(3, 0), LocalVariableReadNode(2, 0), \"||\")])), None, None)])), None, Some(\"end\"))"),
            Error("unsupported node type: UnlessNode(\"unless\", CallNode(Some(CallNode(Some(FalseNode()), None, 8, Some(\"!\"), None, None, None, None)), None, 8, Some(\"!\"), None, None, None, None), None, Some(StatementsNode([IntegerNode(0x153904748)])), None, None)"),
        ]
    "#]];
    //.assert_debug_eq(&program);
}
