// Modules for continous circular system.
// 연속된 원형, 혹은 구형 시스템

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ContCylindricalSystem{              // 연속 cubic 시스템
    pub sys_type : SystemType,          // System type
    pub bctype : BoundaryCond,          // Boundary condition : mixed. reflection for circular domain, periodic for rectangular domain
    pub radius : f64,                   // radius of circular domain
    pub length : f64,                   // length of rectangular domain (-length, length)
    pub dim : usize,                    // dimension of system
}

impl ContCylindricalSystem{
    pub fn new(cyl_dim : usize, radius : f64, length : f64, dim : usize) -> Self{
        // length : length, width, height of system
        // dim : dimension of system

        if cyl_dim > dim || radius <= 0f64 || length <= 0f64 || dim == 0{
            panic!("{}", ErrorCode::InvalidArgumentInput);
        }

        ContCylindricalSystem{
            sys_type : SystemType::ContinuousCylindrical(cyl_dim),
            bctype : BoundaryCond::Mixed(cyl_dim),
            radius : radius,
            length : length,
            dim : dim,
        }
    }
}

impl_argument_trait!(ContCylindricalSystem, "System", ContCylindricalSystemArguments, 5;
    sys_type, SystemType, "Dimension of circular domain.",
    bctype, BoundaryCond, "Boundary condition. ex) Reflective, Periodic, integer indicates Mixed",
    radius, f64, "Radius of Circular Domain",
    length, f64, "Length of Rectangular Domain (-length, length)",
    dim, usize, "Dimension of System");

impl ContCylindricalSystem{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContCylindricalSystemArguments) -> Self{
        let cyl_dim1 = match argument.sys_type{
            SystemType::ContinuousCylindrical(d) => d,
            _ => {
                panic!("{:?}", ErrorCode::InvalidArgumentInput);
            },
        };
        let cyl_dim2 = match argument.bctype{
            BoundaryCond::Mixed(d) => d,
            _ => {
                panic!("{:?}", ErrorCode::InvalidArgumentInput);
            },
        };
        if cyl_dim1 != cyl_dim2 || cyl_dim1 > argument.dim || argument.radius < 0f64 ||
                argument.length <= 0f64 || argument.dim == 0{
            panic!("{:?}", ErrorCode::InvalidArgumentInput);
        }

        Self{
            sys_type    : argument.sys_type,
            bctype      : argument.bctype,
            radius      : argument.radius,
            length      : argument.length,
            dim         : argument.dim,
        }
    }
}


impl SystemCore<f64> for ContCylindricalSystem{
    fn check_inclusion(&self, pos: &Position<f64>) -> Result<bool, Error>{
        // Return whether a position vector is in the system
        // pos : vector to check

        // 주어진 pos의 dimension이 system dimension과 다른 경우 error
        if self.dim != pos.coordinate.len(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let d = match self.sys_type{
            SystemType::ContinuousCylindrical(d) => d,
            _ => {return Err(Error::make_error_syntax(ErrorCode::InvalidType))
            },
        };

        let radius = self.radius;
        let mut r = 0f64;
        for x in &pos[..d]{
            r += *x * *x;
        }
        r = r.sqrt();
        if r > radius{
            return Ok(false);
        }

        let length = self.length;
        for x in &pos[d..]{
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
            BoundaryCond::Mixed(d) =>{

                // Reflection
                let radius = self.radius;
                let mut r = 0f64;
                for x in &pos[..d]{
                    r += *x * *x;
                }
                r = r.sqrt();
                if r > radius{
                    let t = (2f64 * radius - r) / r;
                    for i in 0..d{
                        pos.coordinate[i] *= t;
                    }
                }


                // Periodic
                let length : f64 = self.length;
                for i in d..self.dim{
                    let x = &mut pos.coordinate[i];
                    if (*x).abs() <= length{
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
            _ => return Err(Error::make_error_syntax(ErrorCode::InvalidType)),
        }

        return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
        // 이렇게 했는데도 밖에 있단 의미는 step size가 너무 크단 의미임. error.
    }



    fn random_pos(&self, rng: &mut Pcg64) -> Result<Position<f64>, Error>{
        // System 내부의 임의의 위치를 uniform하게 뽑아 반환
        // rng : random number generator

        use crate::random_mod::get_uniform_vec;

        let d = match self.sys_type{
            SystemType::ContinuousCylindrical(dim) => dim,
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidType));
            },
        };
        let radius : f64 = self.radius;
        let length : f64 = self.length;
        let dim : usize = self.dim;
        let pos0 : Position<f64> = Position::<f64>::new(vec![-0.5f64; self.dim]);
        let mut pos1;
        loop{
            pos1 = &get_uniform_vec(rng, dim) + &pos0;          // 일단 (0, 1)^dim 에서 뽑은 후에, (-0.5, 0.5)^dim이 되도록

                                                                // 평행이동
            let mut r : f64 = 0f64;
            for i in 0..d{
                pos1[i] *= 2f64 * radius;
                r += pos1[i] * pos1[i];
            }
            for i in d..dim{
                pos1[i] *= 2f64 * length;
            }

            r = r.sqrt();
            if r < radius{
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

        let d = match self.sys_type{
            SystemType::ContinuousCylindrical(dim) => dim,
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidType));
            },
        };
        let length : f64 = self.length;
        let radius : f64 = self.radius;
        let dim : usize = self.dim;
        if vec.dim() != dim{
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        loop{
            vec.clear();
            get_uniform_to_vec_nonstandard(rng, vec, -1f64, 1f64);

            let mut r : f64 = 0f64;
            for i in 0..d{
                vec[i] *= 2f64 * radius;
                r += vec[i] * vec[i];
            }
            for i in d..dim{
                vec[i] *= 2f64 * length;
            }

            r = r.sqrt();
            if r < radius{
                break;
            }
        }

        return Ok(());
    }

    fn position_out_of_system(&self) -> Position<f64>{
        // system 밖의 점을 하나 출력해주는 함수
        // searcher를 새로 정의할 때, 맨 처음 위치를 시스템 밖에 두면 편리해서 생긴 기능

        let length : f64 = self.length;
        let dim : usize = self.dim;
        return Position::new(vec![2f64 * length; dim]);   // circular domain에선 아니더라도 rectangular domain에서 밖으로 나감
    }

    fn position_out_of_system_to_vec(&self, vec: &mut Position<f64>) -> Result<(), Error>{
        // system 밖의 점을 하나 vector에 적어주는 함수

        if self.dim != vec.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let length : f64 = self.length;
        for x in vec.iter_mut(){
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
        let sys : ContCylindricalSystem = ContCylindricalSystem::new(1, 3.0, 10.0, 2);

        assert_eq!(sys.sys_type, SystemType::ContinuousCylindrical(1));
        assert_eq!(sys.bctype, BoundaryCond::Mixed(1));
        assert_eq!(sys.radius, 3.0);
        assert_eq!(sys.length, 10.0);
        assert_eq!(sys.dim, 2);
    }

    #[test]
    fn test_inclusion(){
        // System 안에 있는지 여부를 잘 확인하는지 테스트
        let sys : ContCylindricalSystem = ContCylindricalSystem::new(1, 3.0, 10.0, 2);

        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0, 0.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 0.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 3.0])), Ok(true));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![3.0, 14.0])), Ok(false));
        assert_eq!(sys.check_inclusion(&Position::<f64>::new(vec![0.0 ,0.0 ,0.0])),
            Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }

    #[test]
    fn test_random_pos() -> Result<(), Error>{
        // 시스템 내부에 uniform한 분포로 position을 하나 뽑는다.
        // 여기선 단지 그 결과가 실제로 시스템 내부에 잘 있는지 확인
        use crate::random_mod::rng_seed;

        let sys : ContCylindricalSystem = ContCylindricalSystem::new(1, 3.0, 10.0, 2);
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

        // Mixed
        let sys : ContCylindricalSystem = ContCylindricalSystem::new(1, 5.0, 5.0, 2);

        for i in 0..10{
            let x : f64 = 4.05f64 + i as f64 * 0.1f64;
            let pos : Position<f64> = Position::new(vec![x, x]);

            for j in 0..200{
                let y : f64 = j as f64 * 0.001f64;

                for k in 0..200{
                    let z : f64 = k as f64 * 0.001f64;
                    let mut dp : Position<f64> = Position::new(vec![y, z]);

                    let mut pos2 = pos.clone();
                    sys.check_bc(&mut pos2, &mut dp)?;

                    let x1 = if x + y <= 5.0 {x + y} else {10f64 - x - y};
                    let x2 = if x + z <= 5.0 {x + z} else {-10f64 + x + z};
                    assert_eq!(pos2, Position::<f64>::new(vec![x1, x2]));
                }
            }
        }
        return Ok(());
    }
}
