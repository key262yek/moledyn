// Searcher가 여러개 있는 경우에서의 random target search문제 결과 확인하기

use rand_pcg::Pcg64;
use rts::random_mod::rng_seed;
use rts::system_mod::{SystemCore, cont_circ::ContCircSystem};
use rts::target_mod::{TargetCore, cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, MoveType, cont_passive_indep::ContPassiveIndepSearcher};
use rts::position::{Position};
use rts::error::Error;

#[test]
#[ignore]
fn test_n_ptl_fpt() -> Result<(), Error>{
    // n ptl fpt가 기존의 연구결과와 잘 맞는지 확인하는 테스트
    // 하지만 단일 cpu로 계산하기엔 시간이 너무 오래걸려서 접음.
    // 돌리면 분명 Fail이 날 것.
    let data_set : [(usize, f64); 30] = [(1, 8.00462E+01), (2, 3.85612E+01), (3, 2.48137E+01), (4, 1.81125E+01),
                                         (5, 1.39493E+01), (6, 1.13163E+01), (7, 9.34559E+00), (8, 7.98293E+00),
                                         (9, 6.96739E+00), (10, 6.06164E+00), (15, 3.62867E+00), (20, 2.47003E+00),
                                         (25, 1.82103E+00), (30, 1.40517E+00), (35, 1.12499E+00), (40, 9.24904E-01),
                                         (45, 7.83406E-01), (50, 6.68625E-01), (60, 5.06901E-01), (70, 4.02499E-01),
                                         (80, 3.29755E-01), (90, 2.70959E-01), (100, 2.32858E-01), (109, 2.02250E-01),
                                         (120, 1.70510E-01), (133, 1.51860E-01), (146, 1.28170E-01), (161, 1.07530E-01),
                                         (177, 9.32130E-02), (194, 8.14110E-02)];

    let num_ensemble : usize = 10;
    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator

    for &data in data_set.iter(){
        let (n, mfpt_c) = data;
        let mfpt = ensemble_n_ptl_fpt(n, num_ensemble, &mut rng)?;
        assert!(((mfpt - mfpt_c)/mfpt_c).abs() < (num_ensemble as f64).powf(-0.3));
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
    let dt : f64 = 1e-2;                                    // Time step
    let dim : usize = 2;                                    // dimension of system
    let mut min_fpt : f64 = std::f64::MAX;                              // variable for computing average fpt

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target

    for _i in 0..n{
        // Searcher initially located by uniform distribution
        let mut searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, rng, MoveType::Brownian(1f64))?;
        let mut time : f64 = 0f64;

        while !target.check_find(&searcher.pos)?{
            let mut single_move : Position<f64> = searcher.random_move(rng, dt)?;
            sys.check_bc(&mut searcher.pos, &mut single_move)?;
            time += dt;
        }
        if min_fpt > time{
            min_fpt = time;
        }
    }

    Ok(min_fpt)
}
