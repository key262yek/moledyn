// Modules for continous circular system.
//

use crate::error::{Error, ErrorCode};
use crate::system_mod::{SystemType, BoundaryCond, SystemCore};
use crate::position::{Position, Numerics};
use rand_pcg::Pcg64;


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ContCircSystem{
    pub stype : SystemType,
    pub bctype : BoundaryCond,
    pub radius : f64,
    pub dim : usize,
}

impl ContCircSystem{
    pub fn new(r : f64, dim : usize) -> Self{
        ContCircSystem{
            stype : SystemType::ContinousCircular,
            bctype : BoundaryCond::Reflection,
            radius : r,
            dim : dim,
        }
    }
}

impl SystemCore<f64> for ContCircSystem{
    // Return whether a position vector is in the system
    fn check_inclusion(&self, pos: &Position<f64>) -> Result<bool, Error>{
        if self.dim != pos.coordinate.len(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let r : f64 = pos.norm();

        if r > self.radius{
            return Ok(false);
        }
        else{
            return Ok(true);
        }
    }

    fn check_bc(&self, pos: &mut Position<f64>, dp: &mut Position<f64>) -> Result<(), Error>{
        // first order 보다는 훨씬 정확함.
        // 얼마나 작아야 정확하려나
        pos.mut_add(dp)?;
        if self.check_inclusion(pos)?{
            return Ok(());
        }

        let r0 : f64 = self.radius;
        let s : f64 = pos.norm();
        pos.mut_scalar_mul((2f64 * r0 - s) / s);
        if self.check_inclusion(pos)?{
            return Ok(());
        }
        return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
    }


    // System 내부의 임의의 위치를 uniform하게 뽑아 반환
    fn random_pos(&self, rng: &mut Pcg64) -> Result<Position<f64>, Error>{
        use crate::random_mod::get_uniform_vec;

        let r : f64 = self.radius;
        let dim : usize = self.dim;
        let pos0 : Position<f64> = Position::<f64>::new(vec![-0.5f64, -0.5f64]);
        let mut pos1;
        loop{
            pos1 = &get_uniform_vec(rng, dim) + &pos0;
            pos1.mut_scalar_mul(2f64 * r);
            if self.check_inclusion(&pos1)?{
                break;
            }
        }
        return Ok(pos1);
    }

    // System 내부의 임의의 위치를 uniform하게 뽑아서 mutable reference에 기입
    fn random_pos_to_vec(&self, rng: &mut Pcg64, vec: &mut Position<f64>) -> Result<(), Error>{
        use crate::random_mod::get_uniform_to_vec_nonstandard;

        let r : f64 = self.radius;
        let dim : usize = self.dim;
        if vec.dim() != dim{
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        loop{
            vec.clear();
            get_uniform_to_vec_nonstandard(rng, vec, -r, r);
            if self.check_inclusion(vec)?{
                break;
            }
        }
        return Ok(());
    }

    // system 밖의 점을 하나 출력해주는 함수
    fn position_out_of_system(&self) -> Position<f64>{
        let r : f64 = self.radius;
        let dim : usize = self.dim;
        return Position::new(vec![2f64 * r; dim]);
    }

    // system 밖의 점을 하나 vector에 적어주는 함수
    fn position_out_of_system_to_vec(&self, vec: &mut Position<f64>) -> Result<(), Error>{
        if self.dim != vec.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let r : f64 = self.radius;
        for x in &mut vec.coordinate{
            *x = 2f64 * r;
        }
        Ok(())
    }
}

pub fn check_bc_exact(sys: ContCircSystem, pos: &mut Position<f64>, dp: &mut Position<f64>) -> Result<(), Error>{
    // Most exact way
    pos.mut_add(dp)?;
    if sys.check_inclusion(pos)?{
        return Ok(());
    }

    pos.mut_sub(dp)?;
    let r0 = sys.radius;
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
    // 비례로 줄여들어오는 것보다 오히려 더 안좋다.
    pos.mut_add(dp)?;
    if sys.check_inclusion(pos)?{
        return Ok(());
    }

    pos.mut_sub(dp)?;
    let r0 = sys.radius;
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

        assert_eq!(sys.stype, SystemType::ContinousCircular);
        assert_eq!(sys.radius, 3.0);
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
