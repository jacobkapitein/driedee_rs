use std::str::FromStr;

use super::math::vector_len;

pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector3D {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn from_normalise(&self) -> Self {
        let length = vector_len(&self);
        Self::from_coords(self.x / length, self.y / length, self.z / length)
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
        Ok(Self::from_coords(x, y, z))
    }
}

impl std::ops::Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_coords(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_coords(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Div<f32> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from_coords(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Mul<f32> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_coords(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Clone for Vector3D {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            w: self.w.clone(),
        }
    }
}
