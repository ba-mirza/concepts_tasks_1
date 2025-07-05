mod traits;
use traits::*;

use std::{pin::Pin, rc::{Rc}};

#[allow(unused)]
trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}


fn main() {

    let byte_slice: &[u8] = "Hello, world!".as_bytes();
    let pinned_byte_slice = Pin::new(byte_slice);
    pinned_byte_slice.say_hi();

    let string_value = String::from("Hello, world!");
    let pinned_string_value = Pin::new(&string_value);
    pinned_string_value.say_hi();

    let rc_value = Rc::new(42);
    let pinned_rc_value = Pin::new(&rc_value);
    pinned_rc_value.say_hi();

    let vec_value = vec![1, 2, 3];
    let pinned_vec_value = Pin::new(&vec_value);
    pinned_vec_value.say_hi();

    let boxed_value = Box::new(42);
    let pinned_boxed_value = Pin::new(&boxed_value);
    pinned_boxed_value.say_hi();
}
