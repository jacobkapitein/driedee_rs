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
