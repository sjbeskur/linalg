use super::vector3::Vector3;
pub struct Quaternion{

    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion{

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self{
        Self{x, y, z, w}
    }

    pub fn vector_part(&self) -> Vector3{
        Vector3::new(self.x, self.y, self.z)
    }



}