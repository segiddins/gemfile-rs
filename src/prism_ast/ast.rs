use super::{deserialize::*, generated::NodeKind};

macro_rules! node {
    ( @ $(#[$attr:meta])* $name:ident { } -> ($($result:tt)*) ) => (
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
          $($result)*
        }
    );

    ( @ $(#[$attr:meta])* $name:ident { $(#[$field_attr:meta])* $param:ident : $type:ty, $($rest:tt)* } -> ($($result:tt)*) ) => (
      node!(@ $(#[$attr])* $name { $($rest)* } -> (
          $($result)*
          $(#[$field_attr])*
          pub $param : $type,
      ));
    );
    ($(#[$attr:meta])* $name:ident, $($tail:tt)* ) => {
        node!(@ $(#[$attr])* $name { $($tail)* } -> ());
    };
}

macro_rules! NodeKind {
    (case $(#[$attr:meta])+ $name:ident) => {
        node! {
            #[doc = "A node representing a program"]
            ProgramNode,
            #[doc = "A node representing a statement"]
            locals: Vec<ConstantRef>,
            statements: NodeRef,
        }
    };
    ($(#[$attr:meta])+, $(#[$case_attr:meta])+ $name:ident, $($tail:tt)*) => {
      enum NodeKind {
        $name($name),
        $($tail)*
      }
    };
}

macro_rules! nodes {
    () => {};
    (cases () -> ($($result:tt)*) -> () ) => {
      #[doc = "A node representing a program"]
      pub enum NodeKind {
        $($result)*
      }
    };
    (nodes () -> ($($nodes:tt)*) -> $($cases:tt)* ) => {
      pub enum NodeKind {
        $($result)*
      }
    };
    ( { $(#[$attr:meta])* $name:ident, $($fields:tt)* }, $($tail:tt)* ) => {
      node!($(#[$attr])* $name, $($fields)*);
      nodes!($($tail)*);
    };
}

nodes! {
  {
    #[doc = "A node representing a program"]
    ProgramNode,
    #[doc = "A node representing a statement"]
    locals: Vec<ConstantRef>,
    statements: NodeRef,
  },
  {
    #[doc = "A node representing a program"]
    ProgramNode2,
    #[doc = "A node representing a statement"]
    locals: Vec<ConstantRef>,
    statements: NodeRef,
  },
}
