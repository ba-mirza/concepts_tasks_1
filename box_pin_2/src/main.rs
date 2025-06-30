use core::fmt;
use std::pin::Pin;

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Box<{:?}>", self.get_ref());
    }
}

fn main() {
    let boxed_value = Box::new(42);
    let pinned_boxed_value = Pin::new(&boxed_value);
    pinned_boxed_value.say_hi();
}
