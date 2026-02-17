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
            triangles: self.triangles.iter().map(|t| t.translate(offset)).collect(),
        }
    }

    /// Merges another mesh into this one (by reference, cloning triangles)
    pub fn merge(&mut self, other: &Mesh) {
        self.triangles.extend(other.triangles.iter().cloned());
    }

    /// Merges another mesh into this one by consuming it (no cloning)
    pub fn merge_owned(&mut self, other: Mesh) {
        self.triangles.extend(other.triangles);
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

impl Mesh {
    /// Applies a cylindrical curve transformation to the mesh.
    ///
    /// Wraps the mesh around a cylinder along the X axis.
    /// - `curve_degrees`: Arc angle in degrees (0=flat, 90=quarter, 360=full cylinder)
    /// - `total_width`: Total width of the flat mesh in mm (the arc length)
    ///
    /// The Y axis remains unchanged (cylinder axis).
    /// The X axis maps to the angular position.
    /// The Z axis (depth/thickness) becomes the radial offset from the cylinder surface.
    pub fn apply_curve(&mut self, curve_degrees: f64, total_width: f64) {
        if curve_degrees == 0.0 || total_width <= 0.0 {
            return;
        }

        let curve_radians = curve_degrees.to_radians();
        let radius = total_width / curve_radians;

        for triangle in &mut self.triangles {
            for vertex in [&mut triangle.v0, &mut triangle.v1, &mut triangle.v2] {
                let angle = (vertex.x / total_width) * curve_radians;
                let r = radius + vertex.z;
                vertex.x = r * angle.sin();
                vertex.z = r * angle.cos() - radius;
            }
        }
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

    // --- apply_curve tests ---

    #[test]
    fn test_apply_curve_zero_degrees_no_change() {
        let mut mesh = Mesh::cube(10.0, 10.0, 1.0, Vector3::new(5.0, 5.0, 0.5));
        let original = mesh.clone();
        mesh.apply_curve(0.0, 100.0);

        // Should be unchanged
        for (t_orig, t_curved) in original.triangles.iter().zip(mesh.triangles.iter()) {
            assert_eq!(t_orig.v0, t_curved.v0);
            assert_eq!(t_orig.v1, t_curved.v1);
            assert_eq!(t_orig.v2, t_curved.v2);
        }
    }

    #[test]
    fn test_apply_curve_preserves_triangle_count() {
        let mut mesh = Mesh::cube(10.0, 10.0, 1.0, Vector3::new(5.0, 5.0, 0.5));
        let count_before = mesh.triangle_count();
        mesh.apply_curve(90.0, 100.0);
        assert_eq!(mesh.triangle_count(), count_before);
    }

    #[test]
    fn test_apply_curve_y_unchanged() {
        // Y axis (cylinder axis) should not be affected by curve
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 5.0, 0.0),
            Vector3::new(10.0, 10.0, 0.0),
            Vector3::new(10.0, 15.0, 0.0),
        ));
        let original_ys: Vec<f64> = vec![
            mesh.triangles[0].v0.y,
            mesh.triangles[0].v1.y,
            mesh.triangles[0].v2.y,
        ];

        mesh.apply_curve(180.0, 100.0);

        assert_relative_eq!(mesh.triangles[0].v0.y, original_ys[0], epsilon = 1e-10);
        assert_relative_eq!(mesh.triangles[0].v1.y, original_ys[1], epsilon = 1e-10);
        assert_relative_eq!(mesh.triangles[0].v2.y, original_ys[2], epsilon = 1e-10);
    }

    #[test]
    fn test_apply_curve_origin_vertex_unchanged_z() {
        // A vertex at x=0, z=some_value should keep its z after curve
        // because angle=0 → sin(0)=0, cos(0)=1 → new_z = (r+z)*1 - r = z
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 1.5),
            Vector3::new(0.0, 1.0, 1.5),
            Vector3::new(0.0, 2.0, 1.5),
        ));

        mesh.apply_curve(180.0, 100.0);

        // At x=0: angle=0, new_x = r*sin(0) = 0, new_z = (r+z)*cos(0) - r = z
        assert_relative_eq!(mesh.triangles[0].v0.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(mesh.triangles[0].v0.z, 1.5, epsilon = 1e-10);
    }

    #[test]
    fn test_apply_curve_360_degrees_forms_cylinder() {
        // For 360° curve, the vertex at x=total_width should map back near x=0
        // (full circle). angle = 2π, sin(2π) ≈ 0, cos(2π) ≈ 1
        let total_width = 100.0;
        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(total_width, 0.0, 0.0),
            Vector3::new(total_width / 2.0, 1.0, 0.0),
        ));

        mesh.apply_curve(360.0, total_width);

        // v0 at x=0: should stay near origin
        assert_relative_eq!(mesh.triangles[0].v0.x, 0.0, epsilon = 1e-6);
        // v1 at x=total_width (360°): should come back near origin
        assert_relative_eq!(mesh.triangles[0].v1.x, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_apply_curve_90_degrees_quarter_cylinder() {
        // At 90°, vertex at x=total_width should be at angle=π/2
        let total_width = 100.0;
        let radius = total_width / (90.0_f64.to_radians()); // r = 100 / (π/2) ≈ 63.66

        let mut mesh = Mesh::new();
        mesh.add_triangle(Triangle::new(
            Vector3::new(total_width, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ));

        mesh.apply_curve(90.0, total_width);

        // v0 at x=total_width: angle=π/2
        // new_x = r * sin(π/2) = r ≈ 63.66
        // new_z = r * cos(π/2) - r = 0 - r ≈ -63.66
        assert_relative_eq!(mesh.triangles[0].v0.x, radius, epsilon = 0.01);
        assert_relative_eq!(mesh.triangles[0].v0.z, -radius, epsilon = 0.01);
    }

    #[test]
    fn test_apply_curve_negative_width_no_change() {
        let mut mesh = Mesh::cube(10.0, 10.0, 1.0, Vector3::new(5.0, 5.0, 0.5));
        let original = mesh.clone();
        mesh.apply_curve(90.0, -1.0); // Invalid width, should skip

        for (t_orig, t_curved) in original.triangles.iter().zip(mesh.triangles.iter()) {
            assert_eq!(t_orig.v0, t_curved.v0);
        }
    }
}
