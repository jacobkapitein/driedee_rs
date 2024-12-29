extern crate sdl2;

use sdl2::{
    pixels::Color,
    rect::{FPoint, Point},
    render::Canvas,
    video::Window,
    Sdl,
};

use super::{
    math::{interpolate, vector_cross_product, vector_dot_product},
    matrix_4x4::Matrix4X4,
    mesh::Mesh,
    triangle::Triangle,
    vector_3d::Vector3D,
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
    look_direction: Vector3D,
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
            mesh_cube: Mesh::from_file("C:\\Users\\jacob\\Downloads\\axis.obj"),
            theta: 0.0,
            camera: Vector3D::new(),
            look_direction: Vector3D::from_coords(0.0, 0.0, 1.0),
        }
    }

    pub fn on_user_update(&mut self, elapsed_time: f32) -> bool {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        // self.theta += 1.0 * elapsed_time;
        let rotation_z_matrix = Matrix4X4::from_rotation_z(self.theta * 0.5);
        let rotation_x_matrix = Matrix4X4::from_rotation_x(self.theta);

        let translation_matrix = Matrix4X4::from_translation(0.0, 0.0, 8.0);

        let mut world_matrix: Matrix4X4 = &rotation_z_matrix * &rotation_x_matrix;
        world_matrix = &world_matrix * &translation_matrix;

        let up_vector = Vector3D::from_coords(0.0, 1.0, 0.0);
        let target_vector = &self.camera + &up_vector;

        let camera_matrix = Matrix4X4::from_point_at(&self.camera, &target_vector, &up_vector);
        let view_matrix = camera_matrix.quick_inverse();

        let mut triangles_to_draw: Vec<Triangle> = Vec::new();

        // Now, draw the triangles
        for triangle in &self.mesh_cube.triangles {
            let mut projected_triangle = Triangle::new();
            let mut transformed_triangle = Triangle::new();
            let mut viewed_triangle = Triangle::new();

            transformed_triangle.vectors[0] = &world_matrix * &triangle.vectors[0];
            transformed_triangle.vectors[1] = &world_matrix * &triangle.vectors[1];
            transformed_triangle.vectors[2] = &world_matrix * &triangle.vectors[2];

            // Calculate normals
            let line1 = &transformed_triangle.vectors[1] - &transformed_triangle.vectors[0];
            let line2 = &transformed_triangle.vectors[2] - &transformed_triangle.vectors[0];

            // Get cross product of lines to get normal to triangle surface
            let normal = vector_cross_product(&line1, &line2).from_normalise();

            let camera_ray = &transformed_triangle.vectors[0] - &self.camera;

            // Temporarily off for debugging purposes
            if vector_dot_product(&normal, &camera_ray) >= 0.0 {
                continue;
            }

            // Calculate illumination
            let light_direction = Vector3D::from_coords(0.0, 0.0, -1.0).from_normalise();
            let dot_product = f32::max(0.1, vector_dot_product(&light_direction, &normal));

            projected_triangle.base_color =
                self.get_color(dot_product, projected_triangle.base_color);

            // Convert world space to view space
            viewed_triangle.vectors[0] = &view_matrix * &transformed_triangle.vectors[0];
            viewed_triangle.vectors[1] = &view_matrix * &transformed_triangle.vectors[1];
            viewed_triangle.vectors[2] = &view_matrix * &transformed_triangle.vectors[2];

            // Project triangles from 3D to 2D
            projected_triangle.vectors[0] = &self.projection_matrix * &viewed_triangle.vectors[0];
            projected_triangle.vectors[1] = &self.projection_matrix * &viewed_triangle.vectors[1];
            projected_triangle.vectors[2] = &self.projection_matrix * &viewed_triangle.vectors[2];

            projected_triangle.vectors[0] =
                &projected_triangle.vectors[0] / projected_triangle.vectors[0].w;
            projected_triangle.vectors[1] =
                &projected_triangle.vectors[1] / projected_triangle.vectors[1].w;
            projected_triangle.vectors[2] =
                &projected_triangle.vectors[2] / projected_triangle.vectors[2].w;

            let offset_view = Vector3D::from_coords(1.0, 1.0, 0.0);
            projected_triangle.vectors[0] = &projected_triangle.vectors[0] + &offset_view;
            projected_triangle.vectors[1] = &projected_triangle.vectors[1] + &offset_view;
            projected_triangle.vectors[2] = &projected_triangle.vectors[2] + &offset_view;

            projected_triangle.vectors[0].x *= 0.5 * self.size_x as f32;
            projected_triangle.vectors[0].y *= 0.5 * self.size_y as f32;
            projected_triangle.vectors[1].x *= 0.5 * self.size_x as f32;
            projected_triangle.vectors[1].y *= 0.5 * self.size_y as f32;
            projected_triangle.vectors[2].x *= 0.5 * self.size_x as f32;
            projected_triangle.vectors[2].y *= 0.5 * self.size_y as f32;

            triangles_to_draw.push(projected_triangle);
        }

        // First, sort all the triangles
        triangles_to_draw.sort_by(|t1, t2| {
            let z1 = (t1.vectors[0].z + t1.vectors[1].z + t1.vectors[2].z) / 3.0;
            let z2 = (t2.vectors[0].z + t2.vectors[1].z + t2.vectors[2].z) / 3.0;

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
            &projected_triangle.vectors[0],
            &projected_triangle.vectors[1],
            &projected_triangle.vectors[2],
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
                FPoint::new(triangle.vectors[0].x, triangle.vectors[0].y),
                FPoint::new(triangle.vectors[1].x, triangle.vectors[1].y),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(triangle.vectors[1].x, triangle.vectors[1].y),
                FPoint::new(triangle.vectors[2].x, triangle.vectors[2].y),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(triangle.vectors[2].x, triangle.vectors[2].y),
                FPoint::new(triangle.vectors[0].x, triangle.vectors[0].y),
            )
            .expect("Error drawing line");
    }
}
