fn handler0() {
    println!("handler[0]");
}

fn handler1(a: usize) {
    println!("handler[1]: {}", a);
}

fn handler1s(a: &str) {
    println!("handler[1s]: {}", a);
}

fn handler2(a: usize, b: &str) {
    println!("handler[2]: {}, {}", a, b);
}

trait Handler<T, R> {
    fn call(&self, param: T) -> R;
}

impl<F, R> Handler<(), R> for F
where
    F: Fn() -> R
{
    fn call(&self, _param: ()) -> R {
        (self)()
    }
}

impl<F, R, A> Handler<(A,), R> for F
where
    F: Fn(A) -> R
{
    fn call(&self, param: (A,)) -> R {
        (self)(param.0)
    }
}

impl<F, R, A, B> Handler<(A, B), R> for F
where
    F: Fn(A, B) -> R
{
    fn call(&self, param: (A, B)) -> R {
        (self)(param.0, param.1)
    }
}

fn main() {
    let f0 = handler0;
    f0();
    f0.call(());

    let f1 = handler1;
    f1(42);
    f1.call((42,));

    let f1s = handler1s;
    f1s("x");
    f1s.call(("x",));

    let f2 = handler2;
    f2(42, "x");
    f2.call((42, "x"));
}
