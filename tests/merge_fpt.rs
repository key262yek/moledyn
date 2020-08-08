// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_merge::ContPassiveMergeSearcher};

#[test]
fn test_mergeable_searcher() -> Result<(), Error>{
    // n ptl fpt가 iterator를 이용한 계산에서도 잘 나오는지 확인
    let data_set : [(usize, f64, f64, f64); 12] = [(1, 0.05, 1.0, 8.20877E+01),
                                         (2, 0.05, 1.0, 6.27514E+01),
                                         (4, 0.05, 1.0, 3.51468E+01), (8, 0.05, 1.0, 1.67379E+01),
                                         (16, 0.05, 1.0, 6.65563E+00), (32, 0.05, 1.0, 2.40294E+00),
                                         (1, 0.05, 0.0, 8.16412E+01), (2, 0.05, 0.0, 4.73814E+01),
                                         (4, 0.05, 0.0, 2.38803E+01), (8, 0.05, 0.0, 1.09340E+01),
                                         (16, 0.05, 0.0, 4.64192E+00), (32, 0.05, 0.0, 1.74246E+00)];

    let num_ensemble : usize = 1000;
    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator

    for &data in data_set.iter(){
        let (n, radius, alpha, mfpt_c) = data;
        let mfpt = ensemble_n_ptl_fpt(n, num_ensemble,  radius, alpha, &mut rng)?;
        println!("{} {} {} dataset. expect : {}, what we get : {}", n, radius, alpha, mfpt_c, mfpt);
        assert!(((mfpt - mfpt_c)/mfpt_c).abs() < (num_ensemble as f64).powf(-0.2));
    }
    Ok(())
}


fn ensemble_n_ptl_fpt(n : usize, num_ensemble : usize, radius : f64, alpha : f64, rng: &mut Pcg64) -> Result<f64, Error>{
    // n ptl fpt를 num_ensemble 만큼 반복
    let mut data : f64 = 0f64;
    for _i in 0..num_ensemble{
        let time : f64 = n_ptl_fpt(n, radius, alpha, rng)?;
        data += time;
    }
    return Ok(data / num_ensemble as f64);
}


fn n_ptl_fpt(n : usize, radius : f64, alpha : f64, rng: &mut Pcg64) -> Result<f64, Error>{
    // n ptl이 target을 찾는 FPT를 출력해주는 함수.
    // n번 시스템을 돌려서 그 중 최소 fpt를 반환함.
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dt : f64 = 1e-2;                                    // Time step
    let dim : usize = 2;                                    // dimension of system

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target
    let mut vec_searchers : Vec<ContPassiveMergeSearcher> = Vec::with_capacity(n);

    for _i in 0..n{
        let searcher = ContPassiveMergeSearcher::new_uniform(&sys, &target, rng, MoveType::Brownian(1f64), radius, alpha)?;
        vec_searchers.push(searcher);
    }
    let mut list_searchers : LinkedList<ContPassiveMergeSearcher> = LinkedList::from(vec_searchers);
    let mut time : f64 = 0f64;

    'outer : loop{
        time += dt;

        list_searchers.into_iter();
        while let Some(searcher) = list_searchers.get_mut(){
            let mut single_move : Position<f64> = searcher.random_move(rng, dt)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            if target.check_find(&searcher.pos)?{
                break 'outer;
            }
        }

        list_searchers.into_double_iter();
        while let Some((idx1, s1, idx2, s2)) = list_searchers.enumerate_double(){
            let d : f64 = s1.pos().distance(s2.pos())?;
            if d < 2f64 * radius{
                list_searchers.merge(idx1, idx2)?;
            }
        }
    }

    Ok(time)
}
