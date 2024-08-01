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
    use std::marker::PhantomData;

    use crate::{math::graph::*, utility::idregistry::ExplicitIntegralIdentifierRegistry};

    struct CountingGraphVisitor {
        vertex_count: usize,
        edge_count: usize,
    }

    #[test]
    fn graph_empty() {
        let _: Graph<usize, f32, f32, _> = Graph::new(
            ExplicitIntegralIdentifierRegistry::null_registry(),
            ExplicitIntegralIdentifierRegistry::null_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "The breadth-first search must begin on a vertex in the graph.")]
    fn graph_empty_visiting() {
        let g: Graph<usize, f32, f32, _> = Graph::new(
            ExplicitIntegralIdentifierRegistry::null_registry(),
            ExplicitIntegralIdentifierRegistry::null_registry(),
        );
        let mut visitor = CountingGraphVisitor {
            vertex_count: 0,
            edge_count: 0,
        };

        breadth_first_traversal(&g, 0, &mut visitor);
    }

    #[test]
    fn graph_simple_visiting() {
        let mut g: Graph<usize, f32, f32, _> = Graph::new(
            ExplicitIntegralIdentifierRegistry::new(3),
            ExplicitIntegralIdentifierRegistry::new(2),
        );

        let v1 = mutators::add_vertex(&mut g, 1.5);
        let v2 = mutators::add_vertex(&mut g, 1.5);
        let v3 = mutators::add_vertex(&mut g, 1.5);
        mutators::add_edge(&mut g, v2, v3, 2.0);

        let mut visitor = CountingGraphVisitor {
            vertex_count: 0,
            edge_count: 0,
        };

        breadth_first_traversal(&g, v1, &mut visitor);

        assert_eq!(visitor.edge_count, 0);
        assert_eq!(visitor.vertex_count, 1);

        breadth_first_traversal(&g, v2, &mut visitor);

        assert_eq!(visitor.edge_count, 1);
        assert_eq!(visitor.vertex_count, 2);

        breadth_first_traversal(&g, v3, &mut visitor);

        assert_eq!(visitor.edge_count, 0);
        assert_eq!(visitor.vertex_count, 1);
    }

    #[test]
    fn graph_nontrivial_bfs() {
        #[derive(Clone, PartialEq)]
        enum VertexTag {
            V1,
            V2,
            V3,
            V4,
            V5,
        }

        let mut g: Graph<usize, VertexTag, PhantomData<f32>, _> = Graph::new(
            ExplicitIntegralIdentifierRegistry::new(6),
            ExplicitIntegralIdentifierRegistry::new(12),
        );

        let v1 = mutators::add_vertex(&mut g, VertexTag::V1);
        let v2 = mutators::add_vertex(&mut g, VertexTag::V2);
        let v3 = mutators::add_vertex(&mut g, VertexTag::V3);
        let v4 = mutators::add_vertex(&mut g, VertexTag::V4);
        let v5 = mutators::add_vertex(&mut g, VertexTag::V5);

        mutators::add_edge(&mut g, v1, v2, PhantomData);
        mutators::add_edge(&mut g, v1, v3, PhantomData);
        mutators::add_edge(&mut g, v1, v4, PhantomData);

        mutators::add_edge(&mut g, v3, v2, PhantomData);
        mutators::add_edge(&mut g, v3, v5, PhantomData);
        mutators::add_edge(&mut g, v3, v4, PhantomData);

        mutators::add_edge(&mut g, v4, v5, PhantomData);
        mutators::add_edge(&mut g, v4, v1, PhantomData);

        mutators::add_edge(&mut g, v2, v5, PhantomData);

        mutators::add_edge(&mut g, v5, v2, PhantomData);

        // BFS from V1 should result in the entire vertex set.
        {
            let mut vertex_collector = VertexCollector::new(|_| true);
            breadth_first_traversal(&g, v1, &mut vertex_collector);
            let g_bfs: LinkedList<usize> = vertex_collector
                .vertices()
                .iter()
                .map(|vdesc| vdesc.id().clone())
                .collect();
            assert_eq!(g_bfs, LinkedList::from([v1, v2, v3, v4, v5]))
        }
        {
            // BFS from V2 and V5 are just the two element set containing V2 and V5.
            let mut vertex_collector = VertexCollector::new(|_| true);
            breadth_first_traversal(&g, v2, &mut vertex_collector);
            let g_bfs: LinkedList<usize> = vertex_collector
                .vertices()
                .iter()
                .map(|vdesc| vdesc.id().clone())
                .collect();
            assert_eq!(g_bfs, LinkedList::from([v2, v5]));
        }
        {
            // BFS from V2 and V5 are just the two element set containing V2 and V5.
            let mut vertex_collector = VertexCollector::new(|_| true);
            breadth_first_traversal(&g, v5, &mut vertex_collector);
            let g_bfs: LinkedList<usize> = vertex_collector
                .vertices()
                .iter()
                .map(|vdesc| vdesc.id().clone())
                .collect();
            assert_eq!(g_bfs, LinkedList::from([v5, v2]));
        }
        {
            // BFS from V3 is the entire set.
            let mut vertex_collector = VertexCollector::new(|_| true);
            breadth_first_traversal(&g, v3, &mut vertex_collector);
            let g_bfs: LinkedList<usize> = vertex_collector
                .vertices()
                .iter()
                .map(|vdesc| vdesc.id().clone())
                .collect();
            assert_eq!(g_bfs, LinkedList::from([v3, v2, v5, v4, v1]))
        }
        {
            // BFS from V4 is the entire set.
            let mut vertex_collector = VertexCollector::new(|_| true);
            breadth_first_traversal(&g, v4, &mut vertex_collector);
            let g_bfs: LinkedList<usize> = vertex_collector
                .vertices()
                .iter()
                .map(|vdesc| vdesc.id().clone())
                .collect();
            assert_eq!(g_bfs, LinkedList::from([v4, v5, v1, v2, v3]))
        }
    }

    impl<'a> GraphVisitor<'a, usize, f32, f32> for CountingGraphVisitor {
        fn reset(&mut self) {
            self.vertex_count = 0;
            self.edge_count = 0;
        }

        fn visit_vertex(&mut self, _: &'a VertexDescriptor<usize, f32>) {
            self.vertex_count += 1;
        }

        fn visit_edge(&mut self, _: usize, _: &'a EdgeDescriptor<usize, f32>, _: usize) {
            self.edge_count += 1;
        }
    }
}
