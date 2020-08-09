// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use criterion::{criterion_group, criterion_main, Criterion};
use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_indep::ContPassiveIndepSearcher};


fn n_ptl_fpt(n : usize, sys: &ContCircSystem, target : &ContBulkTarget, list_searchers : &mut LinkedList<ContPassiveIndepSearcher>, rng : &mut Pcg64) -> Result<f64, Error>{
    // random_move에서 불필요한 allocation이 계속되도록 두고 있었음.
    // setup 제외 performance
    // 10ptl : 449.80 us 467.54 us 486.47 us / Performance improved -32.549% -27.152% -18.631%
    // 100ptl : 206.34 us 217.44 us 230.10 us / Performance imporved -58.337% -52.825% -47.472%
    // 1000ptl : 369.79 us 386.71 us 412.86 us / Performance regressed +59.059% +77.191% +100.30%
    // C에 비해 충분히 빨라졌다!

    // 그래도 1000개 시스템에서 regression이 있음. 확인필요.
    // 매번 searcher를 새로 정의하는 것 역시 매우 큰 memory 낭비.
    // searcher에서 renew 함수 추가함.

    // 10ptl : 447.62 us 466.76 us 490.04 us / Performance regressed +11.868% +27.489% +42.990%
    // 100ptl : 171.27 us 175.26 us 179.19 us / Performance improved -68.732% -65.967% -62.827%
    // 1000ptl : 98.720 us 101.43 us 104.61 us / Performance improved -45.547% -41.784% -37.552%
    // 1000ptl 에서 4배정도 빨라졌다!

    let dt : f64 = 5e-3;                                    // Time step

    for i in 0..n{
        list_searchers.contents[i].renew_uniform(sys, target, rng)?;
    }
    let mut time : f64 = 0f64;
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

    let mut vec_searchers : Vec<ContPassiveIndepSearcher> = Vec::with_capacity(n);
    for _i in 0..n{
        let mut searcher = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64), Position::<f64>::new(vec![0.0; dim]));
        searcher.itype = InitType::Uniform;
        vec_searchers.push(searcher);
    }
    let mut list_searchers : LinkedList<ContPassiveIndepSearcher> = LinkedList::from(vec_searchers);

    c.bench_function("rts_iterator", |b|  b.iter(|| n_ptl_fpt(n, &sys, &target, &mut list_searchers, &mut rng)));
}

criterion_group!(benches, bench_rts);
criterion_main!(benches);