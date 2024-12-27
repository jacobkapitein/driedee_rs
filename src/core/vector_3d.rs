pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new() -> Vector3D {
        Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Clone for Vector3D {
    fn clone(&self) -> Vector3D {
        Vector3D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}
