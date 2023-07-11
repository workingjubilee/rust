#![feature(type_alias_impl_trait)]

type Foo = impl PartialEq<(Foo, i32)>;

struct Bar;

impl PartialEq<(Foo, i32)> for Bar {
//~^ ERROR cannot implement trait on type alias impl trait
    fn eq(&self, _other: &(Foo, i32)) -> bool {
        true
    }
}

fn foo() -> Foo {
    Bar
}

fn main() {}

// in rust-lang/rust#98250 we found out that rustdoc can stack overflow on code rustc does not! fun!
