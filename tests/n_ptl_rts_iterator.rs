// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use rts::prelude::*;
use rts::system_mod::{cont_circ::ContCircSystem};
use rts::target_mod::{cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, cont_passive_indep::ContPassiveIndepSearcher};

#[test]
fn test_n_ptl_iterator() -> Result<(), Error>{
    // n ptl fpt가 iterator를 이용한 계산에서도 잘 나오는지 확인
    let data_set : [(usize, f64); 30] = [(1, 8.00462E+01), (2, 3.85612E+01), (3, 2.48137E+01), (4, 1.81125E+01),
                                         (5, 1.39493E+01), (6, 1.13163E+01), (7, 9.34559E+00), (8, 7.98293E+00),
                                         (9, 6.96739E+00), (10, 6.06164E+00), (15, 3.62867E+00), (20, 2.47003E+00),
                                         (25, 1.82103E+00), (30, 1.40517E+00), (35, 1.12499E+00), (40, 9.24904E-01),
                                         (45, 7.83406E-01), (50, 6.68625E-01), (60, 5.06901E-01), (70, 4.02499E-01),
                                         (80, 3.29755E-01), (90, 2.70959E-01), (100, 2.32858E-01), (109, 2.02250E-01),
                                         (120, 1.70510E-01), (133, 1.51860E-01), (146, 1.28170E-01), (161, 1.07530E-01),
                                         (177, 9.32130E-02), (194, 8.14110E-02)];

    let num_ensemble : usize = 100;
    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator

    for &data in data_set[..10].iter(){
        let (n, mfpt_c) = data;
        let mfpt = ensemble_n_ptl_fpt(n, num_ensemble, &mut rng)?;
        assert!(((mfpt - mfpt_c)/mfpt_c).abs() < (num_ensemble as f64).powf(-0.1));
    }
    Ok(())
}


fn ensemble_n_ptl_fpt(n : usize, num_ensemble : usize, rng: &mut Pcg64) -> Result<f64, Error>{
    // n ptl fpt를 num_ensemble 만큼 반복
    let mut data : f64 = 0f64;
    for _i in 0..num_ensemble{
        let time : f64 = n_ptl_fpt(n, rng)?;
        data += time;
    }
    return Ok(data / num_ensemble as f64);
}


fn n_ptl_fpt(n : usize, rng: &mut Pcg64) -> Result<f64, Error>{
    // n ptl이 target을 찾는 FPT를 출력해주는 함수.
    // n번 시스템을 돌려서 그 중 최소 fpt를 반환함.
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dt : f64 = 5e-3;                                    // Time step
    let dim : usize = 2;                                    // dimension of system

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target
    let mut vec_searchers : Vec<ContPassiveIndepSearcher> = Vec::with_capacity(n);

    for _i in 0..n{
        let searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, rng, MoveType::Brownian(1f64))?;
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
