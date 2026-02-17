// Benchmark suite for PIXEstL

use criterion::{criterion_group, criterion_main, Criterion};
use pixestl::color::{find_closest_color, CieLab, ColorDistance, ColorDistanceMethod, Rgb};
use pixestl::lithophane::{Mesh, Vector3};
use pixestl::palette::quantize_pixels;

fn bench_cielab_conversion(c: &mut Criterion) {
    let colors: Vec<Rgb> = (0..256)
        .map(|i| Rgb::new(i as u8, (i / 2) as u8, (i / 3) as u8))
        .collect();

    c.bench_function("cielab_conversion_256", |b| {
        b.iter(|| colors.iter().map(|c| CieLab::from(*c)).collect::<Vec<_>>())
    });
}

fn bench_delta_e(c: &mut Criterion) {
    let lab1 = CieLab::from(Rgb::new(255, 0, 0));
    let lab2 = CieLab::from(Rgb::new(0, 255, 0));

    c.bench_function("delta_e", |b| b.iter(|| lab1.distance(&lab2)));
}

fn bench_find_closest_color(c: &mut Criterion) {
    let palette: Vec<Rgb> = (0..100)
        .map(|i| {
            Rgb::new(
                ((i * 37) % 256) as u8,
                ((i * 73) % 256) as u8,
                ((i * 113) % 256) as u8,
            )
        })
        .collect();
    let target = Rgb::new(128, 128, 128);

    let mut group = c.benchmark_group("find_closest_color");

    group.bench_function("rgb_100_colors", |b| {
        b.iter(|| find_closest_color(&target, &palette, ColorDistanceMethod::Rgb))
    });

    group.bench_function("cielab_100_colors", |b| {
        b.iter(|| find_closest_color(&target, &palette, ColorDistanceMethod::CieLab))
    });

    group.finish();
}

fn bench_quantize_pixels(c: &mut Criterion) {
    let palette: Vec<Rgb> = (0..20)
        .map(|i| {
            Rgb::new(
                ((i * 37) % 256) as u8,
                ((i * 73) % 256) as u8,
                ((i * 113) % 256) as u8,
            )
        })
        .collect();

    let pixels_10k: Vec<Rgb> = (0..10_000)
        .map(|i| Rgb::new((i % 256) as u8, ((i / 256) % 256) as u8, 128))
        .collect();

    let mut group = c.benchmark_group("quantize_pixels");

    group.bench_function("10k_rgb", |b| {
        b.iter(|| quantize_pixels(&pixels_10k, &palette, ColorDistanceMethod::Rgb))
    });

    group.bench_function("10k_cielab", |b| {
        b.iter(|| quantize_pixels(&pixels_10k, &palette, ColorDistanceMethod::CieLab))
    });

    group.finish();
}

fn bench_mesh_cube(c: &mut Criterion) {
    c.bench_function("mesh_cube_1000", |b| {
        b.iter(|| {
            let mut mesh = Mesh::with_capacity(12_000);
            for i in 0..1000 {
                let cube = Mesh::cube(1.0, 1.0, 1.0, Vector3::new(i as f64 * 2.0, 0.0, 0.0));
                mesh.merge_owned(cube);
            }
            mesh
        })
    });
}

fn bench_mesh_merge(c: &mut Criterion) {
    // Create 100 row meshes with ~100 triangles each
    let row_meshes: Vec<Mesh> = (0..100)
        .map(|row| {
            let mut mesh = Mesh::new();
            for col in 0..50 {
                let cube = Mesh::cube(0.8, 0.8, 0.1, Vector3::new(col as f64, row as f64, 0.0));
                mesh.merge_owned(cube);
            }
            mesh
        })
        .collect();

    c.bench_function("mesh_merge_100_rows", |b| {
        b.iter(|| {
            let total: usize = row_meshes.iter().map(|m| m.triangle_count()).sum();
            let mut final_mesh = Mesh::with_capacity(total);
            for row in &row_meshes {
                final_mesh.merge(row);
            }
            final_mesh
        })
    });
}

criterion_group!(
    benches,
    bench_cielab_conversion,
    bench_delta_e,
    bench_find_closest_color,
    bench_quantize_pixels,
    bench_mesh_cube,
    bench_mesh_merge,
);
criterion_main!(benches);
