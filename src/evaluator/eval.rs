use std::fmt::Debug;

use super::{
    db::Db,
    ir::{Block, SourceGemfile},
};
use derive_new::new;
use salsa::Accumulator;

#[salsa::accumulator]
#[derive(new)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<Location>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl From<&ruby_prism::Location<'_>> for Location {
    fn from(location: &ruby_prism::Location<'_>) -> Self {
        Self {
            start: location.start_offset(),
            end: location.end_offset(),
        }
    }
}

impl Diagnostic {
    #[cfg(test)]
    pub fn render(&self, db: &dyn Db, src: SourceGemfile) -> String {
        use annotate_snippets::*;

        let level = match self.level {
            DiagnosticLevel::Error => Level::Error,
            DiagnosticLevel::Warning => Level::Warning,
        };
        let mut message = level.title(&self.message);
        if let Some(location) = &self.location {
            let source = src.source(db);
            let line_start = source[..location.start].lines().count() + 1;
            message = message.snippet(
                Snippet::source(source)
                    .line_start(line_start)
                    .origin(src.path(db).to_str().unwrap())
                    .fold(true)
                    .annotation(level.span(location.start..location.end).label("here")),
            );
        }
        Renderer::plain().render(message).to_string()
    }
}

#[salsa::tracked]
pub struct Definition<'db> {
    pub dependencies: Vec<Dependency>,
}

#[salsa::tracked]
pub fn evaluate(db: &dyn Db, gemfile: SourceGemfile) -> Option<Definition<'_>> {
    let parsed = super::parse::parse_source_gemfile(db, gemfile)?;
    let dependencies = eval_block(db, parsed.root_context(db));
    Some(Definition::new(db, dependencies))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Dependency;

fn eval_block(db: &dyn Db, block: &Block) -> Vec<Dependency> {
    block
        .children
        .iter()
        .flat_map(|node| match node {
            super::ir::Node::Block(block) => eval_block(db, block),
            super::ir::Node::StringLiteral(lit) => {
                Diagnostic::new(
                    DiagnosticLevel::Warning,
                    "String literal found".to_string(),
                    Some(lit.loc.clone()),
                )
                .accumulate(db);
                vec![]
            }
            super::ir::Node::MethodCall(call) => {
                Diagnostic::new(
                    DiagnosticLevel::Warning,
                    "Method call found".to_string(),
                    Some(call.loc.clone()),
                )
                .accumulate(db);
                vec![]
            }
            _ => vec![],
        })
        .collect()
}
