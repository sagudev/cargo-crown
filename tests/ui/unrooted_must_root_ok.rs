#[crown::unrooted_must_root_lint::must_root]
struct Foo(i32);
#[crown::unrooted_must_root_lint::must_root]
struct Bar(Foo);

fn foo1(_: &Foo) {}
fn foo2(_: &()) -> &Foo {
    unimplemented!()
}

fn main() {}
