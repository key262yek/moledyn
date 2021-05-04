// Benchmarks for reflective boundary condition

use rts::position::{Position, Numerics};
use criterion::{black_box, criterion_group, criterion_main, Criterion};



fn bench_use_zip(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (1.9B iterations)
    // 2.6020 ns 2.6077 ns 2.6144 ns
    let mut pos : Position<f64> = Position::new(vec![0.99, 0.0]);
    let dp : Position<f64> = Position::new(vec![0.1, 0.1]);

    c.bench_function("use_zip", |b|  b.iter(|| pos.mut_add_bench(black_box(&dp))));
}


fn bench_use_range(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (2.1B iterations)
    // 2.3245 ns 2.3269 ns 2.3296 ns
    let mut pos : Position<f64> = Position::new(vec![0.99, 0.0]);
    let dp : Position<f64> = Position::new(vec![0.1, 0.1]);

    c.bench_function("use_range", |b|  b.iter(|| pos.mut_add(black_box(&dp))));
}

criterion_group!(benches, bench_use_zip, bench_use_range);
criterion_main!(benches);
