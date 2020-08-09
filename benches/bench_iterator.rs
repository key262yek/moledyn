// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use criterion::{criterion_group, criterion_main, Criterion};
use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_indep::ContPassiveIndepSearcher};


fn n_ptl_fpt(n : usize) -> Result<f64, Error>{
    // 매번 setup을 새로하는 코드
    // 10ptl : 97.983 us 101.04 us 104.68 us
    // 100ptl : 267.73 us 275.54 us 284.39 us / Performance has regressed with 170% rate
    // 1000ptl : 5.7822 ms 5.9184 ms 6.0646 ms
    // Performance has regressed with 1900% rate

    // C code
    // 10ptl : 3.71ms per iteration (rust is x37 faster)
    // 100ptl : 3ms per iteration (x 12 faster)
    // 1000ptl : 0.167s per iteartion (x 30 faster)

    // 하지만 양쪽 코드 모두 setup 시간이 상당히 소모된다.
    // C 코드에서 setup 1번에 계산되는 ensemble 수를 늘리면 상당히 줄어듬.
    // setup 한번에 순수하게 ensemble 만 계산하는 코드를 다시 짜야할 것.

    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dt : f64 = 5e-3;                                    // Time step
    let dim : usize = 2;                                    // dimension of system
    let mut rng : Pcg64 = rng_seed(1231412314);             // Random number generator

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target
    let mut vec_searchers : Vec<ContPassiveIndepSearcher> = Vec::with_capacity(n);

    for _i in 0..n{
        let searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, MoveType::Brownian(1f64))?;
        vec_searchers.push(searcher);
    }
    let mut list_searchers : LinkedList<ContPassiveIndepSearcher> = LinkedList::from(vec_searchers);
    let mut time : f64 = 0f64;

    'outer : loop{
        list_searchers.into_iter();
        time += dt;
        while let Some(searcher) = list_searchers.get_mut(){
            let mut single_move : Position<f64> = searcher.random_move(&mut rng, dt)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            if target.check_find(&searcher.pos)?{
                break 'outer;
            }
        }
    }

    Ok(time)
}

fn bench_rts(c : &mut Criterion){
    c.bench_function("rts_iterator", |b|  b.iter(|| n_ptl_fpt(1000)));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);