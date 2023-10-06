/// Fake jstraceable
pub trait JSTraceable {}
impl JSTraceable for i32 {}

struct TraceableStruct;
impl JSTraceable for TraceableStruct {}

struct NotTraceableStruct;

// second generic argument must not be traceable
#[crown::trace_in_no_trace_lint::must_not_have_traceable(0)]
struct NoTrace<NoTraceable> {
    n: NoTraceable,
}

struct Foo(NoTrace<i32>);

fn main() {}
