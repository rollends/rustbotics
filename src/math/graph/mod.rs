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

//! Graph module.
//!
//! Provides traits and implementations supporting basic Graph representation
//! and operations, such as graph mutation and path finding.


use std::hash::Hash;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
use crate::utility::idregistry::IdentifierRegistry;

pub mod elements;

use elements::*;

/// Graph data structure.
/// 
/// Stores a digraph, including both forward edges (that reside in the graph)
/// and backward edges (not in the digraph), and maintains a bidirectional 
/// registry mapping the vertices and edges to their identifiers; this allows
/// users to store the data associated with their vertices and edges in the
/// graph while primarily working with the (hopefully lightweight) identifiers.
pub struct Graph<Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, WeightData : Clone + PartialEq, Registry : IdentifierRegistry<Id>>
{
    vertex_id_registry : Registry,
    edge_id_registry : Registry,
    vertices : HashMap<Id, VertexDescriptor<Id, Data>>,
    edges : HashMap<Id, EdgeDescriptor<Id, WeightData>>,
    forward_edges : HashMap<Id, Vec<(Id, Id)>>,
    backward_edges : HashMap<Id, Vec<(Id, Id)>>,
}


/// Graph Mutator trait.
/// 
/// A graph mutator moves the input graph and mutates it according to some rule
/// to produce a new graph. The old graph is consumed, and, ideally, done so
/// in a way that minimizes (or eliminates) cloning.
pub trait GraphMutator<Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, WeightData : Clone + PartialEq, Registry : IdentifierRegistry<Id>>
{
    fn mutate(&mut self, graph : Graph<Id, Data, WeightData, Registry>) -> Graph<Id, Data, WeightData, Registry>;
}


/// Walk.
/// 
/// Stores a list of vertices (and transiting edges) that move from one vertex
/// to another in a graph.
pub struct Walk<'a, Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, WeightData : Clone + PartialEq>
{
    vertices : LinkedList<&'a VertexDescriptor<Id, Data>>,
    edges : LinkedList<&'a EdgeDescriptor<Id, WeightData>>,
}


/// Graph Visitor trait.
/// 
/// Provides an adapter to graph algorithms that allow for custom logic when
/// traversing a graph.
pub trait GraphVisitor<'a, Id, Data, WeightData>
    where Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, WeightData : Clone + PartialEq
{
    fn reset(&mut self);
    fn visit_vertex(&mut self, vertex : &'a VertexDescriptor<Id, Data>);
    fn visit_edge(&mut self, vertex_from : Id, edge : &'a EdgeDescriptor<Id, WeightData>, vertex_to : Id);
}


impl<Id : Copy + Eq + Hash + Display, Registry : IdentifierRegistry<Id>, Data : Clone + PartialEq, WeightData : Clone + PartialEq> Graph<Id, Data, WeightData, Registry>
{
    /// Creates a new (empty) graph with the given registries.
    pub fn new(vertex_registry : Registry, edge_registry : Registry) -> Graph<Id, Data, WeightData, Registry> {
        Graph {
            vertex_id_registry : vertex_registry,
            edge_id_registry : edge_registry,
            vertices : HashMap::new(),
            edges : HashMap::new(),
            forward_edges : HashMap::new(),
            backward_edges : HashMap::new(),
        }
    }

    /// Returns a list of edges and vertices that are (out) neighbours of the 
    /// given vertex.
    pub fn neighbours_of<'a>(&'a self, vertex_id : Id) -> LinkedList<(&'a EdgeDescriptor<Id, WeightData>, &'a VertexDescriptor<Id, Data>)> {
        self.out_neighbours_of(vertex_id)
    }


    /// Checks if the given vertices are adjacent in the sense that the second
    /// vertex is the out neighbour of the first vertex. Returns true if they
    /// are adjacent, false otherwise.
    pub fn is_adjacent(&self, vertex_from : Id, vertex_to : Id) -> bool {
        self.out_neighbours_of(vertex_from)
            .iter()
            .find(|(_, vid_to)| *vid_to.id() == vertex_to)
            .is_some()
    }


    pub fn out_neighbours_of<'a>(&'a self, vertex_id : Id) -> LinkedList<(&'a EdgeDescriptor<Id, WeightData>, &'a VertexDescriptor<Id, Data>)> {
        self.forward_edges.get(&vertex_id)
            .cloned()
            .unwrap_or(Vec::new())
            .iter()
            .map( 
                |(eid, vid)| {
                    let edge = self.edges.get(eid);
                    let vertex = self.vertices.get(vid);

                    (
                        edge.expect(format!("Graph is ill-formed. Expected edge id {eid} was not found in graph.").as_str()),
                        vertex.expect(format!("Graph is ill-formed. Expected vertex id {vid} was not found in graph.").as_str())
                    )
                }
            )
            .collect()
    }


    pub fn in_neighbours_of<'a>(&'a self, vertex_id : Id)-> LinkedList<(&'a EdgeDescriptor<Id, WeightData>, &'a VertexDescriptor<Id, Data>)> {
        self.backward_edges.get(&vertex_id)
            .cloned()
            .unwrap_or(Vec::new())
            .iter()
            .map( 
                |(eid, vid)| {
                    let edge = self.edges.get(eid);
                    let vertex = self.vertices.get(vid);

                    (
                        edge.expect(format!("Graph is ill-formed. Expected edge id {eid} was not found in graph.").as_str()),
                        vertex.expect(format!("Graph is ill-formed. Expected vertex id {vid} was not found in graph.").as_str())
                    )
                }
            )
            .collect()
    }


    /// Creates a graph with the same vertices and edges except the edges 
    /// are reversed.
    pub fn reverse_graph(self) -> Graph<Id, Data, WeightData, Registry> {
        Graph {
            vertex_id_registry : self.vertex_id_registry,
            edge_id_registry : self.edge_id_registry,
            vertices : self.vertices,
            edges : self.edges,
            forward_edges : self.backward_edges,
            backward_edges : self.forward_edges,
        }
    }


    pub fn select_vertices_with_data<'a>(&'a self, desc : Data) -> LinkedList<&'a VertexDescriptor<Id, Data>> { 
        self.vertices
            .values()
            .filter(|other_desc| desc == *other_desc.data())
            .collect()
    }
}


pub struct VertexCollector<'a, Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, F : Fn(&Data) -> bool>
{
    vertices : LinkedList<&'a VertexDescriptor<Id, Data>>,
    selector : F
}


impl<'a, Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, F : Fn(&Data)->bool> VertexCollector<'a, Id, Data, F> {

    pub fn vertices(&self) -> &LinkedList<&'a VertexDescriptor<Id, Data>> {
        &self.vertices
    }
}


pub fn make_vertex_collector<'a, Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, F : Fn(&Data) -> bool>(selector : F) -> VertexCollector<'a, Id, Data, F> {
    VertexCollector {
        vertices : LinkedList::new(),
        selector : selector
    }
}


impl<'a, Id : Copy + Eq + Hash + Display, Data : Clone + PartialEq, WeightData : Clone + PartialEq, F : Fn(&Data)->bool> GraphVisitor<'a, Id, Data, WeightData> for VertexCollector<'a, Id, Data, F> 
{
    fn reset(&mut self) {
        self.vertices = LinkedList::new()
    }

    fn visit_vertex(&mut self, vertex : &'a VertexDescriptor<Id, Data>) {
        if (self.selector)(vertex.data()) {
            self.vertices.push_back(vertex)
        }
    }

    fn visit_edge(&mut self, _ : Id, _ : &'a EdgeDescriptor<Id, WeightData>, _ : Id) { }
}

pub mod mutators;
mod tests;

pub fn breadth_first_traversal<'a, Id : Copy + Eq + Hash + Display, Registry : IdentifierRegistry<Id>, Data : Clone + PartialEq, WeightData : Clone + PartialEq, V : GraphVisitor<'a, Id, Data, WeightData>>(graph : &'a Graph<Id, Data, WeightData, Registry>, source : Id, visitor : &mut V) {
    assert!(graph.vertices.contains_key(&source), "The breadth-first search must begin on a vertex in the graph.");

    let mut transition_queue = VecDeque::new();
    let mut covered_vertices = HashSet::new();

    visitor.reset();
    
    transition_queue.push_back((None, source));
    covered_vertices.insert(source);

    loop {
        let transition = transition_queue.pop_front();

        match transition {
            None => {
                break;
            },
            Some((maybe_edge_id, vertex_id)) => {
                let vertex: &VertexDescriptor<Id, Data> = graph.vertices.get(&vertex_id).unwrap();
    
                maybe_edge_id.map(
                    |(from_vertex_id, edge_id) : (Id, Id)| {
                        let edge = graph.edges.get(&edge_id).unwrap();
                        visitor.visit_edge(from_vertex_id, edge, vertex_id)
                    }
                );
                
                visitor.visit_vertex(vertex);
    
                for (edge_id, to_vertex_id) in graph.forward_edges.get(&vertex_id).unwrap_or(&Vec::new()) {
                    let new_transition = (Some((vertex_id, *edge_id)), *to_vertex_id);

                    if !covered_vertices.contains(to_vertex_id) {
                        covered_vertices.insert(*to_vertex_id);
                        transition_queue.push_back(new_transition);
                    }
                }

            },
        }
    }
}