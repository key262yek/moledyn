// Benchmarks for force computation method

use rts::searcher_mod::types::InteractType;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_enum(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (729M iterations)
    // [6.8366 ns 6.8416 ns 6.8468 ns]
    // Found 10 outliers among 100 measurements (10.00%)
    // 7 (7.00%) high mild
    // 3 (3.00%) high severe

    struct Searcher{
        int_type : InteractType,
    }

    impl Searcher{
        fn force(&self, radius : f64) -> f64{
            match self.int_type{
                InteractType::Exponential(_dim, gamma, strength) =>{
                    strength * (- radius / gamma).exp()
                },
                InteractType::Coulomb(_dim, _strength) =>{
                    0f64
                },
            }
        }
    }

    let int_type = InteractType::Exponential(2, 1.0, 3.0);
    let s = Searcher{int_type};

    c.bench_function("force_from_enum", |b|  b.iter(|| s.force(black_box(3f64))));
}


fn bench_struct(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (717M iterations)
    // [6.8386 ns 6.8485 ns 6.8599 ns]
    // Found 6 outliers among 100 measurements (6.00%)
    // 4 (4.00%) high mild
    // 2 (2.00%) high severe

    #[allow(dead_code)]
    struct Searcher{
        dim : usize,
        gamma : f64,
        strength : f64,
    }

    impl Searcher{
        fn force(&self, radius : f64) -> f64{
            self.strength * (- radius / self.gamma).exp()
        }
    }

    let s = Searcher{dim : 2, gamma : 1.0, strength : 3.0};

    c.bench_function("force_from_struct", |b| b.iter(|| s.force(black_box(3f64))));
}

fn bench_closure(c : &mut Criterion){
    // Collecting 100 samples in estimated 5.0000 s (723M iterations)
    // [6.8857 ns 6.8915 ns 6.8974 ns]
    // Found 4 outliers among 100 measurements (4.00%)
    // 2 (2.00%) low mild
    // 1 (1.00%) high mild
    // 1 (1.00%) high severe

    let (strength, gamma) = (3.0, 1.0);
    let force = |r : f64| -> f64{
        strength * (- r / gamma).exp()
    };

    c.bench_function("force_from_closure", |b| b.iter(|| force(black_box(3f64))));
}

criterion_group!(benches, bench_enum, bench_struct, bench_closure);
criterion_main!(benches);
