#![feature(non_lifetime_binders)]

use std::marker::PhantomData;

pub trait Foo<T: ?Sized> {
    type Bar<K: ?Sized>;
}

pub struct Bar<T: ?Sized> {
    pd: PhantomData<T>
}

pub struct Baz {}

impl Foo<usize> for Baz {
    type Bar<K: ?Sized> = Bar<K>;
}

pub fn f<T1, T2>(a: T1, b: T2)
where
    T1: for <T> Foo<usize, Bar<T> = Bar<T>>, 
    T2: for <T> Foo<usize, Bar<T> = T1::Bar<T>>,
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        f(Baz{}, Baz{});
    }
}
