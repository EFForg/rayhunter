// Unpacks a pattern, or returns None.
//
// # Examples
// You can use `unpack!` to unroll highly nested enums like this:
// ```
// enum Foo {
//     A(Bar),
//     B,
// }
//
// enum Bar {
//     C(Baz)
// }
//
// struct Baz;
//
// fn get_bang(foo: Foo) -> Option<Baz> {
//     unpack!(Foo::A(bar) = foo);
//     unpack!(Bar::C(baz) = bar);
//     baz
// }
// ```
//
macro_rules! unpack {
    ($pat:pat = $val:expr) => {
        let $pat = $val else {
            return None;
        };
    };
}

// this is apparently how you make a macro publicly usable from this module
pub(crate) use unpack;
