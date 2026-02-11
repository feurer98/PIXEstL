//! 3D geometry primitives for lithophane generation
//!
//! Provides Vector3, Triangle, and Mesh structures for building STL models

use std::ops::{Add, Mul, Sub};

/// 3D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Creates a new 3D vector
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Zero vector
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Computes the cross product with another vector
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Computes the dot product with another vector
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the length (magnitude) of the vector
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalizes the vector to unit length
    pub fn normalize(&self) -> Vector3 {
        let len = self.length();
        if len > 0.0 {
            *self * (1.0 / len)
        } else {
            *self
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// 3D triangle with vertices
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
}

impl Triangle {
    /// Creates a new triangle
    pub fn new(v0: Vector3, v1: Vector3, v2: Vector3) -> Self {
        Self { v0, v1, v2 }
    }

    /// Computes the normal vector for this triangle
    pub fn normal(&self) -> Vector3 {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        edge1.cross(&edge2).normalize()
    }

    /// Translates the triangle by a vector
    pub fn translate(&self, offset: Vector3) -> Triangle {
        Triangle::new(self.v0 + offset, self.v1 + offset, self.v2 + offset)
    }
}

/// 3D mesh composed of triangles
#[derive(Debug, Clone)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Creates a new empty mesh
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    /// Creates a mesh with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            triangles: Vec::with_capacity(capacity),
        }
    }

    /// Adds a triangle to the mesh
    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    /// Adds multiple triangles to the mesh
    pub fn extend(&mut self, triangles: impl IntoIterator<Item = Triangle>) {
        self.triangles.extend(triangles);
    }

    /// Gets the number of triangles in the mesh
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Translates all triangles in the mesh by a vector
    pub fn translate(&self, offset: Vector3) -> Mesh {
        Mesh {
            triangles: self
                .triangles
                .iter()
                .map(|t| t.translate(offset))
                .collect(),
        }
    }

    /// Merges another mesh into this one
    pub fn merge(&mut self, other: &Mesh) {
        self.triangles.extend(other.triangles.iter().cloned());
    }

    /// Creates a cube mesh
    ///
    /// # Arguments
    ///
    /// * `width` - Width (X dimension)
    /// * `depth` - Depth (Y dimension)
    /// * `height` - Height (Z dimension)
    /// * `center` - Center position of the cube
    pub fn cube(width: f64, depth: f64, height: f64, center: Vector3) -> Self {
        let hw = width / 2.0;
        let hd = depth / 2.0;
        let hh = height / 2.0;

        let mut mesh = Mesh::new();

        // Define 8 vertices of the cube (relative to center)
        let v000 = center + Vector3::new(-hw, -hd, -hh);
        let v001 = center + Vector3::new(-hw, -hd, hh);
        let v010 = center + Vector3::new(-hw, hd, -hh);
        let v011 = center + Vector3::new(-hw, hd, hh);
        let v100 = center + Vector3::new(hw, -hd, -hh);
        let v101 = center + Vector3::new(hw, -hd, hh);
        let v110 = center + Vector3::new(hw, hd, -hh);
        let v111 = center + Vector3::new(hw, hd, hh);

        // Front face (+Y)
        mesh.add_triangle(Triangle::new(v010, v110, v111));
        mesh.add_triangle(Triangle::new(v010, v111, v011));

        // Back face (-Y)
        mesh.add_triangle(Triangle::new(v000, v101, v100));
        mesh.add_triangle(Triangle::new(v000, v001, v101));

        // Right face (+X)
        mesh.add_triangle(Triangle::new(v100, v110, v101));
        mesh.add_triangle(Triangle::new(v110, v111, v101));

        // Left face (-X)
        mesh.add_triangle(Triangle::new(v000, v011, v001));
        mesh.add_triangle(Triangle::new(v000, v010, v011));

        // Top face (+Z)
        mesh.add_triangle(Triangle::new(v001, v101, v111));
        mesh.add_triangle(Triangle::new(v001, v111, v011));

        // Bottom face (-Z)
        mesh.add_triangle(Triangle::new(v000, v110, v010));
        mesh.add_triangle(Triangle::new(v000, v100, v110));

        mesh
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_vector3_creation() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_zero() {
        let v = Vector3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(4.0, 5.0, 6.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        let v3 = v1 - v2;
        assert_eq!(v3, Vector3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vector3_mul() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let v2 = v * 2.0;
        assert_eq!(v2, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector3_dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector3_cross() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_vector3_normalize() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert_relative_eq!(normalized.length(), 1.0, epsilon = 1e-10);
        assert_relative_eq!(normalized.x, 0.6, epsilon = 1e-10);
        assert_relative_eq!(normalized.y, 0.8, epsilon = 1e-10);
    }

    #[test]
    fn test_triangle_normal() {
        let t = Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let normal = t.normal();
        // Normal should point in +Z direction
        assert_relative_eq!(normal.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(normal.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(normal.z, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_triangle_translate() {
        let t = Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let offset = Vector3::new(10.0, 20.0, 30.0);
        let translated = t.translate(offset);
        assert_eq!(translated.v0, Vector3::new(10.0, 20.0, 30.0));
        assert_eq!(translated.v1, Vector3::new(11.0, 20.0, 30.0));
        assert_eq!(translated.v2, Vector3::new(10.0, 21.0, 30.0));
    }

    #[test]
    fn test_mesh_creation() {
        let mesh = Mesh::new();
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_add_triangle() {
        let mut mesh = Mesh::new();
        let t = Triangle::new(
            Vector3::zero(),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        mesh.add_triangle(t);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_mesh_cube() {
        let mesh = Mesh::cube(2.0, 2.0, 2.0, Vector3::zero());
        assert_eq!(mesh.triangle_count(), 12); // 6 faces * 2 triangles per face
    }

    #[test]
    fn test_mesh_translate() {
        let mesh = Mesh::cube(2.0, 2.0, 2.0, Vector3::zero());
        let offset = Vector3::new(10.0, 20.0, 30.0);
        let translated = mesh.translate(offset);
        assert_eq!(translated.triangle_count(), mesh.triangle_count());
    }

    #[test]
    fn test_mesh_merge() {
        let mut mesh1 = Mesh::cube(1.0, 1.0, 1.0, Vector3::zero());
        let mesh2 = Mesh::cube(1.0, 1.0, 1.0, Vector3::new(2.0, 0.0, 0.0));
        mesh1.merge(&mesh2);
        assert_eq!(mesh1.triangle_count(), 24); // 12 + 12
    }
}
