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

//! Identifier Registry module.
//!
//! Provides implementations of a registry object that can be used to acquire
//! and release unique identifiers that can be used to keep track and identify
//! objects.

use std::borrow::BorrowMut;
use std::cmp::min;
use std::collections::{HashSet, LinkedList};
use std::fmt::Display;

/// Identifier Registry Failures.
#[derive(Debug)]
pub enum IdentifierRegistryFailure {
    /// Reported when the registry runs out of unique identifiers.
    OutOfIdentifiers,

    /// Reported when the registry is used with an identifier that isn't valid.
    InvalidIdentifier,

    /// Reported when the registry is asked to free an identifier that is not
    /// in use.
    IdentiferAlreadyReleased,
}

pub trait Identifier: Clone + Copy + Eq + Display + core::hash::Hash {}

/// Identifier Registry Trait.
///
/// Identifier registries support acquisition and release operation for unique
/// identifiers. It must be the case that if y = acquire_id then no future call
/// of acquire_id returns y unless it follows a call to release_id(y). Thus, the
/// user of this trait can use the output of acquire_id as a unique identifier
/// to compare other objects identifed by the same registry.
pub trait IdentifierRegistry<ID: Identifier>: Clone {
    type Identifier;

    /// Builds an empty registry.
    fn null_registry() -> Self;

    /// Retrieves a currently unused identifier, removing it from the registry,
    /// or fails. Failure can occur only if the registry runs out of
    /// unique identifiers.
    fn acquire_id(&mut self) -> Result<ID, IdentifierRegistryFailure>;

    /// Returns the provided identifier to the registry so that it can be
    /// reused, or fails. Failure can occur if the provided identifier was not
    /// expected to be in use, or if the identifier was otherwise invalid.
    fn release_id(&mut self, id: ID) -> Result<(), IdentifierRegistryFailure>;
}

/// Explicit, Integral Identifier Registry.
///
/// This registry maintains a list of available and in-use integer identifiers.
#[derive(Clone)]
pub struct ExplicitIntegralIdentifierRegistry {
    all_ids: HashSet<usize>,
    free_ids: HashSet<usize>,
    free_id_alloc_chain: LinkedList<usize>,
    min_unallocated_id: usize,
}

impl Identifier for usize {}

impl IdentifierRegistry<usize> for ExplicitIntegralIdentifierRegistry {
    type Identifier = usize;

    fn null_registry() -> Self {
        ExplicitIntegralIdentifierRegistry {
            all_ids: HashSet::new(),
            free_ids: HashSet::new(),
            free_id_alloc_chain: LinkedList::new(),
            min_unallocated_id: 0,
        }
    }

    fn acquire_id(&mut self) -> Result<Self::Identifier, IdentifierRegistryFailure> {
        let free_id_alloc_chain = self.free_id_alloc_chain.borrow_mut();

        match free_id_alloc_chain.pop_front() {
            Some(new_id) => {
                self.free_ids.remove(&new_id);
                Ok(new_id)
            }

            None => {
                // must increase size of registry
                let all_ids = self.all_ids.borrow_mut();
                let min_unallocated_id = self.min_unallocated_id;

                let old_min_unallocated_id = min_unallocated_id;
                let new_min_unallocated_id = min_unallocated_id
                    + min(usize::MAX - min_unallocated_id, min_unallocated_id + 1)
                    - 1;

                if old_min_unallocated_id == new_min_unallocated_id {
                    return Err(IdentifierRegistryFailure::OutOfIdentifiers);
                }

                self.min_unallocated_id = new_min_unallocated_id;

                for new_id in old_min_unallocated_id..self.min_unallocated_id {
                    all_ids.insert(new_id);
                    self.free_ids.insert(new_id);
                    free_id_alloc_chain.push_back(new_id);
                }

                self.acquire_id()
            }
        }
    }

    fn release_id(&mut self, id: Self::Identifier) -> Result<(), IdentifierRegistryFailure> {
        if !self.all_ids.contains(&id) {
            return Err(IdentifierRegistryFailure::InvalidIdentifier);
        }

        if self.free_ids.contains(&id) {
            return Err(IdentifierRegistryFailure::IdentiferAlreadyReleased);
        }

        self.free_id_alloc_chain.push_front(id);
        self.free_ids.insert(id);
        Ok(())
    }
}

impl ExplicitIntegralIdentifierRegistry {
    /// Build a registry with a non-zero initial size.
    pub fn new(initial_size: usize) -> Self {
        assert!(
            initial_size > 0,
            "Explicit Integral Identifier Registry expects a positive initial size."
        );

        let mut free_ids = LinkedList::new();
        for i in 0..initial_size {
            free_ids.push_back(i)
        }

        let all_ids_i = free_ids.clone().into_iter();
        let free_ids_i = free_ids.clone().into_iter();

        ExplicitIntegralIdentifierRegistry {
            all_ids: all_ids_i.collect(),
            free_ids: free_ids_i.collect(),
            free_id_alloc_chain: free_ids,
            min_unallocated_id: initial_size,
        }
    }
}
