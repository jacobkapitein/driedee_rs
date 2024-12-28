use std::str::FromStr;

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

impl FromStr for Vector3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(format!("Expected 3 parts, found {}", parts.len()));
        }
        let x = parts[0].parse::<f32>().map_err(|_| "Invalid x value")?;
        let y = parts[1].parse::<f32>().map_err(|_| "Invalid y value")?;
        let z = parts[2].parse::<f32>().map_err(|_| "Invalid z value")?;
        Ok(Vector3D { x, y, z })
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
