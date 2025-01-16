extern crate sdl2;

use std::{collections::VecDeque, ffi::NulError, time::Instant};

use sdl2::{
    keyboard::Keycode,
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
    camera: Vector3D,
    look_direction: Vector3D,
    r_yaw: f32,
}

impl Engine {
    pub fn new(title: &str, size_x: u32, size_y: u32) -> Engine {
        let sdl_context = sdl2::init().expect("Error creating SDL context");
        let video_subsystem = sdl_context.video().expect("Error creating video subsystem");

        let window = video_subsystem
            .window(title, size_x, size_y)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .expect("Error creating window");

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .expect("Error creating canvas");

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
            camera: Vector3D::new(),
            look_direction: Vector3D::from_coords(0.0, 0.0, 1.0),
            r_yaw: 0.0,
        }
    }

    pub fn on_user_update(&mut self) -> bool {
        let on_user_update_start = Instant::now();
        let basic_start = Instant::now();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        let translation_matrix = Matrix4X4::from_translation(0.0, 0.0, 4.0);

        let world_matrix = &Matrix4X4::from_identity() * &translation_matrix;

        let up_vector = Vector3D::from_coords(0.0, 1.0, 0.0);
        let mut target_vector = Vector3D::from_coords(0.0, 0.0, 1.0);

        let camera_rotation_matrix = Matrix4X4::from_rotation_y(self.r_yaw);

        self.look_direction = &camera_rotation_matrix * &target_vector;
        target_vector = &self.camera + &self.look_direction;

        let camera_matrix = Matrix4X4::from_point_at(&self.camera, &target_vector, &up_vector);
        let view_matrix = camera_matrix.quick_inverse();

        let mut triangles_to_draw: Vec<Triangle> = Vec::new();

        // println!("Basic start took\t\t{:.2?}", basic_start.elapsed());
        // Do all transformations
        let transformations_start = Instant::now();
        for triangle in &self.mesh_cube.triangles {
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

            // Convert world space to view space
            viewed_triangle.vectors[0] = &view_matrix * &transformed_triangle.vectors[0];
            viewed_triangle.vectors[1] = &view_matrix * &transformed_triangle.vectors[1];
            viewed_triangle.vectors[2] = &view_matrix * &transformed_triangle.vectors[2];

            // Clip viewed triangle against the near plane
            let clipped_triangles = &viewed_triangle.clip_against_plane(
                Vector3D::from_coords(0.0, 0.0, 0.1),
                Vector3D::from_coords(0.0, 0.0, 1.0),
            );

            for clipped_triangle in clipped_triangles {
                let mut projected_triangle = Triangle::new();
                projected_triangle.base_color = clipped_triangle.base_color;
                projected_triangle.base_color =
                    self.get_color(dot_product, projected_triangle.base_color);

                // Project triangles from 3D to 2D
                projected_triangle.vectors[0] =
                    &self.projection_matrix * &clipped_triangle.vectors[0];
                projected_triangle.vectors[1] =
                    &self.projection_matrix * &clipped_triangle.vectors[1];
                projected_triangle.vectors[2] =
                    &self.projection_matrix * &clipped_triangle.vectors[2];

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
        }
        // println!(
        //     "Transformations took\t\t{:.2?}",
        //     transformations_start.elapsed()
        // );

        let sorting_start = Instant::now();
        // First, sort all the triangles
        triangles_to_draw.sort_by(|t1, t2| {
            let z1 = (t1.vectors[0].z + t1.vectors[1].z + t1.vectors[2].z) / 3.0;
            let z2 = (t2.vectors[0].z + t2.vectors[1].z + t2.vectors[2].z) / 3.0;

            z2.partial_cmp(&z1).unwrap_or_else(|| {
                if z1.is_nan() {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
        });
        // println!("Sorting took\t\t\t{:.2?}", sorting_start.elapsed());

        // Rasterize everything to the screen
        // Define clipping planes
        let clipping_planes = [
            (
                Vector3D::from_coords(0.0, 0.0, 0.0),
                Vector3D::from_coords(0.0, 1.0, 0.0),
            ), // Top
            (
                Vector3D::from_coords(0.0, self.size_y as f32 - 1.0, 0.0),
                Vector3D::from_coords(0.0, -1.0, 0.0),
            ), // Bottom
            (
                Vector3D::from_coords(0.0, 0.0, 0.0),
                Vector3D::from_coords(1.0, 0.0, 0.0),
            ), // Left
            (
                Vector3D::from_coords(self.size_x as f32 - 1.0, 0.0, 0.0),
                Vector3D::from_coords(-1.0, 0.0, 0.0),
            ), // Right
        ];
        let rasterization_start = Instant::now();
        for triangle_to_draw in triangles_to_draw {
            let mut triangle_queue: VecDeque<Triangle> = VecDeque::new();
            triangle_queue.push_back(triangle_to_draw);

            for (plane_position, plane_normal) in &clipping_planes {
                let mut next_queue = VecDeque::new();

                while let Some(test_triangle) = triangle_queue.pop_front() {
                    // Clip triangle against the current plane
                    let clipped_triangles = test_triangle
                        .clip_against_plane(plane_position.clone(), plane_normal.clone());

                    // Add clipped triangles to the next queue
                    for clipped_triangle in clipped_triangles {
                        next_queue.push_back(clipped_triangle);
                    }
                }

                // Swap queues for the next iteration
                triangle_queue = next_queue;
            }

            // Draw all remaining triangles in the queue
            for final_triangle in triangle_queue {
                self.draw_filled_triangle(&final_triangle);
                // self.draw_wireframe(&final_triangle);
            }
        }
        // println!("Rasterizing took\t\t{:.2?}", rasterization_start.elapsed());

        let presenting_start = Instant::now();
        // Finally, show the buffer
        self.canvas.present();
        // println!("Presenting took\t\t\t{:.2?}", presenting_start.elapsed());

        // println!(
        //     "On user update took\t{:.2?}",
        //     on_user_update_start.elapsed()
        // );

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
        // Order projected points from top to bottom by their `y` value
        let mut ordered_points = projected_triangle.vectors.clone();
        ordered_points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        // Deconstruct the sorted points
        let p0 = &ordered_points[0];
        let p1 = &ordered_points[1];
        let p2 = &ordered_points[2];

        // Interpolate x-coordinates along the edges
        let x01 = interpolate(p0.x, p0.y, p1.x, p1.y);
        let x12 = interpolate(p1.x, p1.y, p2.x, p2.y);
        let x02 = interpolate(p0.x, p0.y, p2.x, p2.y);

        // Merge x01 and x12 for the full edge from p0 to p2
        let mut x_full = x01;
        x_full.pop(); // Avoid duplicating the middle point
        x_full.extend(x12);

        // Determine left and right edges based on midpoint comparison
        let x_left;
        let x_right;
        let mid_index = x02.len() / 2;

        if x02[mid_index] < x_full[mid_index] {
            x_left = x02;
            x_right = x_full;
        } else {
            x_left = x_full;
            x_right = x02;
        }

        // Draw the triangle
        self.canvas.set_draw_color(projected_triangle.base_color);

        // Ensure we stay within bounds of the interpolation arrays
        let start_y = p0.y as i32;
        let end_y = p2.y as i32;

        for y in start_y..end_y {
            let index = (y - start_y) as usize;

            if index < x_left.len() && index < x_right.len() {
                let x_start = x_left[index] as i32;
                let x_end = x_right[index] as i32;

                for x in x_start..x_end {
                    self.canvas
                        .draw_point(Point::new(self.size_x as i32 - x, self.size_y as i32 - y))
                        .expect("Error drawing pixel");
                }
            }
        }
    }

    pub fn draw_wireframe(&mut self, triangle: &Triangle) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas
            .draw_fline(
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[0].x,
                    self.size_y as f32 - triangle.vectors[0].y,
                ),
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[1].x,
                    self.size_y as f32 - triangle.vectors[1].y,
                ),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[1].x,
                    self.size_y as f32 - triangle.vectors[1].y,
                ),
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[2].x,
                    self.size_y as f32 - triangle.vectors[2].y,
                ),
            )
            .expect("Error drawing line");
        self.canvas
            .draw_fline(
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[2].x,
                    self.size_y as f32 - triangle.vectors[2].y,
                ),
                FPoint::new(
                    self.size_x as f32 - triangle.vectors[0].x,
                    self.size_y as f32 - triangle.vectors[0].y,
                ),
            )
            .expect("Error drawing line");
    }

    pub fn move_camera(&mut self, key: Keycode, elapsed_time: f32) {
        let vector_forward = &self.look_direction * (8.0 * elapsed_time);
        match key {
            Keycode::UP => self.camera.y += 8.0 * elapsed_time,
            Keycode::DOWN => self.camera.y -= 8.0 * elapsed_time,
            Keycode::LEFT => self.r_yaw -= 8.0 * elapsed_time,
            Keycode::RIGHT => self.r_yaw += 8.0 * elapsed_time,
            Keycode::W => {
                self.camera = &self.camera + &vector_forward;
            }
            Keycode::S => {
                self.camera = &self.camera - &vector_forward;
            }
            Keycode::D => {
                let target_vector = Vector3D::from_coords(1.0, 0.0, 0.0);
                let camera_rotation_matrix = Matrix4X4::from_rotation_y(self.r_yaw);
        
                let right_from_look_direction_direction = &camera_rotation_matrix * &target_vector;

                self.camera = &self.camera - &(&right_from_look_direction_direction * (8.0 * elapsed_time));
            }
            Keycode::A => {
                let target_vector = Vector3D::from_coords(-1.0, 0.0, 0.0);
                let camera_rotation_matrix = Matrix4X4::from_rotation_y(self.r_yaw);
        
                let left_from_look_direction_direction = &camera_rotation_matrix * &target_vector;

                self.camera = &self.camera - &(&left_from_look_direction_direction * (8.0 * elapsed_time));
            }
            _ => {}
        }
    }

    pub fn resize_window(&mut self, new_x: i32, new_y: i32) {
        self.size_x = new_x as u32;
        self.size_y = new_y as u32;
    }

    pub fn set_title(&mut self, new_title: String) -> Result<(), NulError> {
        self.canvas.window_mut().set_title(&new_title)
    }
}
