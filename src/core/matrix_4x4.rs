use super::{
    math::{vector_cross_product, vector_dot_product},
    vector_3d::Vector3D,
};

pub struct Matrix4X4 {
    pub content: [[f32; 4]; 4],
}

impl Matrix4X4 {
    pub fn new() -> Self {
        Self {
            content: [[0.0; 4]; 4],
        }
    }

    pub fn from_identity() -> Self {
        let mut matrix = Self::new();

        matrix.content[0][0] = 1.0;
        matrix.content[1][1] = 1.0;
        matrix.content[2][2] = 1.0;
        matrix.content[3][3] = 1.0;

        matrix
    }

    pub fn from_rotation_x(angle_rads: f32) -> Self {
        let mut matrix = Self::new();

        matrix.content[0][0] = 1.0;
        matrix.content[1][1] = angle_rads.cos();
        matrix.content[1][2] = angle_rads.sin();
        matrix.content[2][1] = -angle_rads.sin();
        matrix.content[2][2] = angle_rads.cos();
        matrix.content[3][3] = 1.0;

        matrix
    }

    pub fn from_rotation_y(angle_rads: f32) -> Self {
        let mut matrix = Self::new();

        matrix.content[0][0] = angle_rads.cos();
        matrix.content[0][2] = angle_rads.sin();
        matrix.content[2][0] = -angle_rads.sin();
        matrix.content[1][1] = 1.0;
        matrix.content[2][2] = angle_rads.cos();
        matrix.content[3][3] = 1.0;

        matrix
    }

    pub fn from_rotation_z(angle_rads: f32) -> Self {
        let mut matrix = Self::new();

        matrix.content[0][0] = angle_rads.cos();
        matrix.content[0][1] = angle_rads.sin();
        matrix.content[1][0] = -angle_rads.sin();
        matrix.content[1][1] = angle_rads.cos();
        matrix.content[2][2] = 1.0;
        matrix.content[3][3] = 1.0;

        matrix
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::from_identity();

        matrix.content[3][0] = x;
        matrix.content[3][1] = y;
        matrix.content[3][2] = z;

        matrix
    }

    pub fn from_projection(
        fov_degrees: f32,
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        let mut matrix = Matrix4X4::new();
        let fov_rad = 1.0 / (fov_degrees * 0.5 / 180.0 * 3.14159).tan();

        matrix.content[0][0] = aspect_ratio * fov_rad;
        matrix.content[1][1] = fov_rad;
        matrix.content[2][2] = far_plane / (far_plane - near_plane);
        matrix.content[3][2] = (-far_plane * near_plane) / (far_plane - near_plane);
        matrix.content[2][3] = 1.0;
        matrix.content[3][3] = 0.0;

        matrix
    }

    pub fn from_point_at(position: &Vector3D, target: &Vector3D, up: &Vector3D) -> Self {
        let new_forward = (target - position).from_normalise();
        let new_up = (up - &(&new_forward * vector_dot_product(up, &new_forward))).from_normalise();
        let new_right = vector_cross_product(&new_up, &new_forward);

        let mut matrix = Matrix4X4::new();
        matrix.content[0][0] = new_right.x;
        matrix.content[0][1] = new_right.y;
        matrix.content[0][2] = new_right.z;
        matrix.content[0][3] = 0.0;
        matrix.content[1][0] = new_up.x;
        matrix.content[1][1] = new_up.y;
        matrix.content[1][2] = new_up.z;
        matrix.content[1][3] = 0.0;
        matrix.content[2][0] = new_forward.x;
        matrix.content[2][1] = new_forward.y;
        matrix.content[2][2] = new_forward.z;
        matrix.content[2][3] = 0.0;
        matrix.content[3][0] = position.x;
        matrix.content[3][1] = position.y;
        matrix.content[3][2] = position.z;
        matrix.content[3][3] = 1.0;
        matrix
    }

    pub fn quick_inverse(&self) -> Self {
        let mut matrix = Matrix4X4::new();

        matrix.content[0][0] = self.content[0][0];
        matrix.content[0][1] = self.content[1][0];
        matrix.content[0][2] = self.content[2][0];
        matrix.content[0][3] = 0.0;
        matrix.content[1][0] = self.content[0][1];
        matrix.content[1][1] = self.content[1][1];
        matrix.content[1][2] = self.content[2][1];
        matrix.content[1][3] = 0.0;
        matrix.content[2][0] = self.content[0][2];
        matrix.content[2][1] = self.content[1][2];
        matrix.content[2][2] = self.content[2][2];
        matrix.content[2][3] = 0.0;
        matrix.content[3][0] = -(self.content[3][0] * matrix.content[0][0]
            + self.content[3][1] * matrix.content[1][0]
            + self.content[3][2] * matrix.content[2][0]);
        matrix.content[3][1] = -(self.content[3][0] * matrix.content[0][1]
            + self.content[3][1] * matrix.content[1][1]
            + self.content[3][2] * matrix.content[2][1]);
        matrix.content[3][2] = -(self.content[3][0] * matrix.content[0][2]
            + self.content[3][1] * matrix.content[1][2]
            + self.content[3][2] * matrix.content[2][2]);
        matrix.content[3][3] = 1.0;

        matrix
    }
}

impl std::ops::Mul for &Matrix4X4 {
    type Output = Matrix4X4;

    fn mul(self, matrix: Self) -> Self::Output {
        let mut new_matrix = Matrix4X4::new();

        for col in 0..matrix.content.len() {
            for row in 0..matrix.content[col].len() {
                new_matrix.content[row][col] = self.content[row][0] * matrix.content[0][col]
                    + self.content[row][1] * matrix.content[1][col]
                    + self.content[row][2] * matrix.content[2][col]
                    + self.content[row][3] * matrix.content[3][col];
            }
        }

        new_matrix
    }
}

impl std::ops::Mul<&Vector3D> for &Matrix4X4 {
    type Output = Vector3D;

    fn mul(self, vector: &Vector3D) -> Self::Output {
        let mut new_vector = Vector3D::new();

        new_vector.x = vector.x * self.content[0][0]
            + vector.y * self.content[1][0]
            + vector.z * self.content[2][0]
            + vector.w * self.content[3][0];
        new_vector.y = vector.x * self.content[0][1]
            + vector.y * self.content[1][1]
            + vector.z * self.content[2][1]
            + vector.w * self.content[3][1];
        new_vector.z = vector.x * self.content[0][2]
            + vector.y * self.content[1][2]
            + vector.z * self.content[2][2]
            + vector.w * self.content[3][2];
        new_vector.w = vector.x * self.content[0][3]
            + vector.y * self.content[1][3]
            + vector.z * self.content[2][3]
            + vector.w * self.content[3][3];

        new_vector
    }
}
