#[cfg(test)]
use salsa::Database as _;
#[cfg(test)]
use std::str::FromStr as _;

use salsa::Accumulator;

use crate::evaluator::{eval, ir::Gemfile};

use super::{db, ir};

#[test]
fn mything() {
    db::DbImpl::default().attach(|db| {
        let source = ir::SourceGemfile::new(
            db,
            "gem 'foo'".to_string(),
            std::path::PathBuf::from_str("Gemfile").unwrap(),
        );

        let text = source.source(db);
        let parse_result = ruby_prism::parse(text.as_bytes());
    })
}

#[salsa::tracked]
pub fn parse_source_gemfile(db: &dyn db::Db, source: ir::SourceGemfile) -> Option<ir::Gemfile<'_>> {
    let parse_result = ruby_prism::parse(source.source(db).as_bytes());
    for e in parse_result.errors() {
        eval::Diagnostic::new(
            eval::DiagnosticLevel::Error,
            e.message().to_string(),
            Some((&e.location()).into()),
        )
        .accumulate(db);
    }
    for e in parse_result.warnings() {
        eval::Diagnostic::new(
            eval::DiagnosticLevel::Warning,
            e.message().to_string(),
            Some((&e.location()).into()),
        )
        .accumulate(db);
    }

    let root = to_ir(db, parse_result.node());
    match root {
        ir::Node::Block(g) => Some(Gemfile::new(db, None, g)),
        ir::Node::Error(_) => None,
        _ => {
            eval::Diagnostic::new(
                eval::DiagnosticLevel::Error,
                format!("expected Gemfile node, got {:#?}", root),
                None,
            )
            .accumulate(db);
            None
        }
    }
}

fn to_ir(db: &dyn db::Db, node: ruby_prism::Node) -> ir::Node {
    match node {
        ruby_prism::Node::ProgramNode { .. } => {
            let node = node.as_program_node().unwrap();
            let children: Vec<_> = node
                .statements()
                .body()
                .iter()
                .map(|n| to_ir(db, n))
                .collect();
            ir::Node::Block(ir::Block::new(children))
        }
        ruby_prism::Node::CallNode { .. } => {
            let node = node.as_call_node().unwrap();
            let receiver = node.receiver().map(|r| to_ir(db, r));
            let method = node.name().as_slice();
            let method_name = String::from_utf8_lossy(method);
            let args = node.arguments().map_or(vec![], |a| {
                a.arguments()
                    .iter()
                    .map(|a| to_ir(db, a))
                    .collect::<Vec<_>>()
            });

            ir::Node::MethodCall(ir::MethodCall::new(
                receiver.map(Box::new),
                method_name.to_string(),
                args,
                (&node.location()).into(),
            ))
        }
        ruby_prism::Node::StringNode { .. } => {
            let node = node.as_string_node().unwrap();
            ir::Node::StringLiteral(ir::StringLiteral::new(
                String::from_utf8_lossy(node.unescaped()).to_string(),
                (&node.location()).into(),
            ))
        }
        _ => {
            eval::Diagnostic::new(
                eval::DiagnosticLevel::Error,
                format!("unimplemented: {:#?}", node),
                Some((&node.location()).into()),
            )
            .accumulate(db);
            ir::Node::Error(ir::ErrorNode::new(
                "unsupported node".to_string(),
                (&node.location()).into(),
            ))
        }
    }
}

#[cfg(test)]
fn parse_string(string: &str, expect: expect_test::Expect) {
    use std::{
        fmt::{self, Debug},
        str::FromStr,
    };

    use salsa::Database;

    db::DbImpl::default().attach(|db| {
        let source = ir::SourceGemfile::new(
            db,
            string.to_string(),
            std::path::PathBuf::from_str("Gemfile").unwrap(),
        );
        let gemfile = parse_source_gemfile(db, source);

        struct DebugString(String);
        impl Debug for DebugString {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.0.as_str())
            }
        }

        let diagnostics = eval::evaluate::accumulated::<eval::Diagnostic>(db, source)
            .into_iter()
            .map(|d| DebugString(d.render(db, source)))
            .collect::<Vec<_>>();

        expect.assert_debug_eq(&(gemfile, diagnostics));
    });
}

#[test]
fn test_parse() {
    parse_string(
        "",
        expect_test::expect![[r#"
            (
                Some(
                    Gemfile {
                        [salsa id]: Id(400),
                        ruby_version: None,
                        root_context: Block {
                            children: [],
                        },
                    },
                ),
                [],
            )
        "#]],
    );
    parse_string(
        "'string'",
        expect_test::expect![[r#"
            (
                Some(
                    Gemfile {
                        [salsa id]: Id(400),
                        ruby_version: None,
                        root_context: Block {
                            children: [
                                StringLiteral {
                                    value: "string",
                                    loc: Location {
                                        start: 0,
                                        end: 8,
                                    },
                                },
                            ],
                        },
                    },
                ),
                [
                    warning: String literal found
                     --> Gemfile:1:1
                      |
                    1 | 'string'
                      | -------- here
                      |,
                    warning: possibly useless use of a literal in void context
                     --> Gemfile:1:1
                      |
                    1 | 'string'
                      | -------- here
                      |,
                ],
            )
        "#]],
    );
    parse_string(
        "gem 'foo'",
        expect_test::expect![[r#"
            (
                Some(
                    Gemfile {
                        [salsa id]: Id(400),
                        ruby_version: None,
                        root_context: Block {
                            children: [
                                MethodCall {
                                    receiver: None,
                                    method_name: "gem",
                                    args: [
                                        StringLiteral {
                                            value: "foo",
                                            loc: Location {
                                                start: 4,
                                                end: 9,
                                            },
                                        },
                                    ],
                                    loc: Location {
                                        start: 0,
                                        end: 9,
                                    },
                                },
                            ],
                        },
                    },
                ),
                [
                    warning: Method call found
                     --> Gemfile:1:1
                      |
                    1 | gem 'foo'
                      | --------- here
                      |,
                ],
            )
        "#]],
    );
}
