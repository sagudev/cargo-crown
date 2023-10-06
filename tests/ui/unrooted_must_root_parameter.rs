#[crown::unrooted_must_root_lint::must_root]
struct Foo(i32);

fn foo1(_: Foo) {}

fn main() {}
