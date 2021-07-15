// N ptl rts test
// iterator 를 이용해 한 시스템에 n ptl이 모두 있는 상황의 simulation

use moledyn::prelude::*;
use moledyn::system_mod::{cont_circ::ContCircSystem};
use moledyn::target_mod::{cont_bulk::ContBulkTarget};
use moledyn::agent_mod::{Passive, cont_passive_merge::ContPassiveMergeAgent};
use moledyn::time_mod::{ExponentialStep};

#[test]
#[ignore]
fn test_many_mergeable_searcher() -> Result<(), Error>{
    // n ptl fpt가 iterator를 이용한 계산에서도 잘 나오는지 확인
    let data_set : [(usize, f64, f64, f64); 4] = [(256, 0.2, 1.0, 6.28692e-01), (512, 0.2, 1.0, 1.45039e-01),
                                (1024, 0.2, 1.0, 6.53105e-02), (2048, 0.2, 1.0, 2.42769e-02)];

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
    let sys_size : f64 = 10f64;                             // System size
    let target_size : f64 = 1f64;                           // Target size
    let dim : usize = 2;                                    // dimension of system
    let mut timeiter = ExponentialStep::new(1e-10, 1e-5, 10)?;

    let sys : ContCircSystem = ContCircSystem::new(sys_size, dim);   // System
    let target : ContBulkTarget = ContBulkTarget::new(Position::new(vec![0.0; dim]), target_size);  // Target
    let mut vec_agents : Vec<ContPassiveMergeAgent> = Vec::with_capacity(n);

    for _i in 0..n{
        let agent = ContPassiveMergeAgent::new_uniform(&sys, &target, rng, MoveType::Brownian(1f64), radius, alpha)?;
        vec_agents.push(agent);
    }
    let mut list_agents : LinkedList<ContPassiveMergeAgent> = LinkedList::from(vec_agents);

    let mut data : f64 = 0f64;
    for _i in 0..num_ensemble{
        let time : f64 = n_ptl_fpt(&sys, &target, &mut list_agents, &mut timeiter, rng)?;
        data += time;
    }
    return Ok(data / num_ensemble as f64);
}


fn n_ptl_fpt(sys : &ContCircSystem, target : &ContBulkTarget, list_agents : &mut LinkedList<ContPassiveMergeAgent>, timeiter : &mut ExponentialStep, rng: &mut Pcg64) -> Result<f64, Error>{
    // n ptl이 target을 찾는 FPT를 출력해주는 함수.
    // n번 시스템을 돌려서 그 중 최소 fpt를 반환함.

    timeiter.renew();
    list_agents.connect_all()?;
    list_agents.into_iter();
    while let Some(agent) = list_agents.get_mut(){
        agent.renew_uniform(sys, target, rng)?;
    }

    for (time, dt) in timeiter.into_diff(){
        list_agents.into_iter();
        while let Some(agent) = list_agents.get_mut(){
            let mut single_move : Position<f64> = agent.random_move(rng, dt)?;
            sys.check_bc(&mut agent.pos, &mut single_move)?;
            if target.check_find(&agent.pos)?{
                return Ok(time);
            }
        }

        list_agents.into_double_iter();
        while let Some((idx1, s1, idx2, s2)) = list_agents.enumerate_double(){
            let d : f64 = s1.pos().distance(s2.pos())?;
            if d < 2f64 * s1.ptl_radius{
                list_agents.merge(idx1, idx2)?;
            }
        }
    }

    Err(Error::make_error_syntax(ErrorCode::UnexpectedEnd))
}
