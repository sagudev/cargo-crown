#[crown::unrooted_must_root_lint::must_root]
struct Foo(i32);
struct Bar(Foo);

fn main() {}
