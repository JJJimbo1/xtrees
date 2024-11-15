#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::random;
use xtrees::*;

fn map_from_01(value: f32, min: f32, max: f32) -> f32 {
    value * (min.max(max) - min.min(max)) + min
}

fn insert(n: usize) {
    let mut qt = QuadTree::new(Quad::new(0.0, 0.0, 500.0, 500.0));
    for i in 0..n {
        let x = map_from_01(random(), -500.0, 500.0);
        let y = map_from_01(random(), -500.0, 500.0);
        qt.insert(i, Quad::new(x, y, 0.5, 0.5));
    }
}

fn search(quadtree: &QuadTree<usize>) {
    let x = map_from_01(random(), -500.0, 500.0);
    let y = map_from_01(random(), -500.0, 500.0);
    quadtree.search(&Quad::new(x, y, 50.0, 50.0));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert", |b| b.iter(|| insert(1000)));

    let mut quadtree = QuadTree::new(Quad::new(0.0, 0.0, 500.0, 500.0));
    for i in 0..1000 {
        let x = map_from_01(random(), -500.0, 500.0);
        let y = map_from_01(random(), -500.0, 500.0);
        quadtree.insert(i, Quad::new(x, y, 0.5, 0.5));
    }

    c.bench_function("search", |b| b.iter(|| search(&quadtree)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
