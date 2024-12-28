use sdl2::pixels::Color;

use super::vector_3d::Vector3D;

pub struct Triangle {
    pub vector3d: [Vector3D; 3],
    pub base_color: Color,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vector3d: [Vector3D::new(), Vector3D::new(), Vector3D::new()],
            base_color: Color::RGB(255, 255, 255),
        }
    }

    pub fn new_from_vectors(vectors: [Vector3D; 3]) -> Triangle {
        Triangle {
            vector3d: vectors,
            base_color: Color::RGB(255, 255, 255),
        }
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        Triangle {
            vector3d: self.vector3d.clone(),
            base_color: self.base_color.clone(),
        }
    }
}
