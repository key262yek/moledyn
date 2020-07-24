// setup 하는데 걸리는 시간이 얼마나 차이나는지 확인


use criterion::{criterion_group, criterion_main, Criterion};
use rand_pcg::Pcg64;
use rts::random_mod::rng_seed;
use rts::system_mod::cont_circ::ContCircSystem;
use rts::target_mod::cont_bulk::ContBulkTarget;
use rts::searcher_mod::{MoveType, cont_passive_indep::ContPassiveIndepSearcher};
use rts::position::Position;
use rts::error::Error;

fn setup() -> Result<(), Error>{
    // 10000번 setup 하는데 걸리는 시간 16.291ms
    // C : time for 10000 ensemble : 1.10960e+00ms/iter
    // 여기서 15배 차이난다.

    // allocation 문제를 어느정도 해결한 후에는 8.8ms
    // 아직도 8배 차이나는 이유가 뭘까.

    // new_uniform에도 allocation이 들어있다.
    // random_pos를 random_pos_to_vec으로 바꿈.
    // 3.9927ms로 줄어듬. 이제 3배 정도. 구조가 살짝 복잡한걸 감안하면 이정도는 봐줄만한 정도.

    const NUM_ENSEMBLE : usize = 10000;                       // Number of ensemble
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dim : usize = 2;                                    // dimension of system

    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator
    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target

    for _i in 0..NUM_ENSEMBLE{
        // Searcher initially located by uniform distribution
        let mut _searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, MoveType::Brownian(1f64))?;
        let mut _time : f64 = 0f64;
    }

    Ok(())
}

fn bench_setup(c : &mut Criterion){
    c.bench_function("rts_rust", |b|  b.iter(|| setup()));
}

criterion_group!(benches, bench_setup);
criterion_main!(benches);
