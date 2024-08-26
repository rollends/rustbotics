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

//! Graph Path Finding module.
//!
//! Provides implementations of path finding algorithms.

use crate::math::graph::*;

/// Walk Builder.
///
/// Used by path finding algorithms to build a walk from one vertex to another.
struct WalkBuilder<'a, Id: Identifier, Data: Clone + PartialEq, WeightData: Clone + PartialEq> {
    initial_vertex_id: Id,
    final_vertex_id: Id,
    current_edge: Option<(Id, &'a EdgeDescriptor<Id, WeightData>, Id)>,
    back_edge_lookup: HashMap<Id, (Id, &'a EdgeDescriptor<Id, WeightData>, Id)>,
    vertex_lookup: HashMap<Id, &'a VertexDescriptor<Id, Data>>,
    path: Option<Walk<'a, Id, Data, WeightData>>,
}

/// Finds a path in a directed graph between two vertices if it exists.
pub fn find_path<
    'a,
    Id: Identifier,
    Registry: IdentifierRegistry<Id>,
    Data: Clone + PartialEq,
    WeightData: Clone + PartialEq,
>(
    graph: &'a Graph<Id, Data, WeightData, Registry>,
    vertex_from_id: Id,
    vertex_to_id: Id,
) -> Option<Walk<'a, Id, Data, WeightData>> {
    assert!(
        graph.vertices.contains_key(&vertex_from_id),
        "The path must begin on a vertex in the graph."
    );
    assert!(
        graph.vertices.contains_key(&vertex_to_id),
        "The path must end on a vertex in the graph."
    );

    let mut walk_builder = WalkBuilder::new(vertex_from_id, vertex_to_id);
    traversal::breadth_first_traversal(graph, vertex_from_id, &mut walk_builder);
    walk_builder.path.take()
}

impl<'a, Id: Identifier, Data: Clone + PartialEq, WeightData: Clone + PartialEq>
    GraphVisitor<'a, Id, Data, WeightData> for WalkBuilder<'a, Id, Data, WeightData>
{
    fn reset(&mut self) {
        self.current_edge.take();
        self.back_edge_lookup = HashMap::new();
        self.vertex_lookup = HashMap::new();
        self.path.take();
    }

    fn visit_vertex(&mut self, vertex: &'a VertexDescriptor<Id, Data>) {
        let id = *vertex.id();

        self.vertex_lookup.insert(id, vertex);

        match self.current_edge.take() {
            None => {}
            Some(edge) => {
                self.back_edge_lookup.insert(id, edge);
            }
        }

        if id == self.final_vertex_id {
            // Build Path.
            self.path.replace(self.build_path());
        }
    }

    fn visit_edge(
        &mut self,
        vertex_from_id: Id,
        edge: &'a EdgeDescriptor<Id, WeightData>,
        vertex_to_id: Id,
    ) {
        self.current_edge
            .replace((vertex_from_id, edge, vertex_to_id));
    }

    fn should_terminate(&self) -> bool {
        self.path.is_some()
    }
}

impl<'a, Id: Identifier, Data: Clone + PartialEq, WeightData: Clone + PartialEq>
    WalkBuilder<'a, Id, Data, WeightData>
{
    fn new(initial_vertex_id: Id, final_vertex_id: Id) -> WalkBuilder<'a, Id, Data, WeightData> {
        WalkBuilder {
            initial_vertex_id: initial_vertex_id,
            final_vertex_id: final_vertex_id,
            current_edge: None,
            back_edge_lookup: HashMap::new(),
            vertex_lookup: HashMap::new(),
            path: None,
        }
    }

    fn build_path(&self) -> Walk<'a, Id, Data, WeightData> {
        let mut vertex_list = LinkedList::new();
        let mut edge_list = LinkedList::new();

        let mut current_vertex = *self
            .vertex_lookup
            .get(&self.final_vertex_id)
            .take()
            .expect("Expected final vertex to have already been traversed.");

        loop {
            vertex_list.push_front(current_vertex);

            if *current_vertex.id() == self.initial_vertex_id {
                break;
            }

            // Get back edge.
            let back_edge = *self
                .back_edge_lookup
                .get(current_vertex.id())
                .take()
                .expect("Expected vertex to have a back-edge if it was traversed.");

            let (from_id, edge, _) = back_edge;

            edge_list.push_front(edge);

            // Change current vertex.
            current_vertex = self
                .vertex_lookup
                .get(&from_id)
                .take()
                .expect("Expected vertex on the path to have already been traversed.");
        }

        Walk {
            vertices: vertex_list,
            edges: edge_list,
        }
    }
}
