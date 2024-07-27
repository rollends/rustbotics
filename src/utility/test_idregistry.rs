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
    use crate::utility::idregistry::*;

    #[test]
    fn idregistry_make() {
        let _ = make_explicit_id_registry(10);
    }

    #[test]
    #[should_panic(
        expected = "Explicit Integral Identifier Registry expects a positive initial size."
    )]
    fn idregistry_bad_make() {
        let _ = make_explicit_id_registry(0);
    }

    #[test]
    fn idregistry_acquire_id() {
        let mut registry = make_explicit_id_registry(2);
        let mut id1 = 1337;
        let mut id2 = 1337;
        assert_eq!(id1, 1337);
        assert_eq!(id2, 1337);
        id1 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id1, 0);
        id2 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id2, 1);
    }

    #[test]
    fn idregistry_acquire_id_resize() {
        let mut registry = make_explicit_id_registry(2);
        let mut id1 = 1337;
        let mut id2 = 1337;
        assert_eq!(id1, 1337);
        assert_eq!(id2, 1337);
        id1 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id1, 0);
        id2 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id2, 1);
        id2 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id2, 2);
    }

    #[test]
    fn idregistry_improper_release() {
        let mut registry = make_explicit_id_registry(2);
        let mut id1 = 1337;
        let old_id = id1;
        let id2 = 1337;
        id1 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_ne!(id1, old_id);
        assert_eq!(id1, 0);
        registry
            .release_id(id2)
            .expect_err("Successfully freed an unallocated identifier when not expected.");
    }

    #[test]
    fn idregistry_double_release() {
        let mut registry = make_explicit_id_registry(2);
        let id1;
        id1 = registry
            .acquire_id()
            .expect("Failed to acquire an identifier when expected.");
        assert_eq!(id1, 0);
        registry
            .release_id(id1)
            .expect("Failed to free an identifier that was allocated.");
        registry.release_id(id1).expect_err(
            "Successfully freed an identifier that was already freed when not expected.",
        );
    }
}
