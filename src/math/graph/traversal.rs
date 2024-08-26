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

use crate::math::graph::*;

/// Breadth-First Traversal.
///
/// Performs a breadth-first traversal (BFT) on the graph from the given vertex
/// and applies the provided visitor to every edge and vertex it visits in
/// order. Due to how BFT is performed, the traversal of an edge happens just
/// before the out vertex it corresponds to is visited.
pub fn breadth_first_traversal<
    'a,
    Id: Identifier,
    Registry: IdentifierRegistry<Id>,
    Data: Clone + PartialEq,
    WeightData: Clone + PartialEq,
    V: GraphVisitor<'a, Id, Data, WeightData>,
>(
    graph: &'a Graph<Id, Data, WeightData, Registry>,
    source: Id,
    visitor: &mut V,
) {
    assert!(
        graph.vertices.contains_key(&source),
        "The breadth-first search must begin on a vertex in the graph."
    );

    let mut transition_queue = VecDeque::new();
    let mut covered_vertices = HashSet::new();

    visitor.reset();

    transition_queue.push_back((None, source));
    covered_vertices.insert(source);

    while !visitor.should_terminate() {
        let transition = transition_queue.pop_front();

        match transition {
            None => {
                break;
            }
            Some((maybe_edge_id, vertex_id)) => {
                let vertex: &VertexDescriptor<Id, Data> = graph.vertices.get(&vertex_id).unwrap();

                maybe_edge_id.map(|(from_vertex_id, edge_id): (Id, Id)| {
                    let edge = graph.edges.get(&edge_id).unwrap();
                    visitor.visit_edge(from_vertex_id, edge, vertex_id)
                });

                visitor.visit_vertex(vertex);

                for (edge_id, to_vertex_id) in
                    graph.forward_edges.get(&vertex_id).unwrap_or(&Vec::new())
                {
                    let new_transition = (Some((vertex_id, *edge_id)), *to_vertex_id);

                    if !covered_vertices.contains(to_vertex_id) {
                        covered_vertices.insert(*to_vertex_id);
                        transition_queue.push_back(new_transition);
                    }
                }
            }
        }
    }
}
