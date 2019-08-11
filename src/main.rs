use std::fmt;
use std::marker::PhantomData;

mod extensions;

use crate::extensions::Extensions;

fn handler0() {
    println!("handler[0]");
}

fn handler1(a: Envelope<String>) {
    println!("handler[1]: {}", a);
}

fn handler1i(a: Envelope<i32>) {
    println!("handler[1u]: {}", a);
}

fn handler2(a: Envelope<String>, b: Envelope<i32>) {
    println!("handler[2]: {}, {}", a, b);
}

fn handler2s(a: Envelope<i32>, b: Envelope<String>) {
    println!("handler[2s]: {}, {}", a, b);
}

trait Handler<T, R>: Sized {
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

struct Dispatcher<T, R, F = Handler<T, R>> {
    handler: F,
    _t: PhantomData<(T, R)>,
}

impl<T, R, F> Dispatcher<T, R, F>
where
    F: Handler<T, R>,
    T: PickUp<Item = T>,
{
    fn new(handler: F) -> Self {
        Dispatcher {
            handler,
            _t: PhantomData,
        }
    }

    fn run(&self, bag: &Extensions) -> R {
        self.handler.call(T::pick_up(bag))
    }
}

struct Envelope<T>(T);

impl<T: Clone> Clone for Envelope<T> {
    fn clone(&self) -> Envelope<T> {
        Envelope(self.0.clone())
    }
}

impl<T: fmt::Display> fmt::Display for Envelope<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait PickUp: Sized {
    type Item;

    fn pick_up(bag: &Extensions) -> Self::Item;
}

impl PickUp for Envelope<String> {
    type Item = Envelope<String>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        if let Some(item) = bag.get::<Envelope<String>>() {
            item.clone()
        } else {
            Envelope("".to_string())
        }
    }
}

impl PickUp for Envelope<i32> {
    type Item = Envelope<i32>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        if let Some(item) = bag.get::<Envelope<i32>>() {
            item.clone()
        } else {
            Envelope(0)
        }
    }
}

impl PickUp for () {
    type Item = ();

    fn pick_up(_bag: &Extensions) -> Self::Item {
        ()
    }
}

impl<A: PickUp<Item = A>> PickUp for (A,) {
    type Item = (A,);

    fn pick_up(bag: &Extensions) -> Self::Item {
        (A::pick_up(bag),)
    }
}

impl<A: PickUp<Item = A>, B: PickUp<Item = B>> PickUp for (A, B) {
    type Item = (A, B);

    fn pick_up(bag: &Extensions) -> Self::Item {
        (A::pick_up(bag), B::pick_up(bag))
    }
}

fn main() {
    let f0 = handler0;
    f0();
    f0.call(());

    let f1 = handler1;
    f1(Envelope("Universe".to_string()));
    f1.call((Envelope("Universe".to_string()),));

    let f1i = handler1i;
    f1i(Envelope(42));
    f1i.call((Envelope(42),));

    let f2 = handler2;
    f2(Envelope("Universe".to_string()), Envelope(42));
    f2.call((Envelope("Universe".to_string()), Envelope(42)));

    let f2s = handler2s;
    f2s(Envelope(42), Envelope("Universe".to_string()));
    f2s.call((Envelope(42), Envelope("Universe".to_string())));

    let mut bag = Extensions::new();
    bag.insert(Envelope("Universe".to_string()));
    bag.insert(Envelope(42));

    let d0 = Dispatcher::new(handler0);
    d0.run(&bag);

    let d1 = Dispatcher::new(handler1);
    d1.run(&bag);

    let d2 = Dispatcher::new(handler2);
    d2.run(&bag);

    let d2s = Dispatcher::new(handler2s);
    d2s.run(&bag);
}
