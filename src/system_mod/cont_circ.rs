// Modules for continous circular system.
// 연속된 원형, 혹은 구형 시스템

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ContCircSystem{              // 연속 원형(구형) 시스템
    pub stype : SystemType,             // System type
    pub bctype : BoundaryCond,          // Boundary condition : here, only reflective bc is available
    pub sys_size : f64,                   // radius of system
    pub dim : usize,                    // dimension of system
}

impl ContCircSystem{
    pub fn new(r : f64, dim : usize) -> Self{
        // r : radius of system
        // dim : dimension of system

        ContCircSystem{
            stype : SystemType::ContinuousCircular,
            bctype : BoundaryCond::Reflection,
            sys_size : r,
            dim : dim,
        }
    }
}

impl_argument_trait!(ContCircSystem, ContCircSystemArguments, 2,
    stype, SystemType, SystemType::ContinuousCircular,
    bctype, BoundaryCond, BoundaryCond::Reflection;
    sys_size, f64, "Size of System",
    dim, usize, "Dimension of System");

impl SystemCore<f64> for ContCircSystem{
    fn check_inclusion(&self, pos: &Position<f64>) -> Result<bool, Error>{
        // Return whether a position vector is in the system
        // pos : vector to check

        // 주어진 pos의 dimension이 system dimension과 다른 경우 error
        if self.dim != pos.coordinate.len(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let r : f64 = pos.norm();   // distance between center and position

        if r > self.sys_size{
            return Ok(false);
        }
        else{
            return Ok(true);
        }
    }

    fn check_bc(&self, pos: &mut Position<f64>, dp: &mut Position<f64>) -> Result<(), Error>{
        // check boundary condition
        // For every movement of ptl (from pos to pos + dp here)
        // we should check the boundary condition
        // pos : initial position of ptl
        // dp : displacement of ptl

        pos.mut_add(dp)?;                           // add dp to pos
        if self.check_inclusion(pos)?{              // if ptl is still in the system after movement
            return Ok(());                          // return
        }

        let r0 : f64 = self.sys_size;
        let s : f64 = pos.norm();
        pos.mut_scalar_mul((2f64 * r0 - s) / s);    // 그냥 반지름 비례로 크기만 줄임. 꽤나 정확함.
        if self.check_inclusion(pos)?{              // 지금은 안에 있는가?
            return Ok(());
        }
        return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
        // 이렇게 했는데도 밖에 있단 의미는 step size가 너무 크단 의미임. error.
    }



    fn random_pos(&self, rng: &mut Pcg64) -> Result<Position<f64>, Error>{
        // System 내부의 임의의 위치를 uniform하게 뽑아 반환
        // rng : random number generator

        use crate::random_mod::get_uniform_vec;

        let r : f64 = self.sys_size;
        let dim : usize = self.dim;
        let pos0 : Position<f64> = Position::<f64>::new(vec![-0.5f64; self.dim]);
        let mut pos1;
        loop{
            pos1 = &get_uniform_vec(rng, dim) + &pos0;          // 일단 (0, 1)^dim 에서 뽑은 후에, (-0.5, 0.5)^dim이 되도록
                                                                // 평행이동
            pos1.mut_scalar_mul(2f64 * r);                      // 그러고 2r을 곱해서 (-r, r)^dim로 변환
            if self.check_inclusion(&pos1)?{                    // 그래도 system 밖에 있을 수 있으니 check
                break;
            }
        }
        return Ok(pos1);
    }

    fn random_pos_to_vec(&self, rng: &mut Pcg64, vec: &mut Position<f64>) -> Result<(), Error>{
        // System 내부의 임의의 위치를 uniform하게 뽑아서 mutable reference에 기입
        // rng : random number generator
        // vec : 결과 적을 mutable reference

        use crate::random_mod::get_uniform_to_vec_nonstandard;

        let r : f64 = self.sys_size;
        let dim : usize = self.dim;
        if vec.dim() != dim{
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        loop{
            vec.clear();
            get_uniform_to_vec_nonstandard(rng, vec, -r, r);        // (-r, r)^dim 에서 uniform하게 뽑음
            if self.check_inclusion(vec)?{                          // 그리고 그게 system 내부에 있는지 확인
                break;
            }
        }
        return Ok(());
    }

    fn position_out_of_system(&self) -> Position<f64>{
        // system 밖의 점을 하나 출력해주는 함수
        // searcher를 새로 정의할 때, 맨 처음 위치를 시스템 밖에 두면 편리해서 생긴 기능

        let r : f64 = self.sys_size;
        let dim : usize = self.dim;
        return Position::new(vec![2f64 * r; dim]);   // (2r, 2r,...)  꼴은 무조건 밖에 있을 것.
    }

    fn position_out_of_system_to_vec(&self, vec: &mut Position<f64>) -> Result<(), Error>{
        // system 밖의 점을 하나 vector에 적어주는 함수

        if self.dim != vec.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let r : f64 = self.sys_size;
        for x in &mut vec.coordinate{
            *x = 2f64 * r;
        }
        Ok(())
    }
}

pub fn check_bc_exact(sys: ContCircSystem, pos: &mut Position<f64>, dp: &mut Position<f64>) -> Result<(), Error>{
    // Most exact way
    // 가장 정확한 방법으로 reflection을 계산하는 함수
    // pos 에서 pos + dp 를 잇는 선분에서 시스템의 경계점을 찾고
    // 그 경계점 위치가 곧 접평면의 법선벡터임을 이용해 대칭점을 계산하는 방식.
    // 단순히 중심과의 거리를 계산해 비례해서 시스템 안으로 옮겨오도록 하는 함수보다 2배 정도 느리다.
    // 움직이는 거리가 작으면 두 함수의 차이는 거의 없다.
    // sys : system configuration
    // pos : initial position of ptl
    // dp : displacement of ptl

    pos.mut_add(dp)?;
    if sys.check_inclusion(pos)?{
        return Ok(());
    }

    pos.mut_sub(dp)?;
    let r0 = sys.sys_size;
    let r = pos.norm();
    let dr = dp.norm();
    let rdr = pos.inner_product(dp)?;
    let t =  (- rdr + (rdr * rdr + dr * dr * (r0 * r0 - r * r)).sqrt()) / (dr * dr);
    let k = (1f64 - t) * (rdr + t * dr * dr) / (r0 * r0);

    dp.mut_scalar_mul(1f64 - 2f64 * k * t);
    pos.mut_scalar_mul(1f64 - 2f64 * k);
    pos.mut_add(dp)?;

    if sys.check_inclusion(pos)?{
        return Ok(());
    }
    return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
}

pub fn check_bc_first_order(sys: ContCircSystem, pos: &mut Position<f64>, dp: &mut Position<f64>) -> Result<(), Error>{
    // first order expansion
    // 위의 정확한 계산에서 dp가 작고, pos이 경계에 가깝단 조건에서 first order만 남긴 결과
    // 비례로 줄여들어오는 것보다 오히려 더 안좋다.

    pos.mut_add(dp)?;
    if sys.check_inclusion(pos)?{
        return Ok(());
    }

    pos.mut_sub(dp)?;
    let r0 = sys.sys_size;
    let r = pos.norm();
    let dr = dp.norm();
    let rdr = pos.inner_product(dp)?;

    let (a, b, cos) = (r / r0, dr / r0, rdr / (r * dr));

    let t = (1f64 - a) / (b * cos);
    let k = a * b * cos - a * (1f64 - a);

    pos.mut_scalar_mul(1f64 - 2f64 * k);
    dp.mut_scalar_mul(1f64 - 2f64 * k * t);
    pos.mut_add(dp)?;

    if sys.check_inclusion(pos)?{
        return Ok(());
    }
    return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_new(){
        let sys : ContCircSystem = ContCircSystem::new(3.0, 2);

        assert_eq!(sys.stype, SystemType::ContinuousCircular);
        assert_eq!(sys.sys_size, 3.0);
        assert_eq!(sys.dim, 2);
    }

    #[test]
    fn test_inclusion(){
        // System 안에 있는지 여부를 잘 확인하는지 테스트
        let sys : ContCircSystem = ContCircSystem::new(3.0, 2);

        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0, 0.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 3.0])), Ok(false));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0 ,0.0 ,0.0])),
            Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }

    #[test]
    fn test_random_pos() -> Result<(), Error>{
        // 시스템 내부에 uniform한 분포로 position을 하나 뽑는다.
        // 그 분포가 진짜로 uniform한지는 tests/random_vec에서 확인하고
        // 여기선 단지 그 결과가 실제로 시스템 내부에 잘 있는지 확인
        use crate::random_mod::rng_seed;

        let sys : ContCircSystem = ContCircSystem::new(3.0, 2);
        let mut rng : Pcg64 = rng_seed(1231412314);
        let n : usize = 10;

        for _i in 0..n{
            let pos : Position<f64> = sys.random_pos(&mut rng)?;
            assert_eq!(sys.check_inclusion(&pos), Ok(true));
        }

        Ok(())
    }

    #[test]
    fn test_check_bc() -> Result<(), Error>{
        // 2D test
        // boundary condition이 잘 작동하는지 확인
        // 차이가 너무 커지면 error가 날 수 있음.
        let sys : ContCircSystem = ContCircSystem::new(5.0, 2);

        for i in 0..10{
            let x : f64 = 4.05f64 + i as f64 * 0.1f64;
            let pos : Position<f64> = Position::new(vec![x, 0.0]);

            for j in 0..50{
                let y : f64 = j as f64 * 0.001f64;
                for k in 0..10{
                    let t : f64 = k as f64 * PI * 0.1f64;
                    let dp : Position<f64> = Position::new(vec![y * t.cos(), y * t.sin()]);

                    let mut pos1 = pos.clone();
                    let mut dp1 = dp.clone();
                    check_bc_exact(sys, &mut pos1, &mut dp1)?;

                    let mut pos2 = pos.clone();
                    let mut dp2 = dp.clone();
                    sys.check_bc(&mut pos2, &mut dp2)?;

                    assert_eq!(pos1, pos2);
                }
            }
        }
        return Ok(());
    }

    #[test]
    fn test_check_bc_single() -> Result<(), Error>{
        // 실제 사례들을 대입해보고 boundary conditino이 잘 작동하는지 확인.
        let sys : ContCircSystem = ContCircSystem::new(5.0, 2);
        let mut pos : Position<f64> = Position::new(vec![4.05, 0.0]);
        let mut dp : Position<f64> = Position::new(vec![0.96, 0.0]);
        let res : Position<f64> = Position::new(vec![4.99, 0.0]);

        check_bc_exact(sys, &mut pos, &mut dp)?;
        assert!((&pos - &res).norm() < 1e-10);
        return Ok(());
    }
}
