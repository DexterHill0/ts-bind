use derive_more::From;

use crate::generics::Generics;

/// TODO
#[derive(Default)]
pub struct Bindings {
    files: Vec<String>,

    types: Vec<TSType>,
}

impl Bindings {
    pub fn add_binding(&mut self, t: impl Into<TSType>) {
        self.types.push(t.into());
    }
}

/// TODO
#[derive(From)]
pub enum TSType {
    Type(Type),
    Interface(Interface),
    AbstractClass(AbstractClass),
}

/// TODO
#[derive(Default)]
pub struct Type {
    pub name: String,
    pub generics: Generics,
}

/// TODO
#[derive(Default)]
pub struct Interface {
    pub name: String,
    pub generics: Generics,
}

/// TODO
#[derive(Default)]
pub struct AbstractClass {
    pub name: String,
    pub generics: Generics,
}

impl AbstractClass {
    pub fn new(name: impl Into<String>) -> Self {
        AbstractClass {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn add_generic(&mut self, name: String) -> &mut Generics {
        self.generics.add_generic(name)
    }
}
