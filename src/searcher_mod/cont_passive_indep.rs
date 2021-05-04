// Module for Continous Passive Independent Searcher

use crate::prelude::*;
use crate::searcher_mod::{Passive};
use crate::random_mod::{get_gaussian_vec, get_gaussian_to_vec_nonstandard};



#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContPassiveIndepSearcher{        // 연속한 시스템에서 Passive하게 움직이는 independent searcher
    pub searcher_type : SearcherType,               // Type of searcher
    pub mtype : MoveType,                   // Type of random movement
    pub itype : InitType<f64>,              // Type of Initialization
    pub dim : usize,                        // dimension of space containing searcher
    pub pos : Position<f64>,                // position of searcher
}

impl ContPassiveIndepSearcher{
    // 모든 정보를 제공했을 경우, 새 Searcher struct를 반환하는 함수
    pub fn new(mtype : MoveType, pos : Position<f64>) -> Self{
        // mtype : Random walk characteristic
        // pos : initial position of searcher

        ContPassiveIndepSearcher{
            searcher_type : SearcherType::ContinuousPassiveIndependent,
            mtype : mtype,
            itype : InitType::SpecificPosition(pos.clone()),
            dim : pos.dim(),
            pos : pos,
        }
    }

    pub fn new_uniform(sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64, mtype : MoveType) -> Result<Self, Error>{
        // system과 target이 주어져 있는 상황에서 시스템 domain 안에서 초기위치를 uniform하게 뽑아 searcher를 정의해주는 함수
        // sys : system configuration
        // target : target configuration
        // rng : random number generator
        // mtype : random walk characteristic

        let mut pos : Position<f64> = sys.position_out_of_system();  // 초기값을 위해 무조건 시스템 밖의 벡터를 받도록 한다
        loop{
            sys.random_pos_to_vec(rng, &mut pos)?;   // System 내부의 random position을 받는다
            if !target.check_find(&pos)?{            // 그 random position이 target과 이미 만났는가 확인
                break;
            }
        }

        Ok(ContPassiveIndepSearcher{
            searcher_type : SearcherType::ContinuousPassiveIndependent,
            mtype : mtype,
            itype : InitType::Uniform,
            dim : pos.dim(),
            pos : pos,
        })
    }

    pub fn renew_uniform(&mut self, sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64) -> Result<(), Error>{
        // 매번 searcher를 새로 정의하는 것 역시 상당한 memory 낭비이다.
        // 있는 searcher를 재활용하도록 하자.

        match sys.position_out_of_system_to_vec(&mut self.pos){
            Ok(()) => (),
            Err(_) => {
                self.pos = sys.position_out_of_system();
                self.dim = self.pos.dim();
            }
        }
        loop{
            sys.random_pos_to_vec(rng, &mut self.pos)?;   // System 내부의 random position을 받는다
            if !target.check_find(&self.pos)?{            // 그 random position이 target과 이미 만났는가 확인
                break;
            }
        }

        Ok(())
    }
}

impl_argument_trait!(ContPassiveIndepSearcher, "Searcher", ContPassiveIndepSearcherArguments, 3,
    searcher_type, SearcherType, SearcherType::ContinuousPassiveIndependent;
    mtype, MoveType, "Random walk Characterstic. ex) 1.0 : Brownian with D=1 / Levy : Levy walk",
    itype, InitType<f64>, "Initialization method. ex) 0,0 : All at 0,0 / Uniform : Uniform",
    num_searcher, usize, "Number of Searchers");

impl ContPassiveIndepSearcher{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContPassiveIndepSearcherArguments) -> Vec<Self>{
        let dim : usize;
        let pos : Position<f64>;

        match &argument.itype{
            InitType::<f64>::Uniform => {
                dim = 0;
                pos = Position::new(vec![]);
            },
            InitType::<f64>::SpecificPosition(p) =>{
                dim = p.dim();
                pos = p.clone();
            }
        }
        vec![Self{
            searcher_type   : argument.searcher_type,
            mtype           : argument.mtype,
            itype           : argument.itype.clone(),
            dim             : dim,
            pos             : pos,
        }; argument.num_searcher]

    }
}

impl SearcherCore<f64> for ContPassiveIndepSearcher{
    fn pos(&self) -> &Position<f64>{
        &self.pos
    }

    // Mutual displacement
    fn mutual_displacement(&self, other : &Self) -> Result<(Position<f64>, f64), Error>{
        if self.dim != other.dim{
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        let mut disp : Position<f64> = &other.pos - &self.pos;
        let distance : f64 = disp.norm();
        disp.mut_scalar_mul(1f64 / distance);
        return Ok((disp, distance));
    }

    fn mutual_displacement_to_vec(&self, other : &Self, vec : &mut Position<f64>) -> Result<f64, Error>{
        // return distance, and direction vector on vec
        if self.dim != other.dim || self.dim != vec.dim() {
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut s = 0f64;
        for i in 0..self.dim{
            let x = self.pos.coordinate[i];
            let y = other.pos.coordinate[i];

            vec[i] = y - x;
            s += (y - x).powi(2);
        }

        let distance : f64 = s.sqrt();
        vec.mut_scalar_mul(1f64 / distance);
        return Ok(distance);
    }

    fn mutual_distance(&self, other : &Self) -> Result<f64, Error>{
        self.pos().distance(other.pos())
    }
}

impl Passive<f64, f64> for ContPassiveIndepSearcher{
    fn random_move(&self, rng : &mut Pcg64, dt : f64) -> Result<Position<f64>, Error>{
        // Random walk characteristic에 따라 그에 맞는 random walk displacement를 반환
        // rng : random number generator
        // dt : time stpe size

        match self.mtype{
            MoveType::Brownian(coeff_diff) => {                                 // Brownian motion의 경우
                let length : f64 = (2f64 * coeff_diff * dt).sqrt();             // variance가 sqrt(2 D dt)
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
        // random walk displacement를 주어진 vec 행렬에 덮어씌워준다.
        // rng : Random number generator
        // dt : Time step size
        // vec : 값을 저장할 벡터
        if self.dim != vec.dim(){    // searcher가 움직이는 공간의 dimension과 주어진 vec의 dimension이 다르면?
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        match self.mtype{
            MoveType::Brownian(coeff_diff) => {                                 // Brownian motion의 경우
                let length : f64 = (2f64 * coeff_diff * dt).sqrt();             // variance가 sqrt(2 D dt)
                get_gaussian_to_vec_nonstandard(rng, vec, 0f64, length);
                Ok(())
            },
            _ => {
                Err(Error::make_error_syntax(ErrorCode::FeatureNotProvided))
            }
        }
    }
}


#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new(){
        let pos = Position::<f64>::new(vec![0.0, 0.0]);
        let searcher1 = ContPassiveIndepSearcher::new(MoveType::Brownian(1f64), pos.clone());
        assert_eq!(searcher1, ContPassiveIndepSearcher{
            searcher_type : SearcherType::ContinuousPassiveIndependent,
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::SpecificPosition(pos.clone()),
            dim     : 2,
            pos     : pos.clone(),
        });
    }

    #[test]
    fn test_uniform() -> Result<(), Error>{
        use crate::system_mod::cont_circ::ContCircSystem;
        use crate::target_mod::cont_bulk::ContBulkTarget;
        use crate::random_mod::get_uniform_to_vec_nonstandard;


        let mut rng1 = rng_seed(12341234);
        let mut rng2 = rng_seed(12341234);

        let system = ContCircSystem::new(10.0, 2);
        let target = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 1.0);

        let searcher1 = ContPassiveIndepSearcher::new_uniform(&system, &target, &mut rng1,
                        MoveType::Brownian(1f64));

        let mut pos = system.position_out_of_system();
        while !system.check_inclusion(&pos)? || target.check_find(&pos)?{
            pos.clear();
            get_uniform_to_vec_nonstandard(&mut rng2, &mut pos, -10.0, 10.0);
        }

        assert_eq!(searcher1?, ContPassiveIndepSearcher{
            searcher_type : SearcherType::ContinuousPassiveIndependent,
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::Uniform,
            dim     : 2,
            pos     : pos.clone(),
        });

        Ok(())
    }
}
