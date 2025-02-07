pub mod db;
pub mod eval;
pub mod ir;
pub mod parse;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use expect_test::expect;

    use super::*;

    #[test]
    fn test_eval_empty() {
        eval(
            "",
            expect![[r#"
            (
                Some(
                    Definition {
                        [salsa id]: Id(800),
                        dependencies: [],
                    },
                ),
                [],
            )
        "#]],
        )
    }

    #[cfg(test)]
    fn eval(source: &str, expect: expect_test::Expect) {
        use salsa::Database;

        db::DbImpl::default().attach(|db| {
            let source = ir::SourceGemfile::new(
                db,
                source.to_string(),
                PathBuf::from_str("Gemfile").unwrap(),
            );
            let definition = eval::evaluate(db, source);

            let diag = eval::evaluate::accumulated::<eval::Diagnostic>(db, source);

            expect.assert_debug_eq(&(definition, diag));
        });
    }

    #[test]
    fn test_eval_simple() {
        eval(
            r#"
            source 'https://rubygems.org'
            gem 'rails', '4.2.5'
            "#,
            expect![[r#"
                (
                    Some(
                        Definition {
                            [salsa id]: Id(800),
                            dependencies: [],
                        },
                    ),
                    [
                        Diagnostic {
                            level: Warning,
                            message: "Method call found",
                            location: Some(
                                Location {
                                    start: 13,
                                    end: 42,
                                },
                            ),
                        },
                        Diagnostic {
                            level: Warning,
                            message: "Method call found",
                            location: Some(
                                Location {
                                    start: 55,
                                    end: 75,
                                },
                            ),
                        },
                    ],
                )
            "#]],
        )
    }
}
