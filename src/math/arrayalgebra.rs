//! Real vector-space algebra implementation using arrays over f32.
//!
//! Provides a default implementation of vectors, covectors and other
//! algebraic structures that is backed by a f32 array.

use crate::math::algebra::{Covector, Vector};
use std::cmp::PartialEq;
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Add, Mul, Neg};

/// Array backed vector.
#[derive(Clone, Copy)]
pub struct ArrayVector<const N: usize> {
    data: [f32; N],
}

pub fn make_array_vector<const N: usize>(array: [f32; N]) -> ArrayVector<N> {
    ArrayVector { data: array }
}

impl<const N: usize> Debug for ArrayVector<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.data.fmt(f)
    }
}

impl<const N: usize> Add<Self> for ArrayVector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_data: [f32; N] = self.data;

        for n in 0..N {
            new_data[n] = new_data[n] + rhs.data[n];
        }

        ArrayVector { data: new_data }
    }
}

impl<const N: usize> Neg for ArrayVector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut data = self.data;

        for n in 0..N {
            data[n] = -data[n]
        }

        ArrayVector { data: data }
    }
}

/// Scalar multiplication for array-backed vector.
impl<const N: usize> Mul<f32> for ArrayVector<N> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        ArrayVector {
            data: self.data.map(|a| a * rhs),
        }
    }
}

/// Vector multiplication for array-backed vector. Used by covector.
impl<const N: usize> Mul<ArrayVector<N>> for ArrayVector<N> {
    type Output = f32;

    fn mul(self, rhs: ArrayVector<N>) -> Self::Output {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a * b)
            .fold(0.0, |a, b| a + b)
    }
}

impl<const N: usize> PartialEq for ArrayVector<N> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<const N: usize> Vector<f32> for ArrayVector<N> {}

impl<const N: usize> Covector<f32, ArrayVector<N>> for ArrayVector<N> {}
