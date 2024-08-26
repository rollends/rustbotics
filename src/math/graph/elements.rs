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

//! Graph Elements module.
//!
//!
use core::hash::Hash;

use crate::utility::idregistry::Identifier;

/// Pairs the (unique) vertex identifier with a (non-unique) vertex datum, fully
/// describing a vertex in a graph.
#[derive(Clone, PartialEq)]
pub struct VertexDescriptor<Id: Identifier, Data: Clone> {
    id: Id,
    data: Data,
}

/// Pairs the (unique) edge identifier with a (non-unique) edge datum, fully
/// describing an edge in a graph.
#[derive(Clone, PartialEq)]
pub struct EdgeDescriptor<Id: Identifier, WeightData: Clone> {
    id: Id,
    data: WeightData,
}

/// Graph Element trait.
///
/// Uniquely identifiably element of a graph with some category that is tied
/// with metadata.
pub trait GraphElement<IdType: Identifier, Data: Clone> {
    /// Unique identifier amongst a specific category of graph elements.
    fn id(&self) -> &IdType;

    /// Data associated with graph element.
    fn data(&self) -> &Data;

    /// Return a copy of the element with new data.
    fn with_data(&self, new_data: Data) -> Self;

    /// Map data in element to produce a new element with the mapped data.
    fn map<F: FnOnce(Data) -> Data>(self, op: F) -> Self;
}

impl<Id: Identifier, Data: Clone> Hash for VertexDescriptor<Id, Data> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl<Id: Identifier, Data: Clone> Hash for EdgeDescriptor<Id, Data> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl<Id: Identifier, Data: Clone> GraphElement<Id, Data> for VertexDescriptor<Id, Data> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn data(&self) -> &Data {
        &self.data
    }

    fn with_data(&self, new_data: Data) -> Self {
        VertexDescriptor {
            id: self.id().clone(),
            data: new_data,
        }
    }

    fn map<F: FnOnce(Data) -> Data>(self, op: F) -> Self {
        VertexDescriptor {
            id: self.id,
            data: op(self.data),
        }
    }
}

impl<Id: Identifier, Data: Clone> GraphElement<Id, Data> for EdgeDescriptor<Id, Data> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn data(&self) -> &Data {
        &self.data
    }

    fn with_data(&self, new_data: Data) -> Self {
        EdgeDescriptor {
            id: self.id().clone(),
            data: new_data,
        }
    }

    fn map<F: FnOnce(Data) -> Data>(self, op: F) -> Self {
        EdgeDescriptor {
            id: self.id,
            data: op(self.data),
        }
    }
}

pub fn make_edge<Id: Identifier, Data: Clone>(id: Id, data: Data) -> EdgeDescriptor<Id, Data> {
    EdgeDescriptor { id: id, data: data }
}

pub fn make_vertex<Id: Identifier, Data: Clone>(id: Id, data: Data) -> VertexDescriptor<Id, Data> {
    VertexDescriptor { id: id, data: data }
}
