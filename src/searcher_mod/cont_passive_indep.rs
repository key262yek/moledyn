// Module for Continous Passive Independent Searcher

use crate::error::{Error, ErrorCode};
use crate::searcher_mod::{SearcherType, SearcherCore, MoveType, Passive};
use crate::system_mod::{SystemCore};
use crate::target_mod::{TargetCore};
use crate::random_mod::{get_gaussian_vec, get_gaussian_to_vec_nonstandard};
use rand_pcg::Pcg64;
use crate::position::{Position, Numerics};
use std::fmt::{self, Display, Formatter};


#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContPassiveIndepSearcher{        // 연속한 시스템에서 Passive하게 움직이는 independent searcher
    pub stype : SearcherType,               // Type of searcher
    pub mtype : MoveType,                   // Type of random movement
    pub dim : usize,                        // dimension of space containing searcher
    pub pos : Position<f64>,                // position of searcher
}

impl ContPassiveIndepSearcher{
    // 모든 정보를 제공했을 경우, 새 Searcher struct를 반환하는 함수
    pub fn new(mtype : MoveType, pos : Position<f64>) -> Self{
        ContPassiveIndepSearcher{
            stype : SearcherType::ContinousPassiveIndependent,
            mtype : mtype,
            dim : pos.dim(),
            pos : pos,
        }
    }

    pub fn new_uniform(sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64, mtype : MoveType) -> Result<Self, Error>{
        // system과 target이 주어져 있는 상황에서 시스템 domain 안에서 초기위치를 uniform하게 뽑아 searcher를 정의해주는 함수
        let mut pos : Position<f64> = sys.position_out_of_system();  // 초기값을 위해 무조건 시스템 밖의 벡터를 받도록 한다
        loop{
            sys.random_pos_to_vec(rng, &mut pos)?;   //
            if !target.check_find(&pos)?{
                break;
            }
        }

        Ok(ContPassiveIndepSearcher::new(mtype, pos))
    }
}

impl SearcherCore<f64> for ContPassiveIndepSearcher{

}

impl Passive<f64> for ContPassiveIndepSearcher{
    fn random_move(&self, rng : &mut Pcg64, dt : f64) -> Result<Position<f64>, Error>{
        match self.mtype{
            MoveType::Brownian(coeff_diff) => {
                let length : f64 = (2f64 * coeff_diff * dt).sqrt();
                let mut mv : Position<f64> = get_gaussian_vec(rng, self.dim);
                mv.mut_scalar_mul(length);
                Ok(mv)
            },
            _ => {
                Err(Error::make_error_syntax(ErrorCode::FeatureNotProvided))
            }
        }
    }

    fn random_move_to_vec(&self, rng: &mut Pcg64, dt: f64, vec: &mut Position<f64>) -> Result<(), Error>{
        if self.dim != vec.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        match self.mtype{
            MoveType::Brownian(coeff_diff) => {
                let length : f64 = (2f64 * coeff_diff * dt).sqrt();
                get_gaussian_to_vec_nonstandard(rng, vec, 0f64, length);
                Ok(())
            },
            _ => {
                Err(Error::make_error_syntax(ErrorCode::FeatureNotProvided))
            }
        }
    }
}

impl Display for ContPassiveIndepSearcher{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{}\nRandom walk : {}, Pos : ({})", self.stype, self.mtype, self.pos)
    }
}




#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt(){
        let searcher = ContPassiveIndepSearcher::new(MoveType::Brownian(0.0), Position::new(vec![0.0, 0.0]));
        assert_eq!(format!("{}", searcher).as_str(),
            "Passive Independent Searcher in Continous system.\nRandom walk : Brownian with diffusion coefficient 0, Pos : (0, 0)");
    }
}
