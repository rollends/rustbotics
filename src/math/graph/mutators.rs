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

//! Graph Mutators module.
//!
//! Provides implementations of fundamental mutators of a graph.

use std::marker::PhantomData;

use crate::math::graph::*;

pub struct GraphVertexAdditionMutator<Id: Identifier, Data: Clone> {
    vertex_id: Option<Id>,
    vertex_data: Option<Data>,
}

pub struct GraphVertexReplacementMutator<
    Id: Identifier,
    Data: Clone,
    Operator: FnOnce(Data) -> Data,
> {
    vertex_id: Option<Id>,
    vertex_operator: Option<Operator>,
    _data: PhantomData<Data>,
}

pub struct GraphEdgeAdditionMutator<Id: Identifier, Data: Clone> {
    edge_id: Option<Id>,
    edge_desc: Option<(Id, Data, Id)>,
}

impl<Id: Identifier, Data: Clone> GraphVertexAdditionMutator<Id, Data> {
    fn new(data: Data) -> Self {
        GraphVertexAdditionMutator {
            vertex_id: None,
            vertex_data: Some(data),
        }
    }
}

impl<Id: Identifier, Data: Clone, Operator: FnOnce(Data) -> Data>
    GraphVertexReplacementMutator<Id, Data, Operator>
{
    fn new(id: Id, operator: Operator) -> Self {
        GraphVertexReplacementMutator {
            vertex_id: Some(id),
            vertex_operator: Some(operator),
            _data: PhantomData,
        }
    }
}

impl<Id: Identifier, Data: Clone> GraphEdgeAdditionMutator<Id, Data> {
    fn new(vfrom: Id, data: Data, vto: Id) -> Self {
        GraphEdgeAdditionMutator {
            edge_id: None,
            edge_desc: Some((vfrom, data, vto)),
        }
    }
}

impl<Id: Identifier, Data: Clone, WeightData: Clone, Registry: IdentifierRegistry<Id>>
    GraphMutator<Id, Data, WeightData, Registry> for GraphVertexAdditionMutator<Id, Data>
{
    fn mutate(
        &mut self,
        graph: Graph<Id, Data, WeightData, Registry>,
    ) -> Graph<Id, Data, WeightData, Registry> {
        let data = self
            .vertex_data
            .take()
            .expect("Vertex addition mutator has already been used.");

        let mut vertex_registry = graph.vertex_id_registry;
        let mut vertices = graph.vertices;

        let new_id = vertex_registry
            .acquire_id()
            .expect("Unable to acquire new identifier for new vertex.");
        self.vertex_id = Some(new_id);

        let vertex = make_vertex(new_id, data);
        vertices.insert(new_id, vertex);

        Graph {
            vertex_id_registry: vertex_registry,
            edge_id_registry: graph.edge_id_registry,
            vertices: vertices,
            edges: graph.edges,
            forward_edges: graph.forward_edges,
            backward_edges: graph.backward_edges,
        }
    }
}

impl<
        Id: Identifier,
        Data: Clone,
        WeightData: Clone,
        Registry: IdentifierRegistry<Id>,
        Operator: FnOnce(Data) -> Data,
    > GraphMutator<Id, Data, WeightData, Registry>
    for GraphVertexReplacementMutator<Id, Data, Operator>
{
    fn mutate(
        &mut self,
        graph: Graph<Id, Data, WeightData, Registry>,
    ) -> Graph<Id, Data, WeightData, Registry> {
        let id = self
            .vertex_id
            .expect("Vertex replacement mutator has already been used.");

        let mut vertices = graph.vertices;
        let vertex = vertices
            .remove(&id)
            .expect(format!("Vertex id {} was not found in graph.", id).as_str())
            .map(
                self.vertex_operator
                    .take()
                    .expect("Vertex operator was not set. This should not happen. Panic!"),
            );
        vertices.insert(id, vertex);

        Graph {
            vertex_id_registry: graph.vertex_id_registry,
            edge_id_registry: graph.edge_id_registry,
            vertices: vertices,
            edges: graph.edges,
            forward_edges: graph.forward_edges,
            backward_edges: graph.backward_edges,
        }
    }
}

impl<Id: Identifier, Data: Clone, WeightData: Clone, Registry: IdentifierRegistry<Id>>
    GraphMutator<Id, Data, WeightData, Registry> for GraphEdgeAdditionMutator<Id, WeightData>
{
    fn mutate(
        &mut self,
        graph: Graph<Id, Data, WeightData, Registry>,
    ) -> Graph<Id, Data, WeightData, Registry> {
        let (vertex_from_id, data, vertex_to_id) = self
            .edge_desc
            .take()
            .expect("Edge addition mutator has already been used.");

        let mut edge_registry = graph.edge_id_registry;
        let mut edges = graph.edges;
        let mut forward_edges = graph.forward_edges;
        let mut backward_edges = graph.backward_edges;

        let new_id = edge_registry
            .acquire_id()
            .expect("Unable to acquire new identifier for new edge.");
        self.edge_id = Some(new_id);

        let edge = make_edge(new_id, data);

        edges.insert(new_id, edge);
        forward_edges
            .entry(vertex_from_id)
            .or_insert(Vec::new())
            .push((new_id, vertex_to_id.clone()));
        backward_edges
            .entry(vertex_to_id)
            .or_insert(Vec::new())
            .push((new_id, vertex_from_id.clone()));

        Graph {
            vertex_id_registry: graph.vertex_id_registry,
            edge_id_registry: edge_registry,
            vertices: graph.vertices,
            edges: edges,
            forward_edges: forward_edges,
            backward_edges: backward_edges,
        }
    }
}

/// Adds a vertex into the graph.
///
/// Mutates the given graph (in-place) by adding a new vertex with the given
/// data and returns the id associated with the new vertex.
pub fn add_vertex<
    Id: Identifier,
    Data: Clone,
    WeightData: Clone,
    Registry: IdentifierRegistry<Id>,
>(
    graph: &mut Graph<Id, Data, WeightData, Registry>,
    data: Data,
) -> Id {
    let empty_graph = Graph::new(Registry::null_registry(), Registry::null_registry());
    let mut current_graph: Graph<Id, Data, WeightData, Registry> =
        std::mem::replace(graph, empty_graph);

    let mut vertex_adder = GraphVertexAdditionMutator::new(data);
    current_graph = vertex_adder.mutate(current_graph);

    let _ = std::mem::replace(graph, current_graph);

    vertex_adder
        .vertex_id
        .take()
        .expect("Failed to insert vertex in graph for an unknown reason.")
}

/// Replaces a vertex in a graph.
///
/// Mutates the given graph (in-place) by changing a vertex.
pub fn map_vertex<
    Id: Identifier,
    Data: Clone,
    WeightData: Clone,
    Registry: IdentifierRegistry<Id>,
    Operator: FnOnce(Data) -> Data,
>(
    graph: &mut Graph<Id, Data, WeightData, Registry>,
    id: Id,
    operator: Operator,
) {
    let empty_graph = Graph::new(Registry::null_registry(), Registry::null_registry());
    let mut current_graph: Graph<Id, Data, WeightData, Registry> =
        std::mem::replace(graph, empty_graph);

    let mut vertex_replacer = GraphVertexReplacementMutator::new(id, operator);
    current_graph = vertex_replacer.mutate(current_graph);

    let _ = std::mem::replace(graph, current_graph);
}

/// Adds a edge into the graph.
///
/// Mutates the given graph (in-place) by adding a new edge between the two
/// vertices (of the given ids) and with the given data. The method returns the
/// id associated with the new edge.
pub fn add_edge<
    Id: Identifier,
    Data: Clone,
    WeightData: Clone,
    Registry: IdentifierRegistry<Id>,
>(
    graph: &mut Graph<Id, Data, WeightData, Registry>,
    vertex_from: Id,
    vertex_to: Id,
    data: WeightData,
) -> Id {
    let empty_graph = Graph::new(Registry::null_registry(), Registry::null_registry());
    let mut current_graph: Graph<Id, Data, WeightData, Registry> =
        std::mem::replace(graph, empty_graph);

    let mut edge_adder = GraphEdgeAdditionMutator::new(vertex_from, data, vertex_to);
    current_graph = edge_adder.mutate(current_graph);

    let _ = std::mem::replace(graph, current_graph);

    edge_adder
        .edge_id
        .take()
        .expect("Failed to insert edge in graph for an unknown reason.")
}
