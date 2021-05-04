// Benchmarks for random number generator

use rts::random_mod::{rng_seed, get_uniform};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn bench_uniform(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (1.5B iterations)
    // 3.3585 ns 3.3640 ns 3.3714 ns
    // 같은 기능의 기존 C 코드 : 36ns/iter
    let mut rng = rng_seed(3123412314);
    c.bench_function("uniform", |b|  b.iter(|| get_uniform(black_box(&mut rng))));
}


fn bench_log10(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (484M iteration
    // 10.268 ns 10.289 ns 10.313 ns
    let mut rng = rng_seed(3123412314);
    c.bench_function("log10", |b|  b.iter(|| get_uniform(black_box(&mut rng)).log10()));
}


fn bench_ln(c : &mut Criterion){
    // 10.457 ns 10.492 ns 10.537 ns

    let mut rng = rng_seed(3123412314);
    c.bench_function("ln", |b|  b.iter(|| get_uniform(black_box(&mut rng)).ln()));
}


fn bench_log2(c : &mut Criterion){
    // 10.510 ns 10.521 ns 10.533 ns

    let mut rng = rng_seed(3123412314);
    c.bench_function("log2", |b|  b.iter(|| get_uniform(black_box(&mut rng)).log2()));
}

criterion_group!(benches, bench_uniform, bench_log10, bench_ln, bench_log2);
criterion_main!(benches);
