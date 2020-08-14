// Module for Continous Passive Independent Searcher

use crate::prelude::*;
use crate::searcher_mod::{Passive, Merge};
use crate::random_mod::{get_gaussian_vec, get_gaussian_to_vec_nonstandard};



#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContPassiveMergeSearcher{            // 연속한 시스템에서 Passive하게 움직이는 mergeable searcher
    pub searcher_type : SearcherType,           // Type of searcher
    pub mtype   : MoveType,                     // Type of random movement
    pub itype   : InitType<f64>,                // Type of Initialization
    pub dim     : usize,                        // dimension of space containing searcher
    pub pos     : Position<f64>,                // position of searcher
    pub radius  : f64,                          // radius of ptl. when they collide, they merge.
    pub size    : usize,                        // size of cluster
    pub alpha   : f64,                          // Exponent of diffusion decrease a function of size
}

impl ContPassiveMergeSearcher{
    // 모든 정보를 제공했을 경우, 새 Searcher struct를 반환하는 함수
    pub fn new(mtype : MoveType, pos : Position<f64>, radius : f64, alpha : f64) -> Self{
        // mtype    : Random walk characteristic
        // pos      : initial position of searcher
        // radius   : Radius of ptl
        // size     : size of ptl
        // alpha    : exponent of diffusion decrease

        ContPassiveMergeSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            mtype   : mtype,
            itype   : InitType::SpecificPosition(pos.clone()),
            dim     : pos.dim(),
            pos     : pos,
            radius  : radius,
            size    : 1,
            alpha   : alpha,
        }
    }

    pub fn new_uniform(sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64, mtype : MoveType, radius : f64, alpha : f64) -> Result<Self, Error>{
        // system과 target이 주어져 있는 상황에서 시스템 domain 안에서 초기위치를 uniform하게 뽑아 searcher를 정의해주는 함수
        // sys      : system configuration
        // target   : target configuration
        // rng      : random number generator
        // mtype    : random walk characteristic
        // size     : size of ptl
        // alpha    : exponent of diffusion decrease

        let mut pos : Position<f64> = sys.position_out_of_system();  // 초기값을 위해 무조건 시스템 밖의 벡터를 받도록 한다
        loop{
            sys.random_pos_to_vec(rng, &mut pos)?;   // System 내부의 random position을 받는다
            if !target.check_find(&pos)?{            // 그 random position이 target과 이미 만났는가 확인
                break;
            }
        }

        Ok(ContPassiveMergeSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            mtype   : mtype,
            itype   : InitType::Uniform,
            dim     : pos.dim(),
            pos     : pos,
            radius  : radius,
            size    : 1,
            alpha   : alpha,
        })
    }

    pub fn renew_uniform(&mut self, sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64) -> Result<(), Error>{
        // 매번 searcher를 새로 정의하는 것 역시 상당한 memory 낭비이다.
        // 있는 searcher를 재활용하도록 하자.
        // independent searcher와 다르게 mergeable searcher는 size도 변할 수 있고, diffusion coefficient도 변한다.
        // 이들을 모두 바꿔줘야함

        sys.position_out_of_system_to_vec(&mut self.pos)?;
        loop{
            sys.random_pos_to_vec(rng, &mut self.pos)?;   // System 내부의 random position을 받는다
            if !target.check_find(&self.pos)?{            // 그 random position이 target과 이미 만났는가 확인
                break;
            }
        }

        self.size = 1;
        match self.mtype{
            MoveType::Brownian(_c) =>{
                let coeff = (self.size as f64).powf(-self.alpha);
                self.mtype = MoveType::Brownian(coeff);
            },
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::FeatureNotProvided));
            }
        }

        Ok(())
    }
}

impl_argument_trait!(ContPassiveMergeSearcher, "Searcher", ContPassiveMergeSearcherArguments, 5,
    searcher_type, SearcherType, SearcherType::ContinuousPassiveInteracting,
    size,   usize,          1;
    mtype,  MoveType,       "Random walk Characterstic. ex) 1.0 : Brownian with D=1 / Levy : Levy walk",
    itype,  InitType<f64>,  "Initialization method. ex) 0,0 : All at 0,0 / Uniform : Uniform",
    radius, f64,            "Radius of particle. When they collide, they merge. ex) 0.1",
    alpha,  f64,            "Exponent of diffusion decrease. D ~ n^alpha ex) 1.0",
    num_searcher, usize, "Number of Searcher");

impl ContPassiveMergeSearcher{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContPassiveMergeSearcherArguments) -> Vec<Self>{
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
            radius          : argument.radius,
            size            : 1,
            alpha           : argument.alpha,
        }; argument.num_searcher]
    }
}

impl SearcherCore<f64> for ContPassiveMergeSearcher{
    fn pos(&self) -> &Position<f64>{
        &self.pos
    }
}

impl Passive<f64> for ContPassiveMergeSearcher{
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

impl Merge for ContPassiveMergeSearcher{
    fn merge(&mut self, other : &Self) -> Result<(), Error>{
        self.size = self.size + other.size;
        match self.mtype{
            MoveType::Brownian(_c) =>{
                let coeff = (self.size as f64).powf(-self.alpha);
                self.mtype = MoveType::Brownian(coeff);
                Ok(())
            },
            _ => {
                Err(Error::make_error_syntax(ErrorCode::FeatureNotProvided))
            }
        }
    }

    fn size(&self) -> usize{
        self.size
    }

    fn add_size(&mut self, size : usize) -> Result<(), Error>{
        self.size = self.size + size;
        match self.mtype{
            MoveType::Brownian(_c) =>{
                let coeff = (self.size as f64).powf(-self.alpha);
                self.mtype = MoveType::Brownian(coeff);
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
        let searcher1 = ContPassiveMergeSearcher::new(MoveType::Brownian(1f64),
                        pos.clone(), 0.1, 0.0);
        assert_eq!(searcher1, ContPassiveMergeSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::SpecificPosition(pos.clone()),
            dim     : 2,
            pos     : pos.clone(),
            radius  : 0.1,
            size    : 1,
            alpha   : 0.0,
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

        let searcher1 = ContPassiveMergeSearcher::new_uniform(&system, &target, &mut rng1,
            MoveType::Brownian(1f64), 0.1, 0.0);

        let mut pos = system.position_out_of_system();
        while !system.check_inclusion(&pos)? || target.check_find(&pos)?{
            pos.clear();
            get_uniform_to_vec_nonstandard(&mut rng2, &mut pos, -10.0, 10.0);
        }

        assert_eq!(searcher1?, ContPassiveMergeSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::Uniform,
            dim     : 2,
            pos     : pos.clone(),
            radius  : 0.1,
            size    : 1,
            alpha   : 0.0,
        });

        Ok(())
    }

    #[test]
    fn test_merge() -> Result<(), Error>{
        let pos = Position::<f64>::new(vec![0.0, 0.0]);
        let mut searcher1 = ContPassiveMergeSearcher::new(MoveType::Brownian(1f64),
                        pos.clone(), 0.1, 0.0);
        let mut searcher2 = ContPassiveMergeSearcher::new(MoveType::Brownian(1f64),
                        pos.clone(), 0.1, 1.0);

        searcher1.merge(&searcher2)?;
        searcher2.merge(&searcher1)?;

        assert_eq!(searcher1.size, 2);
        assert_eq!(searcher1.mtype, MoveType::Brownian(1f64));

        assert_eq!(searcher2.size, 3);
        assert_eq!(searcher2.mtype, MoveType::Brownian(1.0 / 3.0));

        searcher1.merge(&searcher2)?;
        searcher2.merge(&searcher1)?;

        assert_eq!(searcher1.size, 5);
        assert_eq!(searcher1.mtype, MoveType::Brownian(1f64));

        assert_eq!(searcher2.size, 8);
        assert_eq!(searcher2.mtype, MoveType::Brownian(0.125f64));

        Ok(())
    }
}
