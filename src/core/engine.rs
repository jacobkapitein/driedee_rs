extern crate sdl2;
use sdl2::{pixels::Color, rect::FPoint, render::Canvas, video::Window, Sdl};

use super::{matrix_4x4::Matrix4X4, mesh::Mesh, triangle::Triangle, vector_3d::Vector3D};

pub struct Engine {
    pub sdl_context: Sdl,
    canvas: Canvas<Window>,
    size_x: u32,
    size_y: u32,
    projection_matrix: Matrix4X4,
    mesh_cube: Mesh,
    theta: f32,
}

impl Engine {
    pub fn new(title: &str, size_x: u32, size_y: u32) -> Engine {
        let sdl_context = sdl2::init().expect("Error creating SDL context");
        let video_subsystem = sdl_context.video().expect("Error creating video subsystem");

        let window = video_subsystem
            .window(title, size_x, size_y)
            .position_centered()
            .opengl()
            .build()
            .expect("Error creating window");

        let canvas = window.into_canvas().build().expect("Error creating canvas");

        let near_plane: f32 = 0.1;
        let far_plane: f32 = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio: f32 = size_y as f32 / size_x as f32;
        let fov_rad = 1.0 / (fov * 0.5 / 180.0 * 3.14159).tan();

        let mut projection_matrix = Matrix4X4::new();
        projection_matrix.matrix[0][0] = aspect_ratio * fov_rad;
        projection_matrix.matrix[1][1] = fov_rad;
        projection_matrix.matrix[2][2] = far_plane / (far_plane - near_plane);
        projection_matrix.matrix[3][2] = (-far_plane * near_plane) / (far_plane - near_plane);
        projection_matrix.matrix[2][3] = 1.0;
        projection_matrix.matrix[3][3] = 0.0;

        Engine {
            sdl_context,
            canvas,
            size_x,
            size_y,
            projection_matrix,
            mesh_cube: Mesh::from_cilinder(12),
            theta: 0.0,
        }
    }

    pub fn on_user_update(&mut self, elapsed_time: f32) -> bool {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut mat_rot_z = Matrix4X4::new();
        let mut mat_rot_x = Matrix4X4::new();
        self.theta += 1.0 * elapsed_time;

        mat_rot_z.matrix[0][0] = self.theta.cos();
        mat_rot_z.matrix[0][1] = self.theta.sin();
        mat_rot_z.matrix[1][0] = -self.theta.sin();
        mat_rot_z.matrix[1][1] = self.theta.cos();
        mat_rot_z.matrix[2][2] = 1.0;
        mat_rot_z.matrix[3][3] = 1.0;

        mat_rot_x.matrix[0][0] = 1.0;
        mat_rot_x.matrix[1][1] = (self.theta * 0.5).cos();
        mat_rot_x.matrix[1][2] = (self.theta * 0.5).sin();
        mat_rot_x.matrix[2][1] = -(self.theta * 0.5).sin();
        mat_rot_x.matrix[2][2] = (self.theta * 0.5).cos();
        mat_rot_x.matrix[3][3] = 1.0;

        // Now, draw the triangles
        for triangle in &self.mesh_cube.triangles {
            let mut projected_triangle = Triangle::new();
            let mut rotated_z = Triangle::new();
            let mut rotated_zx = Triangle::new();

            self.multiply_matrix_vector(
                &triangle.vector3d[0],
                &mut rotated_z.vector3d[0],
                &mat_rot_x,
            );
            self.multiply_matrix_vector(
                &triangle.vector3d[1],
                &mut rotated_z.vector3d[1],
                &mat_rot_x,
            );
            self.multiply_matrix_vector(
                &triangle.vector3d[2],
                &mut rotated_z.vector3d[2],
                &mat_rot_x,
            );

            self.multiply_matrix_vector(
                &rotated_z.vector3d[0],
                &mut rotated_zx.vector3d[0],
                &mat_rot_z,
            );
            self.multiply_matrix_vector(
                &rotated_z.vector3d[1],
                &mut rotated_zx.vector3d[1],
                &mat_rot_z,
            );
            self.multiply_matrix_vector(
                &rotated_z.vector3d[2],
                &mut rotated_zx.vector3d[2],
                &mat_rot_z,
            );

            let mut translated_triangle = rotated_zx.clone();
            translated_triangle.vector3d[0].z = rotated_zx.vector3d[0].z + 3.0;
            translated_triangle.vector3d[1].z = rotated_zx.vector3d[1].z + 3.0;
            translated_triangle.vector3d[2].z = rotated_zx.vector3d[2].z + 3.0;

            self.multiply_matrix_vector(
                &translated_triangle.vector3d[0],
                &mut projected_triangle.vector3d[0],
                &self.projection_matrix,
            );
            self.multiply_matrix_vector(
                &translated_triangle.vector3d[1],
                &mut projected_triangle.vector3d[1],
                &self.projection_matrix,
            );
            self.multiply_matrix_vector(
                &translated_triangle.vector3d[2],
                &mut projected_triangle.vector3d[2],
                &self.projection_matrix,
            );

            projected_triangle.vector3d[0].x += 1.0;
            projected_triangle.vector3d[0].y += 1.0;
            projected_triangle.vector3d[1].x += 1.0;
            projected_triangle.vector3d[1].y += 1.0;
            projected_triangle.vector3d[2].x += 1.0;
            projected_triangle.vector3d[2].y += 1.0;

            projected_triangle.vector3d[0].x *= 0.5 * self.size_x as f32;
            projected_triangle.vector3d[0].y *= 0.5 * self.size_y as f32;
            projected_triangle.vector3d[1].x *= 0.5 * self.size_x as f32;
            projected_triangle.vector3d[1].y *= 0.5 * self.size_y as f32;
            projected_triangle.vector3d[2].x *= 0.5 * self.size_x as f32;
            projected_triangle.vector3d[2].y *= 0.5 * self.size_y as f32;

            self.canvas
                .draw_fline(
                    FPoint::new(
                        projected_triangle.vector3d[0].x,
                        projected_triangle.vector3d[0].y,
                    ),
                    FPoint::new(
                        projected_triangle.vector3d[1].x,
                        projected_triangle.vector3d[1].y,
                    ),
                )
                .expect("Error drawing line");
            self.canvas
                .draw_fline(
                    FPoint::new(
                        projected_triangle.vector3d[1].x,
                        projected_triangle.vector3d[1].y,
                    ),
                    FPoint::new(
                        projected_triangle.vector3d[2].x,
                        projected_triangle.vector3d[2].y,
                    ),
                )
                .expect("Error drawing line");
            self.canvas
                .draw_fline(
                    FPoint::new(
                        projected_triangle.vector3d[2].x,
                        projected_triangle.vector3d[2].y,
                    ),
                    FPoint::new(
                        projected_triangle.vector3d[0].x,
                        projected_triangle.vector3d[0].y,
                    ),
                )
                .expect("Error drawing line");
        }

        // Finally, show the buffer
        self.canvas.present();

        true
    }

    pub fn multiply_matrix_vector(
        &self,
        input: &Vector3D,
        output: &mut Vector3D,
        matrix: &Matrix4X4,
    ) {
        output.x = input.x * matrix.matrix[0][0]
            + input.y * matrix.matrix[1][0]
            + input.z * matrix.matrix[2][0]
            + matrix.matrix[3][0];
        output.y = input.x * matrix.matrix[0][1]
            + input.y * matrix.matrix[1][1]
            + input.z * matrix.matrix[2][1]
            + matrix.matrix[3][1];
        output.z = input.x * matrix.matrix[0][2]
            + input.y * matrix.matrix[1][2]
            + input.z * matrix.matrix[2][2]
            + matrix.matrix[3][2];
        let w: f32 = input.x * matrix.matrix[0][3]
            + input.y * matrix.matrix[1][3]
            + input.z * matrix.matrix[2][3]
            + matrix.matrix[3][3];

        if w != 0.0 {
            output.x /= w;
            output.y /= w;
            output.z /= w;
        }
    }
}
