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

#[cfg(test)]
mod tests {
    use crate::math::arrayalgebra::*;

    #[test]
    fn vector3f_addition() {
        let a = make_array_vector([1.0, 0.0, 1.0]);
        let b = make_array_vector([0.0, 1.0, 0.0]);
        let c = make_array_vector([1.0, 1.0, 1.0]);
        assert_eq!(a + b, c)
    }

    #[test]
    fn vector3f_negation() {
        let a = make_array_vector([1.0, 0.0, 1.0]);
        let b = make_array_vector([-1.0, 0.0, -1.0]);
        assert_eq!(-a, b)
    }

    #[test]
    fn vector3f_scalar_multiplication() {
        let a = make_array_vector([1.0, 0.0, 1.0]);
        let b = make_array_vector([2.0, 0.0, 2.0]);
        let g: f32 = 2.0;
        assert_eq!(a * g, b)
    }

    #[test]
    fn vector3f_coevaluation() {
        let a = make_array_vector([1.0, 0.0, 1.0]);
        let e1 = make_array_vector([1.0, 0.0, 0.0]);
        assert_eq!(e1 * a, 1.0);
    }

    // fn vector3f_in_frame() {

    // }
}
