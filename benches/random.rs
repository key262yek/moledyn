// Benchmarks for random number generator

use rts::random_mod::{rng_seed, get_uniform, get_gaussian};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn bench_uniform(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (1.5B iterations)
    // 3.3585 ns 3.3640 ns 3.3714 ns
    // C : 36ns/iter
    let mut rng = rng_seed(3123412314);
    c.bench_function("uniform", |b|  b.iter(|| get_uniform(black_box(&mut rng))));
}


fn bench_gaussian(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (739M iterations)
    // 6.5053 ns 6.5115 ns 6.5193 ns
    // C : 64ns/iter
    let mut rng = rng_seed(3123412314);
    c.bench_function("gaussian", |b| b.iter(|| get_gaussian(black_box(&mut rng))));
}

criterion_group!(benches, bench_uniform, bench_gaussian);
criterion_main!(benches);
