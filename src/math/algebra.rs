/*
Copyright 2024 Rollen S. D'Souza

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors
   may be used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

//! Algebra module.
//!
//! Generic traits for algebraic operations. Realistically, the majority of
//! applications will use the array-backed implementation of the vector algebra
//! that uses the f32 primitive for the underlying field. However, this module
//! supports the implementation of, for instance, the vector space of integer
//! vectors of dimension N over some prime field.

use std::cmp::PartialEq;
use std::marker::Copy;
use std::ops::{Add, Mul, Neg};

/// Scalar trait for describing types satisfying the field axioms.
pub trait Scalar:
    Sized
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + PartialEq
    + Clone
    + Copy
{
    /// Returns the additive unit in the field.
    fn additive_unit() -> Self;

    /// Returns the multiplicative unit in the field.
    fn multiplicative_unit() -> Self;

    // Returns the additive inverse in the field.
    fn additive_inverse(self) -> Self;
}

/// Vector trait for describing types supporting vector addition and scalar
/// multiplication.
pub trait Vector<Field>:
    Sized
    + Add<Self, Output = Self>
    + Neg<Output = Self>
    + Mul<Field, Output = Self>
    + PartialEq
    + Clone
    + Copy
where
    Field: Scalar,
{
}

/// Covector trait for describing types that act as vectors but also can
/// multiply vectors to produce scalars.
pub trait Covector<Field, V>: Vector<Field> + Mul<V, Output = Field>
where
    Field: Scalar,
{
}

/// Linear Map trait for describing types that act as linear maps on vectors
/// from one vector space to another.
pub trait LinearMap<Field, Domain, Codomain>: Mul<Domain, Output = Codomain>
where
    Field: Scalar,
    Domain: Vector<Field>,
    Codomain: Vector<Field>,
{
}

/// Default implementation of Scalar for the primitive f32.
impl Scalar for f32 {
    fn additive_unit() -> f32 {
        0.0
    }

    fn additive_inverse(self) -> f32 {
        -self
    }

    fn multiplicative_unit() -> f32 {
        1.0
    }
}

/// Default implementation of Scalar for the primitive f64.
impl Scalar for f64 {
    fn additive_unit() -> f64 {
        0.0
    }

    fn additive_inverse(self) -> f64 {
        -self
    }

    fn multiplicative_unit() -> f64 {
        1.0
    }
}
