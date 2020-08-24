// Modules for continous circular system.
// 연속된 원형, 혹은 구형 시스템

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ContCubicSystem{              // 연속 cubic 시스템
    pub sys_type : SystemType,             // System type
    pub bctype : BoundaryCond,          // Boundary condition : reflective or periodic
    pub sys_size : f64,                   // width,length,height of system. ex. 1D domain = (-sys_size, sys_size)
    pub dim : usize,                    // dimension of system
}

impl ContCubicSystem{
    pub fn new(bctype : BoundaryCond, length : f64, dim : usize) -> Self{
        // length : length, width, height of system
        // dim : dimension of system

        ContCubicSystem{
            sys_type : SystemType::ContinuousRectangular,
            bctype : bctype,
            sys_size : length,
            dim : dim,
        }
    }
}

impl_argument_trait!(ContCubicSystem, "System", ContCubicSystemArguments, 3,
    sys_type, SystemType, SystemType::ContinuousRectangular;
    bctype, BoundaryCond, "Boundary condition. ex) Reflection Periodic, or 0 indicate Reflection, 1 indicate Periodic",
    sys_size, f64, "Size of System",
    dim, usize, "Dimension of System");

impl ContCubicSystem{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContCubicSystemArguments) -> Self{
        Self{
            sys_type    : argument.sys_type,
            bctype      : argument.bctype,
            sys_size    : argument.sys_size,
            dim         : argument.dim,
        }
    }
}


impl SystemCore<f64> for ContCubicSystem{
    fn check_inclusion(&self, pos: &Position<f64>) -> Result<bool, Error>{
        // Return whether a position vector is in the system
        // pos : vector to check

        // 주어진 pos의 dimension이 system dimension과 다른 경우 error
        if self.dim != pos.coordinate.len(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let length = self.sys_size;
        for x in &pos.coordinate{
            if x.abs() > length{
                return Ok(false);
            }
        }
        return Ok(true);
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

        match self.bctype{
            BoundaryCond::Reflection => {
                let length : f64 = self.sys_size;
                for x in &mut pos.coordinate{
                    if (*x).abs() < length{
                        continue;
                    }

                    if *x < 0f64{
                        *x = - 2f64 * length - *x;
                    }
                    else{
                        *x = 2f64 * length - *x;
                    }
                }
                if self.check_inclusion(pos)?{              // 지금은 안에 있는가?
                    return Ok(());
                }
            },
            BoundaryCond::Periodic => {
                let length : f64 = self.sys_size;
                for x in &mut pos.coordinate{
                    if (*x).abs() < length{
                        continue;
                    }

                    if *x < 0f64{
                        *x = 2f64 * length + *x;
                    }
                    else{
                        *x = - 2f64 * length + *x;
                    }
                }
                if self.check_inclusion(pos)?{              // 지금은 안에 있는가?
                    return Ok(());
                }
            }
        }

        return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
        // 이렇게 했는데도 밖에 있단 의미는 step size가 너무 크단 의미임. error.
    }



    fn random_pos(&self, rng: &mut Pcg64) -> Result<Position<f64>, Error>{
        // System 내부의 임의의 위치를 uniform하게 뽑아 반환
        // rng : random number generator

        use crate::random_mod::get_uniform_vec;

        let length : f64 = self.sys_size;
        let dim : usize = self.dim;
        let pos0 : Position<f64> = Position::<f64>::new(vec![-0.5f64; self.dim]);
        let mut pos1 = &get_uniform_vec(rng, dim) + &pos0;
        pos1.mut_scalar_mul(2f64 * length);
        return Ok(pos1);
    }

    fn random_pos_to_vec(&self, rng: &mut Pcg64, vec: &mut Position<f64>) -> Result<(), Error>{
        // System 내부의 임의의 위치를 uniform하게 뽑아서 mutable reference에 기입
        // rng : random number generator
        // vec : 결과 적을 mutable reference

        use crate::random_mod::get_uniform_to_vec_nonstandard;

        let length : f64 = self.sys_size;
        let dim : usize = self.dim;
        if vec.dim() != dim{
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        vec.clear();
        get_uniform_to_vec_nonstandard(rng, vec, -length, length);        // (-length, length)^dim 에서 uniform하게 뽑음
        return Ok(());
    }

    fn position_out_of_system(&self) -> Position<f64>{
        // system 밖의 점을 하나 출력해주는 함수
        // searcher를 새로 정의할 때, 맨 처음 위치를 시스템 밖에 두면 편리해서 생긴 기능

        let length : f64 = self.sys_size;
        let dim : usize = self.dim;
        return Position::new(vec![2f64 * length; dim]);   // (2length, 2length,...)  꼴은 무조건 밖에 있을 것.
    }

    fn position_out_of_system_to_vec(&self, vec: &mut Position<f64>) -> Result<(), Error>{
        // system 밖의 점을 하나 vector에 적어주는 함수

        if self.dim != vec.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let length : f64 = self.sys_size;
        for x in &mut vec.coordinate{
            *x = 2f64 * length;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let sys : ContCubicSystem = ContCubicSystem::new(BoundaryCond::Reflection, 3.0, 2);

        assert_eq!(sys.sys_type, SystemType::ContinuousRectangular);
        assert_eq!(sys.bctype, BoundaryCond::Reflection);
        assert_eq!(sys.sys_size, 3.0);
        assert_eq!(sys.dim, 2);
    }

    #[test]
    fn test_inclusion(){
        // System 안에 있는지 여부를 잘 확인하는지 테스트
        let sys : ContCubicSystem = ContCubicSystem::new(BoundaryCond::Reflection, 3.0, 2);

        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0, 0.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 0.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 3.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 4.0])), Ok(false));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0 ,0.0 ,0.0])),
            Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }

    #[test]
    fn test_random_pos() -> Result<(), Error>{
        // 시스템 내부에 uniform한 분포로 position을 하나 뽑는다.
        // 여기선 단지 그 결과가 실제로 시스템 내부에 잘 있는지 확인
        use crate::random_mod::rng_seed;

        let sys : ContCubicSystem = ContCubicSystem::new(BoundaryCond::Reflection, 3.0, 2);
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
        // 1D test
        // boundary condition이 잘 작동하는지 확인

        // Reflective boundary condition
        let sys : ContCubicSystem = ContCubicSystem::new(BoundaryCond::Reflection, 5.0, 1);

        for i in 0..10{
            let x : f64 = 4.05f64 + i as f64 * 0.1f64;
            let pos : Position<f64> = Position::new(vec![x]);

            for j in 0..50{
                let y : f64 = j as f64 * 0.001f64;
                let mut dp : Position<f64> = Position::new(vec![y]);

                let mut pos2 = pos.clone();
                sys.check_bc(&mut pos2, &mut dp)?;

                let res = if x + y < 5.0 {x + y} else {10f64 - (x + y)};
                assert_eq!(pos2.coordinate[0], res);
            }
        }

        // Periodic boundary condition
        let sys : ContCubicSystem = ContCubicSystem::new(BoundaryCond::Periodic, 5.0, 1);

        for i in 0..10{
            let x : f64 = 4.05f64 + i as f64 * 0.1f64;
            let pos : Position<f64> = Position::new(vec![x]);

            for j in 0..50{
                let y : f64 = j as f64 * 0.001f64;
                let mut dp : Position<f64> = Position::new(vec![y]);

                let mut pos2 = pos.clone();
                sys.check_bc(&mut pos2, &mut dp)?;

                let res = if x + y < 5.0 {x + y} else {-10f64 + x + y};
                assert_eq!(pos2.coordinate[0], res);
            }
        }
        return Ok(());
    }
}
