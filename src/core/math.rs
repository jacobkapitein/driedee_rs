use super::vector_3d::Vector3D;

pub fn interpolate(x_start: f32, y_start: f32, x_end: f32, y_end: f32) -> Vec<f32> {
    let mut result = vec![];

    if (y_start - y_end).abs() < f32::EPSILON {
        result.push(x_start);
        return result;
    }

    let slope = (x_end - x_start) / (y_end - y_start);
    let mut x = x_start;

    if y_end > y_start {
        for _ in (y_start as i32..=y_end as i32).map(|v| v as f32) {
            result.push(x);
            x += slope;
        }
    } else {
        for _ in (y_end as i32..=y_start as i32).rev().map(|v| v as f32) {
            result.push(x);
            x -= slope;
        }
    }

    result
}

pub fn vector_dot_product(v1: &Vector3D, v2: &Vector3D) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn vector_len(v: &Vector3D) -> f32 {
    vector_dot_product(v, v).sqrt()
}

pub fn vector_cross_product(v1: &Vector3D, v2: &Vector3D) -> Vector3D {
    let mut new_vector = Vector3D::new();

    new_vector.x = v1.y * v2.z - v1.z * v2.y;
    new_vector.y = v1.z * v2.x - v1.x * v2.z;
    new_vector.z = v1.x * v2.y - v1.y * v2.x;

    new_vector
}

pub fn intersect_plane(
    plane_position: &Vector3D,
    plane_normal: &Vector3D,
    line_start: &Vector3D,
    line_end: &Vector3D,
) -> Vector3D {
    let plane_n = plane_normal.from_normalise();
    let plane_d: f32 = -vector_dot_product(&plane_n, &plane_position);
    let ad: f32 = vector_dot_product(&line_start, &plane_n);
    let bd: f32 = vector_dot_product(&line_end, &plane_n);
    let t: f32 = (-plane_d - ad) / (bd - ad);
    let line_start_to_end: Vector3D = line_end - line_start;
    let line_to_intersect: Vector3D = &line_start_to_end * t;
    return line_start + &line_to_intersect;
}
