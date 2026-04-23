//! Property-based tests using proptest
//!
//! Verifies mathematical invariants and roundtrip conversions that should hold
//! for arbitrary inputs, catching edge cases that hand-written tests miss.

use proptest::prelude::*;

// ── Color roundtrip properties ──────────────────────────────────────────────

mod color_roundtrips {
    use super::*;
    use pixestl::color::{CieLab, Hsl, Rgb};

    proptest! {
        /// RGB → hex → RGB roundtrip must be lossless (exact for u8 values).
        #[test]
        fn rgb_hex_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let original = Rgb::new(r, g, b);
            let hex = original.to_hex();
            let restored = Rgb::from_hex(&hex).unwrap();
            prop_assert_eq!(original, restored);
        }

        /// RGB → CMYK → RGB roundtrip is lossy due to float math, but within ±1.
        #[test]
        fn rgb_cmyk_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let original = Rgb::new(r, g, b);
            let cmyk = original.to_cmyk();
            let restored = Rgb::from_cmyk(cmyk);
            prop_assert!((original.r as i16 - restored.r as i16).abs() <= 1,
                "r: {} vs {}", original.r, restored.r);
            prop_assert!((original.g as i16 - restored.g as i16).abs() <= 1,
                "g: {} vs {}", original.g, restored.g);
            prop_assert!((original.b as i16 - restored.b as i16).abs() <= 1,
                "b: {} vs {}", original.b, restored.b);
        }

        /// RGB → HSL → RGB roundtrip within tolerance ±2 (float rounding).
        #[test]
        fn rgb_hsl_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let original = Rgb::new(r, g, b);
            let hsl: Hsl = original.into();
            let restored = hsl.to_rgb();
            prop_assert!((original.r as i16 - restored.r as i16).abs() <= 2,
                "r: {} vs {} (hsl: {:?})", original.r, restored.r, hsl);
            prop_assert!((original.g as i16 - restored.g as i16).abs() <= 2,
                "g: {} vs {} (hsl: {:?})", original.g, restored.g, hsl);
            prop_assert!((original.b as i16 - restored.b as i16).abs() <= 2,
                "b: {} vs {} (hsl: {:?})", original.b, restored.b, hsl);
        }

        /// RGB → CIELab: L should be in [0, 100], approximately.
        #[test]
        fn rgb_to_cielab_l_range(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let rgb = Rgb::new(r, g, b);
            let lab: CieLab = rgb.into();
            prop_assert!(lab.l >= -0.01 && lab.l <= 100.01,
                "L* out of range: {}", lab.l);
        }
    }
}

// ── Color distance properties ───────────────────────────────────────────────

mod color_distance_properties {
    use super::*;
    use pixestl::color::{CieLab, ColorDistance as _, Rgb};

    proptest! {
        /// Distance is symmetric: d(a,b) == d(b,a) for RGB Euclidean distance.
        #[test]
        fn rgb_distance_symmetric(
            r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
            r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
        ) {
            let a = Rgb::new(r1, g1, b1);
            let b = Rgb::new(r2, g2, b2);
            let d_ab = a.distance(&b);
            let d_ba = b.distance(&a);
            prop_assert!((d_ab - d_ba).abs() < 1e-10);
        }

        /// Distance to self is always 0.
        #[test]
        fn rgb_distance_identity(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let color = Rgb::new(r, g, b);
            prop_assert!((color.distance(&color) - 0.0).abs() < 1e-10);
        }

        /// CIELab Delta E is symmetric.
        #[test]
        fn cielab_delta_e_symmetric(
            r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
            r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
        ) {
            let lab_a: CieLab = Rgb::new(r1, g1, b1).into();
            let lab_b: CieLab = Rgb::new(r2, g2, b2).into();
            let d_ab = lab_a.delta_e(&lab_b);
            let d_ba = lab_b.delta_e(&lab_a);
            prop_assert!((d_ab - d_ba).abs() < 1e-10,
                "asymmetric: {} vs {}", d_ab, d_ba);
        }

        /// CIELab Delta E to self is 0.
        #[test]
        fn cielab_delta_e_identity(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
            let lab: CieLab = Rgb::new(r, g, b).into();
            let d = lab.delta_e(&lab);
            prop_assert!((d - 0.0).abs() < 1e-10, "self distance should be 0, got {}", d);
        }

        /// CIELab Delta E is non-negative.
        #[test]
        fn cielab_delta_e_non_negative(
            r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
            r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
        ) {
            let lab_a: CieLab = Rgb::new(r1, g1, b1).into();
            let lab_b: CieLab = Rgb::new(r2, g2, b2).into();
            let d = lab_a.delta_e(&lab_b);
            prop_assert!(d >= 0.0, "distance should be >= 0, got {}", d);
        }
    }
}

// ── Geometry properties ─────────────────────────────────────────────────────

mod geometry_properties {
    use super::*;
    use pixestl::lithophane::{Mesh, Vector3};

    proptest! {
        /// Normalized non-zero vector has length ≈ 1.0.
        #[test]
        fn normalize_produces_unit_length(
            x in -1000.0f64..1000.0,
            y in -1000.0f64..1000.0,
            z in -1000.0f64..1000.0,
        ) {
            let v = Vector3::new(x, y, z);
            if v.length() > 1e-10 {
                let n = v.normalize();
                prop_assert!((n.length() - 1.0).abs() < 1e-6,
                    "normalized length should be ~1.0, got {}", n.length());
            }
        }

        /// Dot product is commutative: a·b == b·a.
        #[test]
        fn dot_product_commutative(
            x1 in -100.0f64..100.0, y1 in -100.0f64..100.0, z1 in -100.0f64..100.0,
            x2 in -100.0f64..100.0, y2 in -100.0f64..100.0, z2 in -100.0f64..100.0,
        ) {
            let a = Vector3::new(x1, y1, z1);
            let b = Vector3::new(x2, y2, z2);
            let ab = a.dot(&b);
            let ba = b.dot(&a);
            prop_assert!((ab - ba).abs() < 1e-10, "dot not commutative: {} vs {}", ab, ba);
        }

        /// Cross product is anti-commutative: a×b == -(b×a).
        #[test]
        fn cross_product_anti_commutative(
            x1 in -100.0f64..100.0, y1 in -100.0f64..100.0, z1 in -100.0f64..100.0,
            x2 in -100.0f64..100.0, y2 in -100.0f64..100.0, z2 in -100.0f64..100.0,
        ) {
            let a = Vector3::new(x1, y1, z1);
            let b = Vector3::new(x2, y2, z2);
            let ab = a.cross(&b);
            let ba = b.cross(&a);
            prop_assert!((ab.x + ba.x).abs() < 1e-6, "x: {} vs {}", ab.x, -ba.x);
            prop_assert!((ab.y + ba.y).abs() < 1e-6, "y: {} vs {}", ab.y, -ba.y);
            prop_assert!((ab.z + ba.z).abs() < 1e-6, "z: {} vs {}", ab.z, -ba.z);
        }

        /// Merging two meshes: triangle count = sum of individual counts.
        #[test]
        fn mesh_merge_preserves_triangle_count(n1 in 0usize..10, n2 in 0usize..10) {
            let mut mesh1 = Mesh::new();
            for _ in 0..n1 {
                mesh1.add_triangle(pixestl::lithophane::Triangle::new(
                    Vector3::zero(), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0),
                ));
            }
            let mut mesh2 = Mesh::new();
            for _ in 0..n2 {
                mesh2.add_triangle(pixestl::lithophane::Triangle::new(
                    Vector3::zero(), Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0),
                ));
            }
            mesh1.merge(&mesh2);
            prop_assert_eq!(mesh1.triangle_count(), n1 + n2);
        }
    }
}

// ── Config validation properties ────────────────────────────────────────────

mod config_properties {
    use super::*;
    use pixestl::lithophane::LithophaneConfig;

    proptest! {
        /// Any config with non-positive color_pixel_width must fail validation.
        #[test]
        fn invalid_color_pixel_width_fails(w in -100.0f64..=0.0) {
            let config = LithophaneConfig {
                color_pixel_width: w,
                ..LithophaneConfig::default()
            };
            prop_assert!(config.validate().is_err());
        }

        /// Any config with texture_max <= texture_min must fail validation.
        #[test]
        fn invalid_texture_range_fails(
            min_val in 0.1f64..10.0,
            offset in -10.0f64..=0.0,
        ) {
            let config = LithophaneConfig {
                texture_min_thickness: min_val,
                texture_max_thickness: min_val + offset,
                ..LithophaneConfig::default()
            };
            prop_assert!(config.validate().is_err());
        }

        /// Any config with curve outside [0, 360] must fail validation.
        #[test]
        fn invalid_curve_fails(curve in prop::num::f64::ANY.prop_filter("outside 0..360", |c| *c < 0.0 || *c > 360.0)) {
            if curve.is_finite() {
                let config = LithophaneConfig {
                    curve,
                    ..LithophaneConfig::default()
                };
                prop_assert!(config.validate().is_err());
            }
        }
    }
}
