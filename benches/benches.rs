#[macro_use]
extern crate criterion;

use criterion::Criterion;
use mathfu::D1;
use simple_random::*;
use rand::random;
use xtrees::*;

fn run(n : usize) {
    // let mut rand = Random::<WichmannHill>::seeded(1421.234);
    // let mut v = Vec::with_capacity(n);
    let mut qt = QuadTree::<usize>::new_tree(TNode::new(Quad::new(0.0, 0.0, 1000.0, 1000.0), 17, 12));
    for i in 0..n {
        // rand.range(-1000.0, 1000.0);
        // rand.range(-1000.0, 1000.0);
        let x = D1::normalize_from_01(random(), -1000.0, 1000.0);
        let y = D1::normalize_from_01(random(), -1000.0, 1000.0);
        qt.insert(i, Quad::new(x, y, 10.5, 10.5));
    }
    // let v = qt.search(&Quad::new(110.0, 120.0, 1000.0, 800.0));
    // if random::<f32>() > 1.0 {
    //     println!("{}", v.len());
    // }
}



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run", |b| b.iter(|| run(1000)));

    // c.bench_function("raw_iter", |b| b.iter(|| raw_iter(32)));
    // c.bench_function("raw_iter_mul", |b| b.iter(|| raw_iter_mul(32)));

    // c.bench_function("vec_index_math", |b| b.iter(|| vec_2d(100)));
    // c.bench_function("vec_index_math", |b| b.iter(|| vec_index_math(100)));
    // c.bench_function("vec_index_no_math", |b| b.iter(|| vec_index_no_math(100)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);