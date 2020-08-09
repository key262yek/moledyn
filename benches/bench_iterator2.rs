// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use criterion::{criterion_group, criterion_main, Criterion};
use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_indep::ContPassiveIndepSearcher};


fn n_ptl_fpt(n : usize, sys: &ContCircSystem, target : &ContBulkTarget, rng : &mut Pcg64) -> Result<f64, Error>{
    // setup 한번에 여러 ensemble 계산하는 코드
    // 1set 당 100ensemble

    // 10ptl : 3.1427 ms 3.1625 ms 3.1848 ms / Performance has regressed with 5000% rate
    // 100ptl : 1.1487 ms 1.1697 ms 1.1936 ms / Performance has impressed with -60% rate
    // 1000ptl : 1.3247 ms 1.5065 ms 1.7027 ms
    // Performance has regressed with +28.793% rate

    // C code
    // 10ptl : 2.66ms per iteration (rust is x1.5 slower)
    // 100ptl : 0.91ms per iteration
    // 1000ptl : 2.43ms per iteartion (x2 faster)

    // 1set 당 10000ensemble
    // C code
    // 10ptl : 2.5ms per iteration (rust is x1.5 slower)
    // 100ptl : 1ms per iteration
    // 1000ptl : 600us per iteartion

    // setup을 제외한 rust performance
    // 10ptl : 1400 iteration, 2.9442 ms 3.1872 ms 3.4413 ms / Performance has improced -97% rate
    // 100ptl : 5050 iteration, 1.2117 ms 1.2680 ms 1.3257 ms / Performance has improved -61% rate
    // 1000ptl : 10k ensemble, 714.30 us 733.79 us 754.98 us / Performance has improved -45% rate
    // 전반적으로 C랑 비슷한 performance를 보여주고 있음. regression은 아무래도 setup이 얼마나 영향을 주느냐에 달린 것 같은데..

    let dt : f64 = 5e-3;                                    // Time step

    let mut vec_searchers : Vec<ContPassiveIndepSearcher> = Vec::with_capacity(n);
    for _i in 0..n{
        let searcher = ContPassiveIndepSearcher::new_uniform(sys, target, rng, MoveType::Brownian(1f64))?;
        vec_searchers.push(searcher);
    }
    let mut list_searchers : LinkedList<ContPassiveIndepSearcher> = LinkedList::from(vec_searchers);
    let mut time : f64 = 0f64;

    'outer : loop{
        list_searchers.into_iter();
        time += dt;
        while let Some(searcher) = list_searchers.get_mut(){
            let mut single_move : Position<f64> = searcher.random_move(rng, dt)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            if target.check_find(&searcher.pos)?{
                break 'outer;
            }
        }
    }

    Ok(time)
}

fn bench_rts(c : &mut Criterion){
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dim : usize = 2;                                    // dimension of system
    let mut rng : Pcg64 = rng_seed(1231412314);             // Random number generator

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target

    c.bench_function("rts_iterator", |b|  b.iter(|| n_ptl_fpt(1000, &sys, &target, &mut rng)));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);