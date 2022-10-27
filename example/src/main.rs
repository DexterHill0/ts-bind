use ts_bind::{ts, TS};

#[ts]
trait Trait<T, U> {}

struct Foo {
    //x: Y,
}

// impl TS for Foo {
//     const EXPORT_TO: Option<&'static str> = Some("bindings/Foo.ts");

//     fn name() -> String {
//         "Foo".into()
//     }

//     fn decl(bindings: &mut Bindings) -> TSType {
//         Interface::new(Self::name()).into() // ...
//     }

//     fn dependencies() -> Vec<Dependency> {
//         vec![Y]
//     }
// }

// impl<T> Trait<Vec<T>> for Foo {}

// #[ts]
trait Example<T>
where
    T: self::Trait<T, Vec<T>>,
{
    // const CONST_NO_DEFAULT: i32;
    // const CONST_WITH_DEFAULT: i32 = 99;

    // type TypeNoDefault;

    // fn method_without_default(&self);
    // fn method_with_default(&self) {}

    // fn static_method();
    // fn self_method(&self, x: T) -> T;
}

// abstract class Example<T extentds Trait<T, Array<T>> = Foo> { ... }

fn main() {}

// trait unsupported features
// - anything items that arent const / fn
// trait downsides:
// - all fns except non-default instance methods have default implementations in typescript (due to abstract classes)
// - any errors will say `_ts_<trait name>`
//
// generics not implemented features
// - #[ignore_bound] #[rename] (for specific bounds and defaults, not just all of them)
// generics unsupported features
// - dyn Bound
// - fully qualified syntax
//

// Added
// - no 'static anymore
// - traits
// - generic trait bounds
