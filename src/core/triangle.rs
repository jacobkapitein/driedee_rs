use sdl2::pixels::Color;

use crate::core::math::intersect_plane;

use super::{math::vector_dot_product, vector_3d::Vector3D};

pub struct Triangle {
    pub vectors: [Vector3D; 3],
    pub base_color: Color,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vectors: [Vector3D::new(), Vector3D::new(), Vector3D::new()],
            base_color: Color::RGB(255, 255, 255),
        }
    }

    pub fn new_from_vectors(vectors: [Vector3D; 3]) -> Triangle {
        Triangle {
            vectors,
            base_color: Color::RGB(255, 255, 255),
        }
    }

    pub fn from_vectors(vectors: &Vec<Vector3D>, indices: [usize; 3]) -> Triangle {
        Triangle::new_from_vectors([
            vectors[indices[0]].clone(),
            vectors[indices[1]].clone(),
            vectors[indices[2]].clone(),
        ])
    }

    pub fn clip_against_plane(
        &self,
        plane_position: Vector3D,
        mut plane_normal: Vector3D,
    ) -> Vec<Triangle> {
        plane_normal = plane_normal.from_normalise();

        // Precompute the constant part of the distance calculation
        let plane_offset = vector_dot_product(&plane_normal, &plane_position);

        // Compute the signed distances of the triangle's vertices to the plane
        let mut distances = [0.0; 3];
        for (i, vector) in self.vectors.iter().enumerate() {
            distances[i] = vector_dot_product(&plane_normal, vector) - plane_offset;
        }

        // Classify points as inside or outside
        let mut inside_indices = [0; 3];
        let mut outside_indices = [0; 3];
        let mut num_inside = 0;
        let mut num_outside = 0;

        for (i, &distance) in distances.iter().enumerate() {
            if distance >= 0.0 {
                inside_indices[num_inside] = i;
                num_inside += 1;
            } else {
                outside_indices[num_outside] = i;
                num_outside += 1;
            }
        }

        // Handle cases based on the number of points inside the plane
        match num_inside {
            0 => {
                // No points inside: the triangle is fully clipped
                vec![]
            }
            3 => {
                // All points inside: return the triangle as is
                vec![self.clone()]
            }
            1 => {
                // One point inside: form a single new triangle
                let inside = inside_indices[0];
                let outside1 = outside_indices[0];
                let outside2 = outside_indices[1];

                let mut new_triangle = self.clone();

                new_triangle.vectors[0] = self.vectors[inside].clone();
                new_triangle.vectors[1] = intersect_plane(
                    &plane_position,
                    &plane_normal,
                    &self.vectors[inside],
                    &self.vectors[outside1],
                );
                new_triangle.vectors[2] = intersect_plane(
                    &plane_position,
                    &plane_normal,
                    &self.vectors[inside],
                    &self.vectors[outside2],
                );

                vec![new_triangle]
            }
            2 => {
                // Two points inside: form two new triangles
                let inside1 = inside_indices[0];
                let inside2 = inside_indices[1];
                let outside = outside_indices[0];

                let mut new_triangle1 = self.clone();
                let mut new_triangle2 = self.clone();

                // First triangle
                new_triangle1.vectors[0] = self.vectors[inside1].clone();
                new_triangle1.vectors[1] = self.vectors[inside2].clone();
                new_triangle1.vectors[2] = intersect_plane(
                    &plane_position,
                    &plane_normal,
                    &self.vectors[inside1],
                    &self.vectors[outside],
                );

                // Second triangle
                new_triangle2.vectors[0] = self.vectors[inside2].clone();
                new_triangle2.vectors[1] = new_triangle1.vectors[2].clone();
                new_triangle2.vectors[2] = intersect_plane(
                    &plane_position,
                    &plane_normal,
                    &self.vectors[inside2],
                    &self.vectors[outside],
                );

                vec![new_triangle1, new_triangle2]
            }
            _ => panic!("Unexpected case in clip_against_plane!"),
        }
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            vectors: self.vectors.clone(),
            base_color: self.base_color.clone(),
        }
    }
}
