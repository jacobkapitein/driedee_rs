use super::vector_3d::Vector3D;

pub struct Triangle {
    pub vector3d: [Vector3D; 3],
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vector3d: [Vector3D::new(), Vector3D::new(), Vector3D::new()],
        }
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            vector3d: self.vector3d.clone(),
        }
    }
}
