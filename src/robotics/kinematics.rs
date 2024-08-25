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

use crate::math::algebra::{Covector, Scalar, Vector};
use crate::math::graph;
use crate::math::graph::elements::GraphElement;
use crate::utility::idregistry::{ExplicitIntegralIdentifierRegistry, Identifier};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Neg};

pub enum Axes {
    X,
    Y,
    Z,
    O,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FramedVector<'a, F, V, C, Id>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
    Id: Identifier,
{
    frame: &'a Id,
    vector: V,
    _field: PhantomData<F>,
    _covector: PhantomData<C>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FramedCovector<'a, F, V, C, Id>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
    Id: Identifier,
{
    frame: &'a Id,
    covector: C,
    _field: PhantomData<F>,
    _vector: PhantomData<V>,
}

#[derive(Clone)]
pub struct FrameTransformation<F, V, C>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
{
    x_target_src: C,
    y_target_src: C,
    z_target_src: C,

    d_x_src_target: V,
    d_y_src_target: V,
    d_z_src_target: V,
    d_o_src_target: V,

    r: V,

    _field: PhantomData<F>,
}

pub trait Frame<F, V, C, Id, Data>: Clone
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
    Id: Identifier,
    Data: Clone,
{
    fn new() -> Self;

    fn id(&self) -> &Id;
    fn data(&self) -> &Data;

    fn with_id(self, id: Id) -> Self;
    fn with_data(self, data: Data) -> Self;

    fn basis(&self, axis: Axes) -> FramedVector<F, V, C, Id>;
    fn cobasis(&self, axis: Axes) -> FramedCovector<F, V, C, Id>;

    fn x(&self) -> FramedVector<F, V, C, Id> {
        self.basis(Axes::X)
    }
    fn y(&self) -> FramedVector<F, V, C, Id> {
        self.basis(Axes::Y)
    }
    fn z(&self) -> FramedVector<F, V, C, Id> {
        self.basis(Axes::Z)
    }
    fn o(&self) -> FramedVector<F, V, C, Id> {
        self.basis(Axes::O)
    }

    fn d_x(&self) -> FramedCovector<F, V, C, Id> {
        self.cobasis(Axes::X)
    }
    fn d_y(&self) -> FramedCovector<F, V, C, Id> {
        self.cobasis(Axes::Y)
    }
    fn d_z(&self) -> FramedCovector<F, V, C, Id> {
        self.cobasis(Axes::Z)
    }
    fn d_o(&self) -> FramedCovector<F, V, C, Id> {
        self.cobasis(Axes::O)
    }

    fn frame_vector<'a>(&'a self, vector: V) -> FramedVector<'a, F, V, C, Id> {
        FramedVector {
            frame: self.id(),
            vector: vector,
            _field: PhantomData,
            _covector: PhantomData,
        }
    }

    fn frame_covector<'a>(&'a self, covector: C) -> FramedCovector<'a, F, V, C, Id> {
        FramedCovector {
            frame: self.id(),
            covector: covector,
            _field: PhantomData,
            _vector: PhantomData,
        }
    }

    fn lift<'a>(
        &'a self,
        component: FrameComponent,
        coordinates: [F; 3],
    ) -> FramedVector<'a, F, V, C, Id> {
        match component {
            FrameComponent::Position => {
                self.x() * coordinates[0]
                    + self.y() * coordinates[1]
                    + self.z() * coordinates[2]
                    + self.o() * F::multiplicative_unit()
            }
            FrameComponent::Velocity => {
                self.x() * coordinates[0] + self.y() * coordinates[1] + self.z() * coordinates[2]
            }
        }
    }

    fn colift<'a>(&'a self, coordinates: [F; 4]) -> FramedCovector<'a, F, V, C, Id> {
        self.d_x() * coordinates[0]
            + self.d_y() * coordinates[1]
            + self.d_z() * coordinates[2]
            + self.d_o() * coordinates[3]
    }

    fn project<'a>(&'a self, vector: FramedVector<'a, F, V, C, Id>) -> [F; 3] {
        [
            self.cobasis(Axes::X) * vector.clone(),
            self.cobasis(Axes::Y) * vector.clone(),
            self.cobasis(Axes::Z) * vector.clone(),
        ]
    }

    fn to_covector<'a>(
        &'a self,
        vector: FramedVector<'a, F, V, C, Id>,
    ) -> FramedCovector<'a, F, V, C, Id> {
        if self.id() != vector.frame {
            panic!(
                "Invalid operation. Attempted to dualize a vector in frame {} with frame {} operations.",
                vector.frame,
                self.id(),
            )
        }

        self.d_x() * (self.d_x() * vector.clone())
            + self.d_y() * (self.d_y() * vector.clone())
            + self.d_z() * (self.d_z() * vector.clone())
            + self.d_o() * (self.d_o() * vector)
    }

    fn to_vector<'a>(
        &'a self,
        covector: FramedCovector<'a, F, V, C, Id>,
    ) -> FramedVector<'a, F, V, C, Id> {
        if self.id() != covector.frame {
            panic!(
                "Invalid operation. Attempted to dualize a covector in frame {} with frame {} operations.",
                covector.frame,
                self.id(),
            )
        }

        self.x() * (covector.clone() * self.x())
            + self.y() * (covector.clone() * self.y())
            + self.z() * (covector.clone() * self.z())
            + self.o() * (covector * self.o())
    }
}

pub struct KinematicGraph<F, V, C, FrameData, FrameType>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
    FrameData: Clone,
    FrameType: Frame<F, V, C, usize, FrameData>,
{
    graph: graph::Graph<
        usize,
        FrameType,
        FrameTransformation<F, V, C>,
        ExplicitIntegralIdentifierRegistry,
    >,
    frame_name_map: HashMap<String, usize>,
    _data: PhantomData<FrameData>,
}

impl<F, V, C, FrameData, FrameType> KinematicGraph<F, V, C, FrameData, FrameType>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
    FrameData: Clone,
    FrameType: Frame<F, V, C, usize, FrameData>,
{
    pub fn new() -> Self {
        let mut graph = graph::Graph::new(
            ExplicitIntegralIdentifierRegistry::new(1),
            ExplicitIntegralIdentifierRegistry::new(1),
        );
        let mut frame_name_map = HashMap::new();

        let world_frame_id = graph::mutators::add_vertex(&mut graph, Frame::new());
        frame_name_map.insert("World".to_string(), world_frame_id);

        graph::mutators::map_vertex(&mut graph, world_frame_id, |frame: FrameType| {
            frame.with_id(world_frame_id)
        });

        KinematicGraph {
            graph: graph,
            frame_name_map: frame_name_map,
            _data: PhantomData,
        }
    }

    fn get_frame_id(&self, name: &String) -> usize {
        *self
            .frame_name_map
            .get(name)
            .expect(format!("Kinematic graph couldn't find frame with name {}", name).as_str())
    }

    pub fn add_frame_transformation<'a>(
        &'a mut self,
        frame_source_name: &String,
        frame_target_name: &String,
        target_x: FramedVector<'a, F, V, C, usize>,
        target_y: FramedVector<'a, F, V, C, usize>,
        target_z: FramedVector<'a, F, V, C, usize>,
        target_o: FramedVector<'a, F, V, C, usize>,
    ) {
        let frame_source_id = self.get_frame_id(frame_source_name);
        let frame_target_id = self.get_frame_id(frame_target_name);

        let frame_source = self.graph.get_vertex(frame_source_id).data();
        let frame_target = self.graph.get_vertex(frame_target_id).data();

        let v1: V;
        let v2: V;
        let v3: V;
        let v4: V;
        let w1: C;
        let w2: C;
        let w3: C;
        let r: V;

        {
            let [x1, x2, x3] = frame_source.project(target_x);
            let [y1, y2, y3] = frame_source.project(target_y);
            let [z1, z2, z3] = frame_source.project(target_z);
            let [r1, r2, r3] = frame_source.project(target_o);

            let o1 = -x1 * r1 + -x2 * r2 + -x3 * r3;
            let o2 = -y1 * r1 + -y2 * r2 + -y3 * r3;
            let o3 = -z1 * r1 + -z2 * r2 + -z3 * r3;

            w1 = frame_source.colift([x1, x2, x3, o1]).covector;
            w2 = frame_source.colift([y1, y2, y3, o2]).covector;
            w3 = frame_source.colift([z1, z2, z3, o3]).covector;

            v1 = frame_target
                .lift(FrameComponent::Velocity, [x1, y1, z1])
                .vector;
            v2 = frame_target
                .lift(FrameComponent::Velocity, [x2, y2, z2])
                .vector;
            v3 = frame_target
                .lift(FrameComponent::Velocity, [x3, y3, z3])
                .vector;
            v4 = frame_target
                .lift(FrameComponent::Position, [o1, o2, o3])
                .vector;
            r = frame_target
                .lift(FrameComponent::Velocity, [r1, r2, r3])
                .vector;
        }

        let transformation = FrameTransformation {
            x_target_src: w1,
            y_target_src: w2,
            z_target_src: w3,

            d_x_src_target: v1,
            d_y_src_target: v2,
            d_z_src_target: v3,
            d_o_src_target: v4,

            r: r,

            _field: PhantomData,
        };

        let mut new_graph = KinematicGraph::new();

        std::mem::swap(self, &mut new_graph);

        let mut graph = new_graph.graph;

        graph::mutators::add_edge(&mut graph, frame_source_id, frame_target_id, transformation);

        let mut new_obj = KinematicGraph {
            graph: graph,
            frame_name_map: new_graph.frame_name_map,
            _data: PhantomData,
        };

        std::mem::swap(self, &mut new_obj);
    }

    pub fn add_frame(
        self,
        name: String,
        data: FrameData,
    ) -> KinematicGraph<F, V, C, FrameData, FrameType> {
        let frame_name: String = name;
        let mut graph = self.graph;
        let mut frame_name_map = self.frame_name_map;

        let frame_id = graph::mutators::add_vertex(&mut graph, FrameType::new());
        frame_name_map.insert(frame_name, frame_id);

        graph::mutators::map_vertex(&mut graph, frame_id, |frame: FrameType| {
            frame.with_id(frame_id).with_data(data)
        });

        KinematicGraph {
            graph: graph,
            frame_name_map: frame_name_map,
            _data: PhantomData,
        }
    }

    pub fn get_frame(&self, frame_name: &String) -> &FrameType {
        match self.frame_name_map.get(frame_name) {
            Some(id) => self.graph.get_vertex(*id).data(),
            None => panic!(
                "No frame with name {} found in the kinematic graph.",
                frame_name
            ),
        }
    }

    pub fn get_frame_transformation(
        &self,
        frame_from_name: &String,
        frame_to_name: &String,
    ) -> &FrameTransformation<F, V, C> {
        let frame_from = self.get_frame(frame_from_name);
        let frame_to = self.get_frame(frame_to_name);
        let id_from = *frame_from.id();
        let id_to = *frame_to.id();

        if self.graph.is_adjacent(id_from, id_to) {
            self.graph.get_edge_between(id_from, id_to).data()
        } else {
            panic!("The frames are not directly connected.");
        }
    }
}

impl<'a, Field, V, C, Id> Add<Self> for FramedVector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.frame != rhs.frame {
            panic!(
                "Invalid operation. Attempted to add a vector in frame {} with a vector in frame {}.",
                self.frame,
                rhs.frame,
            )
        }

        FramedVector {
            frame: self.frame,
            vector: self.vector + rhs.vector,
            _field: PhantomData,
            _covector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Mul<Field> for FramedVector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn mul(self, rhs: Field) -> Self::Output {
        FramedVector {
            frame: self.frame,
            vector: self.vector * rhs,
            _field: PhantomData,
            _covector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Neg for FramedVector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        FramedVector {
            frame: self.frame,
            vector: -self.vector,
            _field: PhantomData,
            _covector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Add<Self> for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.frame != rhs.frame {
            panic!(
                "Invalid operation. Attempted to add a covector in frame {} with a covector in frame {}.",
                self.frame,
                rhs.frame,
            )
        }

        FramedCovector {
            frame: self.frame,
            covector: self.covector + rhs.covector,
            _field: PhantomData,
            _vector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Mul<Field> for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn mul(self, rhs: Field) -> Self::Output {
        FramedCovector {
            frame: self.frame,
            covector: self.covector * rhs,
            _field: PhantomData,
            _vector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Neg for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        FramedCovector {
            frame: self.frame,
            covector: -self.covector,
            _field: PhantomData,
            _vector: PhantomData,
        }
    }
}

impl<'a, Field, V, C, Id> Mul<FramedVector<'a, Field, V, C, Id>>
    for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
    type Output = Field;

    fn mul(self, rhs: FramedVector<'a, Field, V, C, Id>) -> Self::Output {
        if self.frame != rhs.frame {
            panic!(
                "Invalid operation. Attempted to multiply a covector in frame {} with a vector in frame {}.",
                self.frame,
                rhs.frame,
            )
        }

        self.covector * rhs.vector
    }
}

impl<'a, Field, V, C, Id> Vector<Field> for FramedVector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
}

impl<'a, Field, V, C, Id> Vector<Field> for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
}

impl<'a, Field, V, C, Id> Covector<Field, FramedVector<'a, Field, V, C, Id>>
    for FramedCovector<'a, Field, V, C, Id>
where
    Field: Scalar,
    V: Vector<Field>,
    C: Covector<Field, V>,
    Id: Identifier,
{
}

pub enum FrameComponent {
    Position,
    Velocity,
}

impl<F, V, C> FrameTransformation<F, V, C>
where
    F: Scalar,
    V: Vector<F>,
    C: Covector<F, V>,
{
    pub fn push_forward<'a, Id: Identifier, Data: Clone, FrameType: Frame<F, V, C, Id, Data>>(
        &self,
        f_target: &'a FrameType,
        vector: FramedVector<'a, F, V, C, Id>,
    ) -> FramedVector<'a, F, V, C, Id> {
        let v = vector.vector;
        let x_target = self.x_target_src.clone() * v.clone();
        let y_target = self.y_target_src.clone() * v.clone();
        let z_target = self.z_target_src.clone() * v.clone();

        f_target.lift(FrameComponent::Position, [x_target, y_target, z_target])
    }

    pub fn pull_back<'a, Id: Identifier, Data: Clone, FrameType: Frame<F, V, C, Id, Data>>(
        &self,
        f_source: &'a FrameType,
        covector: FramedCovector<'a, F, V, C, Id>,
    ) -> FramedCovector<'a, F, V, C, Id> {
        let c = covector.covector;
        let d_x_src = c.clone() * self.d_x_src_target.clone();
        let d_y_src = c.clone() * self.d_x_src_target.clone();
        let d_z_src = c.clone() * self.d_z_src_target.clone();

        f_source.colift([d_x_src, d_y_src, d_z_src, F::additive_unit()])
    }
}

// impl<'a, F, V, C, FrameType> Not for FrameTransformation<'a, F, V, C, FrameType>
//     where F : Scalar, V : Vector<F>, C : Covector<F, V>, FrameType : Frame<F, V, C>
// {
//     type Output = FrameTransformation<'a, F, V, C, FrameType>;

//     fn not(self) -> Self::Output {
//         let & f_source = &self.f_source;
//         let & f_target = &self.f_target;

//         let v1 = self.d_x_src_target;
//         let v2 = self.d_y_src_target;
//         let v3 = self.d_z_src_target;
//         // let v4 = self.d_o_src_target;

//         let [x1, y1, z1] = f_target.project(v1);
//         let [x2, y2, z2] = f_target.project(v2);
//         let [x3, y3, z3] = f_target.project(v3);
//         let [r1, r2, r3] = f_source.project(self.r);

//         let w1_new = f_target.colift([x1, y1, z1, r1]);
//         let w2_new = f_target.colift([x2, y2, z2, r2]);
//         let w3_new = f_target.colift([x3, y3, z3, r3]);

//         let v1_new = f_source.lift(FrameComponent::Velocity, [x1, x2, x3]);
//         let v2_new = f_source.lift(FrameComponent::Velocity, [y1, y2, y3]);
//         let v3_new = f_source.lift(FrameComponent::Velocity, [z1, z2, z3]);
//         let v4_new = f_source.lift(FrameComponent::Position, [r1, r2, r3]);

//         let r_new = self.d_o_src_target;

//         FrameTransformation {
//             f_source : self.f_target,
//             f_target : self.f_source,

//             x_target_src : w1_new,
//             y_target_src : w2_new,
//             z_target_src : w3_new,

//             d_x_src_target : v1_new,
//             d_y_src_target : v2_new,
//             d_z_src_target : v3_new,
//             d_o_src_target : v4_new,

//             r : r_new,

//             _p : PhantomData,
//         }
//     }
// }
