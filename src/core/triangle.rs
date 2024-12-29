use sdl2::pixels::Color;

use crate::core::{math::intersect_plane, triangle};

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
        &mut self,
        plane_position: Vector3D,
        mut plane_normal: Vector3D,
    ) -> Vec<Triangle> {
        plane_normal = plane_normal.from_normalise();

        let dist = |p: &Vector3D| -> f32 {
            let plane_position_dot_product =
                plane_normal.x * p.x + plane_normal.y * p.y + plane_normal.z * p.z;
            plane_position_dot_product - vector_dot_product(&plane_normal, &plane_position)
        };

        let mut inside_points: Vec<&Vector3D> = vec![];
        let mut outside_points: Vec<&Vector3D> = vec![];

        // Get distance of each point to plane
        let distance0 = dist(&self.vectors[0]);
        let distance1 = dist(&self.vectors[1]);
        let distance2 = dist(&self.vectors[2]);

        if distance0 >= 0.0 {
            inside_points.push(&self.vectors[0]);
        } else {
            outside_points.push(&self.vectors[0]);
        }
        if distance1 >= 0.0 {
            inside_points.push(&self.vectors[1]);
        } else {
            outside_points.push(&self.vectors[1]);
        }
        if distance2 >= 0.0 {
            inside_points.push(&self.vectors[2]);
        } else {
            outside_points.push(&self.vectors[2]);
        }

        // 0 points inside? Clip entire triangle
        // 1 point inside? Create a new triangle
        // 2 points inside? Create 2 new triangles
        // 3 points inside? Don't clip
        if inside_points.len() == 0 {
            // Clip entire triangle
            return vec![];
        }
        if inside_points.len() == 3 {
            // Don't clip
            return vec![self.clone()];
        }

        if inside_points.len() == 1 && outside_points.len() == 2 {
            // Create a new triangle
            let mut new_triangle = self.clone();

            new_triangle.vectors[0] = inside_points[0].clone();
            new_triangle.vectors[1] = intersect_plane(
                &plane_position,
                &plane_normal,
                inside_points[0],
                outside_points[0],
            );
            new_triangle.vectors[2] = intersect_plane(
                &plane_position,
                &plane_normal,
                inside_points[0],
                outside_points[1],
            );

            return vec![new_triangle];
        } else if inside_points.len() == 2 && outside_points.len() == 1 {
            // Create 2 new triangles
            let mut new_triangle1 = self.clone();
            let mut new_triangle2 = self.clone();

            // Start with triangle 1
            new_triangle1.vectors[0] = inside_points[0].clone();
            new_triangle1.vectors[1] = inside_points[1].clone();
            new_triangle1.vectors[2] = intersect_plane(
                &plane_position,
                &plane_normal,
                inside_points[0],
                outside_points[0],
            );

            // Continue with triangle 2
            new_triangle2.vectors[0] = inside_points[1].clone();
            new_triangle2.vectors[1] = new_triangle1.vectors[2].clone();
            new_triangle2.vectors[2] = intersect_plane(
                &plane_position,
                &plane_normal,
                inside_points[1],
                outside_points[0],
            );

            return vec![new_triangle1, new_triangle2];
        }

        panic!("Hier mogen we niet belanden!!!");
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
