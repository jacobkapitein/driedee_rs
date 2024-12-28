use super::{files::load_from_obj_file, triangle::Triangle, vector_3d::Vector3D};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_file(filename: &str) -> Mesh {
        let mut mesh = Mesh { triangles: vec![] };
        load_from_obj_file(filename, &mut mesh.triangles);
        mesh
    }

    pub fn from_cube() -> Mesh {
        Mesh {
            triangles: vec![
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 0.0),
                    Vector3D::from_coords(0.0, 1.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 0.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 0.0),
                    Vector3D::from_coords(1.0, 0.0, 0.0),
                ]),
                // NORTH
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 1.0),
                    Vector3D::from_coords(1.0, 1.0, 1.0),
                    Vector3D::from_coords(0.0, 1.0, 1.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 1.0, 1.0),
                    Vector3D::from_coords(0.0, 0.0, 1.0),
                ]),
                // EAST
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 1.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 1.0),
                    Vector3D::from_coords(1.0, 0.0, 1.0),
                ]),
                // WEST
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 1.0, 1.0),
                    Vector3D::from_coords(0.0, 1.0, 0.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 1.0, 0.0),
                    Vector3D::from_coords(0.0, 0.0, 0.0),
                ]),
                // TOP
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 1.0, 0.0),
                    Vector3D::from_coords(0.0, 1.0, 1.0),
                    Vector3D::from_coords(1.0, 1.0, 1.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 1.0, 0.0),
                    Vector3D::from_coords(1.0, 1.0, 1.0),
                    Vector3D::from_coords(1.0, 1.0, 0.0),
                ]),
                // BOTTOM
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 0.0, 0.0),
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 1.0),
                    Vector3D::from_coords(0.0, 0.0, 0.0),
                    Vector3D::from_coords(1.0, 0.0, 0.0),
                ]),
            ],
        }
    }

    pub fn from_pyramid() -> Mesh {
        Mesh {
            triangles: vec![
                // BASE (using two triangles to form a square base)
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 0.0), // Bottom-left of the base
                    Vector3D::from_coords(0.0, 1.0, 0.0), // Top-left of the base
                    Vector3D::from_coords(1.0, 0.0, 0.0), // Bottom-right of the base
                ]),
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 1.0, 0.0), // Top-left of the base
                    Vector3D::from_coords(1.0, 1.0, 0.0), // Top-right of the base
                    Vector3D::from_coords(1.0, 0.0, 0.0), // Bottom-right of the base
                ]),
                // SIDE TRIANGLES (each connects the apex to the base)
                // South Triangle
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 0.0), // Base bottom-left
                    Vector3D::from_coords(0.7, 0.7, 1.0), // Apex of the pyramid
                    Vector3D::from_coords(1.0, 0.0, 0.0), // Base bottom-right
                ]),
                // West Triangle
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 0.0, 0.0), // Base bottom-left
                    Vector3D::from_coords(0.7, 0.7, 1.0), // Apex of the pyramid
                    Vector3D::from_coords(0.0, 1.0, 0.0), // Base top-left
                ]),
                // East Triangle
                Triangle::new_from_vectors([
                    Vector3D::from_coords(1.0, 0.0, 0.0), // Base bottom-right
                    Vector3D::from_coords(0.7, 0.7, 1.0), // Apex of the pyramid
                    Vector3D::from_coords(1.0, 1.0, 0.0), // Base top-right
                ]),
                // North Triangle
                Triangle::new_from_vectors([
                    Vector3D::from_coords(0.0, 1.0, 0.0), // Base top-left
                    Vector3D::from_coords(0.7, 0.7, 1.0), // Apex of the pyramid
                    Vector3D::from_coords(1.0, 1.0, 0.0), // Base top-right
                ]),
            ],
        }
    }

    pub fn from_cilinder(num_segments: u32) -> Mesh {
        let mut triangles = Vec::new();

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let top_center = Vector3D::from_coords(0.7, 0.7, 1.0);
            let top1 =
                Vector3D::from_coords(0.7 + 0.7 * angle1.cos(), 0.7 + 0.7 * angle1.sin(), 1.0);
            let top2 =
                Vector3D::from_coords(0.7 + 0.7 * angle2.cos(), 0.7 + 0.7 * angle2.sin(), 1.0);

            triangles.push(Triangle::new_from_vectors([top_center, top1, top2]));
        }

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let bottom_center = Vector3D::from_coords(0.7, 0.7, 0.0);
            let bottom1 =
                Vector3D::from_coords(0.7 + 0.7 * angle1.cos(), 0.7 + 0.7 * angle1.sin(), 0.0);
            let bottom2 =
                Vector3D::from_coords(0.7 + 0.7 * angle2.cos(), 0.7 + 0.7 * angle2.sin(), 0.0);

            triangles.push(Triangle::new_from_vectors([
                bottom_center,
                bottom1,
                bottom2,
            ]));
        }

        for i in 0..num_segments {
            let angle1 = (i as f32) * (2.0 * std::f32::consts::PI / num_segments as f32);
            let angle2 = ((i + 1) % num_segments) as f32
                * (2.0 * std::f32::consts::PI / num_segments as f32);

            let top1 =
                Vector3D::from_coords(0.7 + 0.7 * angle1.cos(), 0.7 + 0.7 * angle1.sin(), 1.0);
            let top2 =
                Vector3D::from_coords(0.7 + 0.7 * angle2.cos(), 0.7 + 0.7 * angle2.sin(), 1.0);
            let bottom1 =
                Vector3D::from_coords(0.7 + 0.7 * angle1.cos(), 0.7 + 0.7 * angle1.sin(), 0.0);
            let bottom2 =
                Vector3D::from_coords(0.7 + 0.7 * angle2.cos(), 0.7 + 0.7 * angle2.sin(), 0.0);

            triangles.push(Triangle::new_from_vectors([
                top1.clone(),
                top2.clone(),
                bottom1.clone(),
            ]));
            triangles.push(Triangle::new_from_vectors([
                top2.clone(),
                bottom2.clone(),
                bottom1.clone(),
            ]));
        }
        Mesh { triangles }
    }
}
