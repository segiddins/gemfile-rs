use std::rc::Rc;

use ruby_prism;
use ruby_prism::Visit;

mod evaluator;
mod prism_ast;

#[derive(Default, Debug)]
pub struct Gemfile {
    statements: Vec<Statement>,
}

enum GemfileNode {
    Program(Vec<GemfileNode>),
    Call(CallNode),
    String(String),
    Env(String),
    Nil,
}

struct CallNode {
    receiver: Option<Box<GemfileNode>>,
    method: String,
    arguments: Vec<Box<GemfileNode>>,
    keyword_arguments: Vec<(String, Box<GemfileNode>)>,
}

struct Visitor {
    gemfile: Gemfile,
    node: Rc<GemfileNode>,
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            gemfile: Gemfile::default(),
            node: Rc::new(GemfileNode::Program(Vec::new())),
        }
    }
}

impl TryFrom<ruby_prism::Node<'_>> for GemfileNode {
    type Error = ();

    fn try_from(node: ruby_prism::Node<'_>) -> Result<Self, Self::Error> {
        Ok(GemfileNode::Program(Vec::new()))
    }
}

impl<'pr> ruby_prism::Visit<'pr> for Visitor {
    fn visit_branch_node_enter(&mut self, node: ruby_prism::Node<'_>) {
        let new_node = Rc::new(GemfileNode::try_from(node).unwrap());
    }

    fn visit_leaf_node_enter(&mut self, node: ruby_prism::Node<'_>) {
        println!("{:?}", node);
    }
}

impl Gemfile {
    pub fn parse(input: &[u8]) -> Self {
        let parse_result = ruby_prism::parse(input);
        let root = parse_result.node();

        let mut visitor = Visitor::new();
        visitor.visit(&root);

        visitor.gemfile
    }
}

#[derive(Debug)]
pub enum Statement {}

fn main() {
    println!("Hello, world!");
}
