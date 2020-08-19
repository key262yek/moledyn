// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use criterion::{criterion_group, criterion_main, Criterion};
use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_merge::ContPassiveMergeSearcher};


fn n_ptl_fpt(n : usize, sys: &ContCircSystem, target : &ContBulkTarget, list_searchers : &mut LinkedList<ContPassiveMergeSearcher>, rng : &mut Pcg64) -> Result<f64, Error>{
    // merge 기능을 포함한 시뮬레이션의 benchmark
    // 10ptl : 847.19 us
    // 100ptl : 1.8697 ms
    // 1000ptl : 4.3767 ms

    // C code 결과 : 1set 당 1000ensemble (10set 평균)
    // 10ptl : 4.45ms per iteration
    // 100ptl : 5.8ms per iteration
    // 1000ptl (100번 1set만 돌림) : 44ms per iteration

    let dt : f64 = 5e-3;                                                    // Time step
    let mut time : f64 = 0f64;
    for i in 0..n{
        list_searchers.contents[i].renew_uniform(sys, target, rng)?;
    }
    list_searchers.connect_all()?;
    let mut single_move = Position::<f64>::new(vec![0.0; sys.dim]);

    'outer : loop{
        list_searchers.into_iter();
        time += dt;
        while let Some(searcher) = list_searchers.get_mut(){
            single_move.clear();
            searcher.random_move_to_vec(rng, dt, &mut single_move)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            if target.check_find(&searcher.pos)?{
                break 'outer;
            }
        }

        list_searchers.into_double_iter();
        while let Some((idx1, s1, idx2, s2)) = list_searchers.enumerate_double(){
            let d : f64 = s1.pos.distance(&s2.pos)?;
            if d < 2f64 * s1.radius{
                list_searchers.merge(idx1, idx2)?;
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
    let n : usize = 1000;
    let radius : f64 = 0.05;
    let alpha : f64 = 1.0;

    let mut vec_searchers : Vec<ContPassiveMergeSearcher> = Vec::with_capacity(n);
    for _i in 0..n{
        let mut searcher = ContPassiveMergeSearcher::new(MoveType::Brownian(1f64), Position::<f64>::new(vec![0.0; dim]), radius, alpha);
        searcher.itype = InitType::Uniform;
        vec_searchers.push(searcher);
    }
    let mut list_searchers : LinkedList<ContPassiveMergeSearcher> = LinkedList::from(vec_searchers);

    c.bench_function("rts_iterator", |b|  b.iter(|| n_ptl_fpt(n, &sys, &target, &mut list_searchers, &mut rng)));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);
