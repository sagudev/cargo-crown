#[crown::unrooted_must_root_lint::must_root]
struct Foo(i32);

fn foo2() -> Foo {
    unimplemented!()
}

fn main() {}
