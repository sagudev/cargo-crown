/// Fake jstraceable
pub trait JSTraceable {}
impl JSTraceable for i32 {}

// second generic argument must not be traceable
#[crown::trace_in_no_trace_lint::must_not_have_traceable(1)]
struct NoTraceComposable<Traceable, NoTraceable> {
    t: Traceable,
    n: NoTraceable,
}

// this is not ok i32 is traceable
struct Foo(NoTraceComposable<u32, i32>);

fn main() {}