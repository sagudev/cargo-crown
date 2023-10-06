/// Fake jstraceable
pub trait JSTraceable {}
impl JSTraceable for i32 {}

#[crown::trace_in_no_trace_lint::must_not_have_traceable]
struct NoTrace<T>(T);

struct Bar;
impl JSTraceable for Bar {}

struct Foo(NoTrace<Bar>);

fn main() {}
