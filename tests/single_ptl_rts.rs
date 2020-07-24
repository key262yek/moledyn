// 단일 입자의 RTS 시스템 결과 확인
// 먼저 시간이 지남에 따라 입자 위치의 분산이 dt에 비례하게 커지는지에 대한
// 테스트를 진행하고
// 후에 시스템 중심에 타겟을 둔 상태로 움직일 때, first passage time의 분포와 평균을 확인하고자 한다.

use rand_pcg::Pcg64;
use rts::random_mod::rng_seed;
use rts::system_mod::{SystemCore, cont_circ::ContCircSystem};
use rts::target_mod::{TargetCore, cont_bulk::ContBulkTarget};
use rts::searcher_mod::{Passive, MoveType, cont_passive_indep::ContPassiveIndepSearcher};
use rts::position::{Position, Numerics};
use rts::error::Error;
use std::default::Default;

#[test]
#[ignore]
fn test_ptl_diffusion_in_time() -> Result<(), Error>{
    // time scale에 상관없이 시간 t가 지난 후의 ptl의 변위의 분산은 2Dt 여야만 한다.
    let mut rng : Pcg64 = rng_seed(123141234);                      // Random number generator
    const NUM_STEP : usize = 10;                                    // step for each ensemble
    const NUM_ENSEMBLE : usize = 10000;                             // number of ensemble
    let dt : f64 = 1e-2f64;                                         // Time duration between step
    let dim : usize = 2;                                            // Dimension of space
    let mut data : [f64; NUM_STEP + 1] = [0f64; NUM_STEP + 1];      // data array to store square of displacement
    let mut single_move : Position<f64> = Default::default();

    for _j in 0..NUM_ENSEMBLE{
        // Brownian searcher at center of dim-dimensional space with diffusion coefficient 1
        let mut searcher : ContPassiveIndepSearcher = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64),
                                                    Position::<f64>::new(vec![0.0; dim]));
        for k in 0..NUM_STEP{
            // Searcher moves with time dt
            single_move.clear();
            searcher.random_move_to_vec(&mut rng, dt, &mut single_move)?;
            searcher.pos.mut_add(&single_move)?;
            let dx2 : f64 = searcher.pos.norm().powi(2);          // Compute square of displacement
            data[k + 1] += dx2;                                     // Add to data
        }
    }

    for k in 1..=NUM_STEP{
        let var_disp = data[k] / NUM_ENSEMBLE as f64;               // variance of displacement
        // Fluctuation follows NUM_ENSEMBLE^{-0.5}
        assert!((var_disp - (2 * dim * k) as f64 * dt).abs() < 1f64 / (NUM_ENSEMBLE as f64).sqrt())
    }


    Ok(())
}

#[test]
#[ignore]
fn test_single_ptl_fpt() -> Result<(), Error>{
    // single ptl의 searching time 계산
    // system size 10, target_size 1, diffusion을 1로 계산한 상황에서
    // single ptl rts는 82 정도의 mfpt가 나와야함.
    // 테스트 결과 : 83.7
    // 적절하다.
    const NUM_ENSEMBLE : usize = 10000;                       // Number of ensemble
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dt : f64 = 1e-2;                                    // Time step
    let dim : usize = 2;                                    // dimension of system
    let mut data : f64 = 0f64;                              // variable for computing average fpt
    let mut single_move : Position<f64> = Position::new(vec![0.0; dim]);

    let mut rng : Pcg64 = rng_seed(1231412314);             // random number generator
    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target

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

    let mfpt : f64 = data / NUM_ENSEMBLE as f64;
    println!("{}\n", mfpt);
    Ok(())
}
