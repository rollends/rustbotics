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
    use std::borrow::Borrow;
    use std::marker::PhantomData;

    use crate::math::arrayalgebra::*;
    use crate::robotics::kinematics::*;

    #[derive(Debug, Clone)]
    struct Node {
        id: Option<usize>,
        data: PhantomData<u8>,
    }

    type Vector4 = ArrayVector<4>;

    impl Frame<f32, Vector4, Vector4, usize, PhantomData<u8>> for Node {
        fn basis(&self, axis: Axes) -> FramedVector<f32, Vector4, Vector4, usize> {
            self.frame_vector(make_array_vector(match axis {
                Axes::X => [1.0, 0.0, 0.0, 0.0],
                Axes::Y => [0.0, 1.0, 0.0, 0.0],
                Axes::Z => [0.0, 0.0, 1.0, 0.0],
                Axes::O => [0.0, 0.0, 0.0, 1.0],
            }))
        }

        fn cobasis(&self, axis: Axes) -> FramedCovector<f32, Vector4, Vector4, usize> {
            self.frame_covector(make_array_vector(match axis {
                Axes::X => [1.0, 0.0, 0.0, 0.0],
                Axes::Y => [0.0, 1.0, 0.0, 0.0],
                Axes::Z => [0.0, 0.0, 1.0, 0.0],
                Axes::O => [0.0, 0.0, 0.0, 1.0],
            }))
        }

        fn new() -> Self {
            Node {
                id: None,
                data: PhantomData,
            }
        }

        fn id(&self) -> &usize {
            self.id
                .as_ref()
                .expect("Expected an identifier for this frame but an identifier was not assigned.")
        }

        fn data(&self) -> &PhantomData<u8> {
            &self.data
        }

        fn with_id(self, id: usize) -> Self {
            Node {
                id: Some(id),
                data: self.data,
            }
        }

        fn with_data(self, data: PhantomData<u8>) -> Self {
            Node {
                id: self.id,
                data: data,
            }
        }
    }

    #[test]
    fn inertial_frame() {
        let f0 = Node::new().with_id(0);

        assert_eq!(*f0.id(), 0);

        // Frame is orthonormalized.
        assert_eq!(f0.d_x() * f0.x(), 1.0);
        assert_eq!(f0.d_y() * f0.y(), 1.0);
        assert_eq!(f0.d_z() * f0.z(), 1.0);
        assert_eq!(f0.d_o() * f0.o(), 1.0);

        assert_eq!(f0.d_x() * f0.y(), 0.0);
        assert_eq!(f0.d_x() * f0.z(), 0.0);
        assert_eq!(f0.d_x() * f0.o(), 0.0);

        assert_eq!(f0.d_y() * f0.x(), 0.0);
        assert_eq!(f0.d_y() * f0.z(), 0.0);
        assert_eq!(f0.d_y() * f0.o(), 0.0);

        assert_eq!(f0.d_z() * f0.x(), 0.0);
        assert_eq!(f0.d_z() * f0.y(), 0.0);
        assert_eq!(f0.d_z() * f0.o(), 0.0);

        assert_eq!(f0.d_o() * f0.x(), 0.0);
        assert_eq!(f0.d_o() * f0.y(), 0.0);
        assert_eq!(f0.d_o() * f0.z(), 0.0);
    }

    #[test]
    fn inertial_frame_coordinates() {
        let f0 = Node::new().with_id(0);
        let v1 = f0.lift(FrameComponent::Position, [1.0, 2.0, 1.0]);

        // Coordinates should be such that we can pull them out with the covectors.
        assert_eq!(f0.d_x() * v1.clone(), 1.0);
        assert_eq!(f0.d_y() * v1.clone(), 2.0);
        assert_eq!(f0.d_z() * v1.clone(), 1.0);

        // Or by projecting down.
        assert_eq!(f0.project(v1.clone()), [1.0, 2.0, 1.0]);
    }

    #[test]
    #[should_panic(
        expected = "Invalid operation. Attempted to add a vector in frame 1 with a vector in frame 0."
    )]
    fn invalid_addition_of_vectors() {
        let f0 = Node::new().with_id(0);
        let f1 = Node::new().with_id(1);

        let v0 = f0.lift(FrameComponent::Position, [0.0, 1.0, 2.0]);
        let p1 = f1.lift(FrameComponent::Velocity, [3.0, 1.0, 2.0]);

        let _ = p1 + v0;
    }

    #[test]
    #[should_panic(
        expected = "Invalid operation. Attempted to add a covector in frame 0 with a covector in frame 1."
    )]
    fn invalid_addition_of_covectors() {
        let f0 = Node::new().with_id(0);
        let f1 = Node::new().with_id(1);

        let v0 = f0.to_covector(f0.lift(FrameComponent::Position, [0.0, 1.0, 2.0]));
        let p1 = f1.to_covector(f1.lift(FrameComponent::Velocity, [3.0, 1.0, 2.0]));

        let _ = v0 + p1;
    }

    #[test]
    #[should_panic(
        expected = "Invalid operation. Attempted to dualize a vector in frame 0 with frame 1 operations."
    )]
    fn invalid_dualize_vector() {
        let f0 = Node::new().with_id(0);
        let f1 = Node::new().with_id(1);

        let _ = f1.to_covector(f0.lift(FrameComponent::Position, [0.0, 1.0, 2.0]));
    }

    #[test]
    #[should_panic(
        expected = "Invalid operation. Attempted to dualize a covector in frame 0 with frame 1 operations."
    )]
    fn invalid_dualize_covector() {
        let f0 = Node::new().with_id(0);
        let f1 = Node::new().with_id(1);

        let v = f0.to_covector(f0.lift(FrameComponent::Position, [0.0, 1.0, 2.0]));
        let _ = f1.to_vector(v);
    }

    #[test]
    fn kinematic_graph_construction() {
        let name = "World".to_string();
        let graph: KinematicGraph<f32, Vector4, Vector4, PhantomData<u8>, Node> =
            KinematicGraph::new();

        assert_eq!(*graph.get_frame(&name).id(), 0);
    }

    #[test]
    #[should_panic(expected = "No frame with name NotAFrame found in the kinematic graph.")]
    fn invalid_frame_in_kinematic_graph() {
        let graph: KinematicGraph<f32, Vector4, Vector4, PhantomData<u8>, Node> =
            KinematicGraph::new();
        let _ = graph.get_frame("NotAFrame".to_string().borrow());
    }

    #[test]
    fn adding_frame_to_kinematic_graph() {
        let mut graph: KinematicGraph<f32, Vector4, Vector4, PhantomData<u8>, Node> =
            KinematicGraph::new();

        graph = graph.add_frame("F1".into(), PhantomData);

        let f_world = graph.get_frame("World".to_string().borrow());
        let f_1 = graph.get_frame("F1".to_string().borrow());

        assert_eq!(*f_1.id(), 1);
    }

    #[test]
    fn kinematic_graph_transform_vectors() {
        let mut graph: KinematicGraph<f32, Vector4, Vector4, PhantomData<u8>, Node> =
            KinematicGraph::new();
        let world_frame = "World".to_string();
        let f1_frame = "F1".to_string();

        graph = graph.add_frame(f1_frame.clone(), PhantomData);

        let f0 = graph.get_frame(&world_frame).clone();
        let v3;
        let origin;

        let v1 = f0.lift(
            FrameComponent::Velocity,
            [-1.0 / 2.0_f32.sqrt(), 0.0, 1.0 / 2.0_f32.sqrt()],
        );
        let v2 = f0.lift(
            FrameComponent::Velocity,
            ([1.0 / 2.0_f32.sqrt(), 0.0, 1.0 / 2.0_f32.sqrt()]),
        );
        v3 = f0.lift(FrameComponent::Velocity, ([0.0, 1.0, 0.0]));
        origin = f0.lift(FrameComponent::Position, ([1.0, 2.0, 3.0]));

        graph.add_frame_transformation(
            &world_frame,
            &f1_frame,
            v1.clone(),
            v2.clone(),
            v3.clone(),
            origin.clone(),
        );

        let target_frame = graph.get_frame(&f1_frame);
        let h_1_0 = graph.get_frame_transformation(&world_frame, &f1_frame);

        let vx = h_1_0.push_forward(target_frame, v1);
        assert!((target_frame.d_x() * vx.clone() - 1.0).abs() < f32::EPSILON);
        assert!((target_frame.d_y() * vx.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_z() * vx.clone()).abs() < f32::EPSILON);

        let vy = h_1_0.push_forward(target_frame, v2);
        assert!((target_frame.d_x() * vy.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_y() * vy.clone() - 1.0).abs() < f32::EPSILON);
        assert!((target_frame.d_z() * vy.clone()).abs() < f32::EPSILON);

        let vz = h_1_0.push_forward(target_frame, v3);
        assert!((target_frame.d_x() * vz.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_y() * vz.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_z() * vz.clone() - 1.0).abs() < f32::EPSILON);

        let origin_f1 = h_1_0.push_forward(target_frame, origin);
        assert!((target_frame.d_x() * origin_f1.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_y() * origin_f1.clone()).abs() < f32::EPSILON);
        assert!((target_frame.d_z() * origin_f1.clone()).abs() < f32::EPSILON);
    }
}
