#![allow(dead_code)]
use std::ops;

#[derive(Debug)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vector3{
        Self{ x,y,z }
    }

    pub fn zeros() -> Vector3{
        Self{ x: 0.0, y: 0.0, z: 0.0 }
    }

    /// The dot product between two vectors (also known as the inner product) returns
    /// a number and is expressed using the "dot(..)" function.  To be clear the return value
    /// is not a vector but a single scalar value.
    /// 
    /// Algebraically it is represented between brackets like so: (V, W) where V and W are Vector3's. 
    /// 
    /// One method to calculate the dot product is:
    /// dotprod = (Vx * Wx + Vy * Wy + Vz * Wz);      
    /// 
    /// 
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x 
      + self.y * other.y 
      + self.z * other.z
    }

    /// Same as 'magnitude' of a vector.
    /// It is equal to the square root of the vectors dot product with itself
    pub fn length(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    /// Same as 'length' of a vector.
    /// It is equal to the square root of the vectors dot product with itself
    pub fn mag(&self) -> f32 {
        self.length()
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Self{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }   
    }

    pub fn angle_between_r(&self, other: &Vector3) -> f32 {
        let mag1  = self.mag();
        let mag2 = other.mag();
        let r = self.dot(other) / (mag1 * mag2);
        f32::acos(r)
    }

    pub fn angle_between_d(&self, other: &Vector3) -> f32 {
        self.angle_between_r(other) * 180.0 / std::f32::consts::PI
    }

}


impl ops::Add for Vector3{
    type Output = Vector3;
    fn add(self, rhs: Vector3)-> Vector3{
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }                
    }
}

impl ops::Sub for Vector3{
    type Output = Vector3;
    fn sub(self, rhs: Vector3)-> Vector3{
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }                
    }
}

impl PartialEq for Vector3{
    fn eq(&self, other: &Vector3) -> bool{
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}


impl std::fmt::Display for Vector3{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f,"[x: {}, y: {}, z: {}]", self.x, self.y, self.z )
    }    
}
