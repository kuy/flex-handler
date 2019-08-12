use std::env;
use std::marker::PhantomData;
use std::result::Result;

mod extensions;

use crate::extensions::Extensions;

use env_logger;
use log::error;

fn handler0() {
    println!("handler[0]");
}

fn handler1(a: &str) {
    println!("handler[1]: {}", a);
}

fn handler1i(a: i32) {
    println!("handler[1i]: {}", a);
}

fn handler2(a: &str, b: i32) {
    println!("handler[2]: {}, {}", a, b);
}

fn handler2s(a: i32, b: &str) {
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
    T: PickUp<Item = Result<T, &'static str>>,
{
    fn new(handler: F) -> Self {
        Dispatcher {
            handler,
            _t: PhantomData,
        }
    }

    fn run(&self, bag: &Extensions) -> R {
        match T::pick_up(bag) {
            Ok(param) => self.handler.call(param),
            Err(msg) => {
                error!("{}", msg);
                panic!()
            },
        }
    }
}

trait PickUp: Sized {
    type Item;

    fn pick_up(bag: &Extensions) -> Self::Item;
}

impl PickUp for &str {
    type Item = Option<Self>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        if let Some(item) = bag.get::<&str>() {
            Some(item)
        } else {
            None
        }
    }
}

impl PickUp for i32 {
    type Item = Option<Self>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        if let Some(item) = bag.get::<i32>() {
            Some(item.clone())
        } else {
            None
        }
    }
}

impl PickUp for () {
    type Item = Result<(), &'static str>;

    fn pick_up(_bag: &Extensions) -> Self::Item {
        Ok(())
    }
}

impl<A: PickUp<Item = Option<A>>> PickUp for (A,) {
    type Item = Result<(A,), &'static str>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        let item_a = if let Some(item) = A::pick_up(bag) {
            item
        } else {
            return Err("PickUp Error A of (A,)")
        };

        Ok((item_a,))
    }
}

impl<A: PickUp<Item = Option<A>>, B: PickUp<Item = Option<B>>> PickUp for (A, B) {
    type Item = Result<(A, B), &'static str>;

    fn pick_up(bag: &Extensions) -> Self::Item {
        let item_a = if let Some(item) = A::pick_up(bag) {
            item
        } else {
            return Err("PickUp Error A of (A, B)")
        };

        let item_b = if let Some(item) = B::pick_up(bag) {
            item
        } else {
            return Err("PickUp Error B of (A, B)")
        };

        Ok((item_a, item_b))
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut bag = Extensions::new();
    bag.insert("Universe");
    bag.insert(42);

    let d0 = Dispatcher::new(handler0);
    d0.run(&bag);

    let d1 = Dispatcher::new(handler1);
    d1.run(&bag);

    let d1i = Dispatcher::new(handler1i);
    d1i.run(&bag);

    let d2 = Dispatcher::new(handler2);
    d2.run(&bag);

    let d2s = Dispatcher::new(handler2s);
    d2s.run(&bag);
}
