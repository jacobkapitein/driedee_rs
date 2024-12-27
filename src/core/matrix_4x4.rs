pub struct Matrix4X4 {
    pub matrix: [[f32; 4]; 4],
}

impl Matrix4X4 {
    pub fn new() -> Matrix4X4 {
        Matrix4X4 {
            matrix: [[0.0; 4]; 4],
        }
    }
}
