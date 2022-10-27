use std::{any::Any, collections::HashMap};

/// Any traits that are exported are implemented on this type.
/// Then, if/when the trait is referenced in a trait bound, we can see if it has been exported as an abstract class or not,
/// allowing us to know whether to generate an empty abstract class or not.
#[doc(hidden)] // private API
pub struct __IsGeneric;

#[macro_export]
macro_rules! __is_trait_exported {
    ($trait_name:path) => {{
        trait __InnerMarkerTrait {
            fn __is_trait_inner_method() -> bool {
                false
            }
        }
        struct __TraitTest<T>(T);
        impl<T: $trait_name> __TraitTest<T> {
            fn __is_trait_inner_method() -> bool {
                true
            }
        }
        impl<T> __InnerMarkerTrait for __TraitTest<T> {}
        __TraitTest::<ts_bind::__IsGeneric>::__is_trait_inner_method()
    }};
}

pub struct Bound {
    trt: Box<dyn Any>,
    generic_types: Vec<String>,
    generic_structs: Vec<Box<dyn Any>>,
}

impl Bound {
    pub fn new(trt: impl Any) -> Self {
        Bound {
            trt: Box::new(trt),
            generic_types: vec![],
            generic_structs: vec![],
        }
    }

    pub fn add_generic(&mut self, name: impl Into<String>) -> &mut Self {
        self.generic_types.push(name.into());

        self
    }

    pub fn add_generic_type(&mut self, typ: impl Any) -> &mut Self {
        self.generic_structs.push(Box::new(typ));

        self
    }
}

#[derive(Default)]
pub struct Generics {
    names: Vec<String>,
    extends: HashMap<String, Vec<Bound>>,
    default: Option<Box<dyn Any>>,

    curr_generic: usize,
}

impl Generics {
    pub fn add_generic(&mut self, name: impl Into<String>) -> &mut Generics {
        self.names.push(name.into());

        self.curr_generic = self.names.len() - 1;

        self
    }

    pub fn add_bound(&mut self, bound: Bound) {
        self.extends
            .entry(self.names[self.curr_generic].clone())
            .and_modify(|v| v.push(bound));
    }

    pub fn add_default(&mut self, def: impl Any) {
        self.default = Some(Box::new(def));
    }
}
