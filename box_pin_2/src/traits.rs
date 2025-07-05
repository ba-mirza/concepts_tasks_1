use core::fmt;
use std::pin::Pin;
use std::rc::Rc;
use std::vec::Vec;
use std::boxed::Box;

pub trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Box<{:?}>", self.get_ref());
    }
}

impl<T: fmt::Debug> SayHi for Rc<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Rc<{:?}>", self.get_ref());
    }
}

impl<T: fmt::Debug> SayHi for Vec<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Vec<{:?}>", self.get_ref());
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from String<{:?}>", self.get_ref());
    }
}

impl SayHi for [u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from &[u8]<{:?}>", self.get_ref());
    }
}

pub trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: fmt::Debug> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        println!("Mutating Box<{:?}>", self.get_mut());        
    }
}