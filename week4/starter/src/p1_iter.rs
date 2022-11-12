//! P1: Cartesian product iterator
//!
//! To get experience with traits and generics, you will implement a new kind
//! of iterator: cartesian product. The product of two iterators is the set
//! of all pairs of items from each iterator. For example:
//!
//! [1, 2] x [3, 4]  =  [(1, 3), (1, 4), (2, 3), (2, 4)]
//!
//! Your task is to design all the structs, traits, and impls that are needed
//! to make this code snippet possible:
//!
//! ```ignore
//! let h1 = hashset![1, 2];
//! let h2 = hashset![3, 4];
//! let product =
//!   h1.into_iter()
//!   .cartesian_product(h2.into_iter())
//!   .collect::<HashSet<_>>();
//! ```
//!
//! That is, there should be a method `cartesian_product` which can be called
//! on *any* iterator, such as the one produced by `HashSet::into_iter`. This method
//! returns a structure that implements the `Iterator` trait, allowing one to call
//! methods like `collect`.
//!
//! The snippet above is provided as a unit test, which you can run with
//! `cargo test product`. The test will not compile until you build the API.
//!
//! To get you started, I would read Rust's documentation on how to implement an iterator:
//! https://doc.rust-lang.org/std/iter/index.html#implementing-iterator

// Your implementation goes here!

// CartesianProduct struct and Iterator implementation for it
// IntoCartesianProduct trait
// impl of IntroCartesianProduct for all Iterator

struct CartesianProduct<A, B> {
    v1: Vec<A>,
    v2: Vec<B>,
    i1: usize,
    i2: usize,
}

impl<A, B> Iterator for CartesianProduct<A, B>
where
    A: Clone,
    B: Clone,
{
    type Item = (A, B);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i1 == self.v1.len() {
            return None;
        }

        let p = (self.v1[self.i1].clone(), self.v2[self.i2].clone());

        if self.i2 == self.v2.len() - 1 {
            self.i1 += 1;
            self.i2 = 0;
        } else {
            self.i2 += 1;
        }

        Some(p)
    }
}

trait IntoCartesianProduct: Iterator {
    fn cartesian_product<V: Iterator>(self, other: V) -> CartesianProduct<Self::Item, V::Item>;
}

impl<T: Iterator> IntoCartesianProduct for T {
    fn cartesian_product<V: Iterator>(self, other: V) -> CartesianProduct<Self::Item, V::Item> {
        CartesianProduct {
            v1: self.collect(),
            v2: other.collect(),
            i1: 0,
            i2: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashset;
    use std::collections::HashSet;

    #[test]
    fn cartesian_product_test() {
        let h1 = hashset![1, 2];
        let h2 = hashset![3, 4];
        let product = h1.into_iter().cartesian_product(h2.into_iter());
        assert_eq!(
            product.collect::<HashSet<_>>(),
            hashset![(1, 3), (1, 4), (2, 3), (2, 4)]
        )
    }
}
