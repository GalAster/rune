#[test]
fn test_grouped_imports() {
    assert_eq! {
        rune! { (i64, bool, bool) =>
            use a::{b::*, b::Foo::Baz, c};

            pub mod a {
                pub mod b {
                    pub enum Foo { Bar, Baz, }
                }

                pub mod c {
                    pub const VALUE = 2;
                }
            }

            pub fn main() {
                (c::VALUE, Foo::Bar is a::b::Foo, Baz is a::b::Foo)
            }
        },
        (2, true, true),
    };
}

#[test]
fn test_reexport() {
    assert_eq! {
        rune! { i64 =>
            mod inner { fn func() { 42 } }
            pub use self::inner::func as main;
        },
        42,
    };

    assert_eq! {
        rune! { i64 =>
            mod inner { fn func() { 42 } }
            pub use crate::inner::func as main;
        },
        42,
    };

    assert_eq! {
        rune! { i64 =>
            mod inner2 { fn func() { 42 } }
            mod inner1 { pub use super::inner2::func; }
            pub use crate::inner1::func as main;
        },
        42,
    };
}