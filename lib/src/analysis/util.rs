
/// Unpacks a pattern, or returns None.
///
/// # Examples
/// Suppose you've got some highly nested enum:
/// ```
/// enum Foo {
///     A(Bar),
///     B,
/// }
///
/// enum Baz {
///     C(Bang)
/// }
///
/// struct Bang;
/// ```
///
/// You can use `unpack!` to unroll it like this:
/// ```
/// fn get_bang(foo: Foo) -> Option<Bang> {
///     unpack!(Foo::A(bar) = foo);
///     unpack!(Baz::C(bang) = bar);
///     bang
/// }
/// ```
macro_rules! unpack {
    ($pat:pat = $val:expr) => {
        let $pat = $val else { return None; };
    };
}

// this is apparently how you make a macro publicly usable from this module
pub(crate) use unpack;
