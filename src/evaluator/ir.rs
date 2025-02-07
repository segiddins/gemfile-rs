use std::{fmt::Debug, ops::Range, path::PathBuf};

use derive_new::new;

#[salsa::input]
pub struct SourceGemfile {
    #[return_ref]
    pub source: String,

    #[return_ref]
    pub path: PathBuf,
}

#[salsa::input]
pub struct EnvVariable {
    #[return_ref]
    name: String,

    #[return_ref]
    value: Option<String>,
}

#[salsa::tracked]
pub struct Gemfile<'db> {
    #[return_ref]
    pub ruby_version: Option<RubyVersion>,

    #[return_ref]
    pub root_context: Block,
}

#[derive(Clone, Debug, PartialEq, Eq, new)]
pub struct Block {
    pub children: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RubyVersion {
    String(String),
    File(String),
}

#[derive(Clone, Debug, PartialEq, Eq, new)]
pub struct MethodCall {
    receiver: Option<Box<Node>>,
    method_name: String,
    args: Vec<Node>,
    pub loc: super::eval::Location,
}

#[derive(Clone, Debug, PartialEq, Eq, new)]
pub struct StringLiteral {
    pub value: String,
    pub loc: super::eval::Location,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
    Error(ErrorNode),
    Block(Block),
    MethodCall(MethodCall),
    StringLiteral(StringLiteral),
}

#[derive(Clone, Debug, PartialEq, Eq, new)]
pub struct ErrorNode {
    pub message: String,
    pub loc: super::eval::Location,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Error(e) => e.fmt(f),
            Node::Block(b) => b.fmt(f),
            Node::MethodCall(m) => m.fmt(f),
            Node::StringLiteral(s) => s.fmt(f),
        }
    }
}

#[salsa::input]
pub struct SalsaInput {
    #[return_ref]
    pub source: String,

    #[return_ref]
    pub path: String,
}

#[salsa::tracked]
pub struct SalsaOutput<'db> {
    pub input: SalsaInput,
    pub substring_range: Range<usize>,
}

impl SalsaOutput<'_> {
    pub fn substring<'a>(&self, db: &'a dyn super::db::Db) -> &'a str {
        &self.input(db).source(db)[self.substring_range(db)]
    }
}

#[salsa::tracked]
pub fn best_substring<'db>(db: &'db dyn super::db::Db, input: SalsaInput) -> SalsaOutput<'db> {
    SalsaOutput::new(db, input, 0..1)
}
