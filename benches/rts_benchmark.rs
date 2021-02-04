// Single ptl random target search system을 기준으로 C 코드와 rust 코드 속도 비교
// time for 100 ensemble : 1.32904e+00s/iter

use criterion::{criterion_group, criterion_main, Criterion};
use rand_pcg::Pcg64;
use rts::random_mod::rng_seed;
use rts::system_mod::{SystemCore, cont_circ::ContCircSystem};
use rts::target_mod::{TargetCore, cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, types::MoveType, cont_passive_indep::ContPassiveIndepSearcher};
use rts::position::Position;
use rts::error::Error;
use std::default::Default;

fn single_ptl_fpt() -> Result<(), Error>{
    // 1 ensemble 당 시간 49.662ms
    // C code : 13.29ms/ensemble
    // C code가 대략 4배 빠르다.

    // malloc을 최소화하도록, 대부분 mutable reference를 주고받도록 함.
    // 200ensemble 당 시간 : 8.6s
    // 1ensemble 당 시간 : 43ms
    // 전혀 나아진게 없네?

    // position.distance 함수에 malloc이 계속 있었음. 이게 굉장히 오랜 시간을 잡아먹으니 이걸 없앰
    // 200 ensemble 3.6s까지 감소.

    // get_**_vec 함수도 항상 malloc이 있음.
    // 이걸 모두 mutable reference에 덮어씌우는 형태로 바꿈.
    // 200 ensemble당 435ms 까지 떨어짐!
    // 1ensemble 당 2ms!

    const NUM_ENSEMBLE : usize = 10000;                       // Number of ensemble
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dt : f64 = 1e-3;                                    // Time step
    let dim : usize = 2;                                    // dimension of system
    let mut data : f64 = 0f64;                              // variable for computing average fpt

    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator
    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target
    let mut single_move : Position<f64> = Position::new(vec![0.0, 0.0]);

    for _i in 0..NUM_ENSEMBLE{
        // Searcher initially located by uniform distribution
        let mut searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, MoveType::Brownian(1f64))?;
        let mut time : f64 = 0f64;

        while !target.check_find(&searcher.pos)?{
            single_move.clear();
            searcher.random_move_to_vec(&mut rng, dt, &mut single_move)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            time += dt;
        }
        data += time;
    }

    let _mfpt : f64 = data / NUM_ENSEMBLE as f64;
    Ok(())
}

fn bench_rts(c : &mut Criterion){
    c.bench_function("rts_rust", |b|  b.iter(|| single_ptl_fpt()));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);
