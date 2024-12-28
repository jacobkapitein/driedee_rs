extern crate sdl2;

use sdl2::{
    pixels::Color,
    rect::{FPoint, Point},
    render::Canvas,
    video::Window,
    Sdl,
};

use super::{
    math::interpolate, matrix_4x4::Matrix4X4, mesh::Mesh, triangle::Triangle, vector_3d::Vector3D,
};

pub struct Engine {
    pub sdl_context: Sdl,
    canvas: Canvas<Window>,
    size_x: u32,
    size_y: u32,
    projection_matrix: Matrix4X4,
    mesh_cube: Mesh,
    theta: f32,
    camera: Vector3D,
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

        let projection_matrix =
            Matrix4X4::from_projection(fov, aspect_ratio, near_plane, far_plane);

        Engine {
            sdl_context,
            canvas,
            size_x,
            size_y,
            projection_matrix,
            mesh_cube: Mesh::from_file("./teapot.obj"),
            theta: 0.0,
            camera: Vector3D::new(),
        }
    }

    pub fn on_user_update(&mut self, elapsed_time: f32) -> bool {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let mut mat_rot_z = Matrix4X4::new();
        let mut mat_rot_x = Matrix4X4::new();
        self.theta += 1.0 * elapsed_time;

        mat_rot_z.content[0][0] = self.theta.cos();
        mat_rot_z.content[0][1] = self.theta.sin();
        mat_rot_z.content[1][0] = -self.theta.sin();
        mat_rot_z.content[1][1] = self.theta.cos();
        mat_rot_z.content[2][2] = 1.0;
        mat_rot_z.content[3][3] = 1.0;

        mat_rot_x.content[0][0] = 1.0;
        mat_rot_x.content[1][1] = (self.theta * 0.5).cos();
        mat_rot_x.content[1][2] = (self.theta * 0.5).sin();
        mat_rot_x.content[2][1] = -(self.theta * 0.5).sin();
        mat_rot_x.content[2][2] = (self.theta * 0.5).cos();
        mat_rot_x.content[3][3] = 1.0;

        let mut triangles_to_draw: Vec<Triangle> = Vec::new();

        // Now, draw the triangles
        for triangle in &self.mesh_cube.triangles {
            let mut projected_triangle = triangle.clone();
            projected_triangle.base_color = Color::RGB(255, 0, 0);
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

            // Translate in world space
            let mut translated_triangle = rotated_zx.clone();
            translated_triangle.vector3d[0].z = rotated_zx.vector3d[0].z + 6.0;
            translated_triangle.vector3d[1].z = rotated_zx.vector3d[1].z + 6.0;
            translated_triangle.vector3d[2].z = rotated_zx.vector3d[2].z + 6.0;

            // Calculate normals
            let mut normal = Vector3D::new();
            let mut line1 = Vector3D::new();
            let mut line2 = Vector3D::new();

            line1.x = translated_triangle.vector3d[1].x - translated_triangle.vector3d[0].x;
            line1.y = translated_triangle.vector3d[1].y - translated_triangle.vector3d[0].y;
            line1.z = translated_triangle.vector3d[1].z - translated_triangle.vector3d[0].z;

            line2.x = translated_triangle.vector3d[2].x - translated_triangle.vector3d[0].x;
            line2.y = translated_triangle.vector3d[2].y - translated_triangle.vector3d[0].y;
            line2.z = translated_triangle.vector3d[2].z - translated_triangle.vector3d[0].z;

            normal.x = line1.y * line2.z - line1.z * line2.y;
            normal.y = line1.z * line2.x - line1.x * line2.z;
            normal.z = line1.x * line2.y - line1.y * line2.x;

            let normal_length =
                (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            normal.x /= normal_length;
            normal.y /= normal_length;
            normal.z /= normal_length;

            if (normal.x * (translated_triangle.vector3d[0].x - self.camera.x)
                + normal.y * (translated_triangle.vector3d[0].y - self.camera.y)
                + normal.z * (translated_triangle.vector3d[0].z - self.camera.z))
                >= 0.0
            {
                continue;
            }

            // Calculate illumination
            let mut light_direction = Vector3D::new();
            light_direction.z = -1.0;
            let light_normal_length = (light_direction.x * light_direction.x
                + light_direction.y * light_direction.y
                + light_direction.z * light_direction.z)
                .sqrt();
            light_direction.x /= light_normal_length;
            light_direction.y /= light_normal_length;
            light_direction.z /= light_normal_length;
            let dot_product = normal.x * light_direction.x
                + normal.y * light_direction.y
                + normal.z * light_direction.z;

            projected_triangle.base_color =
                self.get_color(dot_product, projected_triangle.base_color);

            // Project triangles from 3D to 2D
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

            triangles_to_draw.push(projected_triangle);
        }

        // First, sort all the triangles
        triangles_to_draw.sort_by(|t1, t2| {
            let z1 = (t1.vector3d[0].z + t1.vector3d[1].z + t1.vector3d[2].z) / 3.0;
            let z2 = (t2.vector3d[0].z + t2.vector3d[1].z + t2.vector3d[2].z) / 3.0;

            z2.partial_cmp(&z1).unwrap_or(std::cmp::Ordering::Equal)
        });

        for projected_triangle in triangles_to_draw {
            // self.draw_wireframe(&projected_triangle);
            self.draw_filled_triangle(&projected_triangle);
        }

        // Finally, show the buffer
        self.canvas.present();

        true
    }

    fn get_color(&self, lum: f32, base_color: Color) -> Color {
        // Clamp luminance between 0.0 and 1.0
        let luminance = lum.clamp(0.0, 1.0);

        // Calculate the new color components
        let new_r = (base_color.r as f32 * luminance) as u8;
        let new_g = (base_color.g as f32 * luminance) as u8;
        let new_b = (base_color.b as f32 * luminance) as u8;

        Color::RGB(new_r, new_g, new_b)
    }

    pub fn draw_filled_triangle(&mut self, projected_triangle: &Triangle) {
        // Order projected points from top to bottom
        let mut ordered_points = vec![
            &projected_triangle.vector3d[0],
            &projected_triangle.vector3d[1],
            &projected_triangle.vector3d[2],
        ];
        if ordered_points[1].y < ordered_points[0].y {
            ordered_points.swap(1, 0);
        }
        if ordered_points[2].y < ordered_points[0].y {
            ordered_points.swap(2, 0);
        }
        if ordered_points[2].y < ordered_points[1].y {
            ordered_points.swap(2, 1)
        }

        let mut x01 = interpolate(
            ordered_points[0].x,
            ordered_points[0].y,
            ordered_points[1].x,
            ordered_points[1].y,
        );
        let mut x12 = interpolate(
            ordered_points[1].x,
            ordered_points[1].y,
            ordered_points[2].x,
            ordered_points[2].y,
        );
        let x02 = interpolate(
            ordered_points[0].x,
            ordered_points[0].y,
            ordered_points[2].x,
            ordered_points[2].y,
        );

        let x_left;
        let x_right;

        // Remove last
        x01.pop();
        let mut x012 = x01.clone();
        x012.append(&mut x12);

        let m = x02.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        } else {
            x_left = x012;
            x_right = x02;
        }

        self.canvas.set_draw_color(projected_triangle.base_color);
        for y in (ordered_points[0].y as i32)..(ordered_points[2].y as i32) {
            let index = (y as usize).wrapping_sub(ordered_points[0].y as usize);

            // Check if the index is within bounds
            if index < x_left.len() && index < x_right.len() {
                let x_start = x_left[index] as i32;
                let x_end = x_right[index] as i32;

                for x in x_start..x_end {
                    self.canvas
                        .draw_point(Point::new(x, y))
                        .expect("Error drawing pixel");
                }
            }
        }
    }

    pub fn draw_wireframe(&mut self, triangle: &Triangle) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas
            .draw_fline(
                FPoint::new(triangle.vector3d[0].x, triangle.vector3d[0].y),
                FPoint::new(triangle.vector3d[1].x, triangle.vector3d[1].y),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(triangle.vector3d[1].x, triangle.vector3d[1].y),
                FPoint::new(triangle.vector3d[2].x, triangle.vector3d[2].y),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(triangle.vector3d[2].x, triangle.vector3d[2].y),
                FPoint::new(triangle.vector3d[0].x, triangle.vector3d[0].y),
            )
            .expect("Error drawing line");
    }

    pub fn multiply_matrix_vector(
        &self,
        input: &Vector3D,
        output: &mut Vector3D,
        matrix: &Matrix4X4,
    ) {
        output.x = input.x * matrix.content[0][0]
            + input.y * matrix.content[1][0]
            + input.z * matrix.content[2][0]
            + matrix.content[3][0];
        output.y = input.x * matrix.content[0][1]
            + input.y * matrix.content[1][1]
            + input.z * matrix.content[2][1]
            + matrix.content[3][1];
        output.z = input.x * matrix.content[0][2]
            + input.y * matrix.content[1][2]
            + input.z * matrix.content[2][2]
            + matrix.content[3][2];
        let w: f32 = input.x * matrix.content[0][3]
            + input.y * matrix.content[1][3]
            + input.z * matrix.content[2][3]
            + matrix.content[3][3];

        if w != 0.0 {
            output.x /= w;
            output.y /= w;
            output.z /= w;
        }
    }
}
