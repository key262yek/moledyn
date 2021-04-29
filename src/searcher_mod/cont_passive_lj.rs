// Module for Continous Passive Independent Searcher

use crate::prelude::*;
use crate::searcher_mod::{Passive, Interaction};
use crate::random_mod::{get_gaussian_vec, get_gaussian_to_vec_nonstandard};




#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContPassiveLJSearcher{            // 연속한 시스템에서 Passive하게 움직이는 mergeable searcher
    pub searcher_type : SearcherType,           // Type of searcher
    pub int_type: InteractType,                 // Type of interaction
    pub mtype   : MoveType,                     // Type of random movement
    pub itype   : InitType<f64>,                // Type of Initialization
    pub dim     : usize,                        // Dimension of system contains searcher
    pub pos     : Position<f64>,                // position of searcher
    pub ptl_size: f64,                          // particle size of searcher, should be a order of 1
    pub strength: f64,                          // strength of interaction
    pub coeff_pot   : f64,                      // coefficient of potential
    pub coeff_force : f64,                      // coefficient of force
}

impl ContPassiveLJSearcher{
    fn coeff(ptl_size : f64, strength : f64) -> Result<(f64, f64), Error>{
        let coeff_pot = 4f64 * strength;
        let coeff_force = 24f64 * strength / ptl_size;

        return Ok((coeff_pot, coeff_force));
    }

    // 모든 정보를 제공했을 경우, 새 Searcher struct를 반환하는 함수
    pub fn new(int_type : InteractType, mtype : MoveType, pos : Position<f64>, strength : f64) -> Self{
        // int_type : Interaction Type
        // mtype    : Random walk characteristic
        // pos      : initial position of searcher

        match int_type{
            InteractType::LennardJones(ptl_size) => {
                let (coeff_pot, coeff_force) = Self::coeff(ptl_size, strength).expect("Invalid Argument Input");
                let dim = pos.dim();

                ContPassiveLJSearcher{
                    searcher_type : SearcherType::ContinuousPassiveInteracting,
                    int_type: int_type,
                    mtype   : mtype,
                    itype   : InitType::SpecificPosition(pos.clone()),
                    dim     : dim,
                    pos     : pos,
                    ptl_size: ptl_size,
                    strength: strength,
                    coeff_pot : coeff_pot,
                    coeff_force : coeff_force,
                }
            },
            _ =>{
                panic!("Invalid Argument Input to Searcher Definition");
            }
        }
    }

    pub fn new_uniform(sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64, int_type : InteractType, mtype : MoveType, strength : f64) -> Result<Self, Error>{
        // system과 target이 주어져 있는 상황에서 시스템 domain 안에서 초기위치를 uniform하게 뽑아 searcher를 정의해주는 함수
        // sys      : system configuration
        // target   : target configuration
        // rng      : random number generator
        // int_type : interaction type
        // mtype    : random walk characteristic


        match int_type{
            InteractType::LennardJones(ptl_size) => {
                let (coeff_pot, coeff_force) = Self::coeff(ptl_size, strength)?;

                let mut pos : Position<f64> = sys.position_out_of_system();  // 초기값을 위해 무조건 시스템 밖의 벡터를 받도록 한다
                let dim = pos.dim();
                loop{
                    sys.random_pos_to_vec(rng, &mut pos)?;   // System 내부의 random position을 받는다
                    if !target.check_find(&pos)?{            // 그 random position이 target과 이미 만났는가 확인
                        break;
                    }
                }

                Ok(ContPassiveLJSearcher{
                    searcher_type : SearcherType::ContinuousPassiveInteracting,
                    int_type: int_type,
                    mtype   : mtype,
                    itype   : InitType::Uniform,
                    dim     : dim,
                    pos     : pos,
                    ptl_size:ptl_size,
                    strength: strength,
                    coeff_pot : coeff_pot,
                    coeff_force : coeff_force,
                })
            },
            _ =>{
                Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
            }
        }
    }

    pub fn renew_uniform(&mut self, sys : &dyn SystemCore<f64>, target : &dyn TargetCore<f64>,
                   rng : &mut Pcg64) -> Result<(), Error>{
        // 매번 searcher를 새로 정의하는 것 역시 상당한 memory 낭비이다.
        // 있는 searcher를 재활용하도록 하자.
        // independent searcher와 다르게 mergeable searcher는 size도 변할 수 있고, diffusion coefficient도 변한다.
        // 이들을 모두 바꿔줘야함

        match sys.position_out_of_system_to_vec(&mut self.pos){
            Ok(()) => (),
            Err(_) => {
                self.pos = sys.position_out_of_system();
                self.dim = self.pos.dim();

                let (coeff_pot, coeff_force) = Self::coeff(self.ptl_size, self.strength)?;
                self.coeff_pot = coeff_pot;
                self.coeff_force = coeff_force;
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

impl_argument_trait!(ContPassiveLJSearcher, "Searcher", ContPassiveLJSearcherArguments, 5,
    searcher_type, SearcherType, SearcherType::ContinuousPassiveInteracting;
    mtype,  MoveType,       "Random walk Characterstic. ex) 1.0 : Brownian with D=1 / Levy : Levy walk",
    itype,  InitType<f64>,  "Initialization method. ex) 0,0 : All at 0,0 / Uniform : Uniform",
    ptl_size, f64,          "Particle size of searcher. Should be an order of 1 ex) 1.0, 2.0",
    strength, f64,          "Strength of interaction",
    num_searcher, usize,    "Number of Searcher");

impl ContPassiveLJSearcher{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContPassiveLJSearcherArguments) -> Vec<Self>{
        let dim : usize;
        let pos : Position<f64>;
        let ptl_size : f64;
        let strength : f64 = argument.strength;

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

        ptl_size = argument.ptl_size;

        let (coeff_pot, coeff_force) = Self::coeff(ptl_size, strength).expect("Invalid Argument Input");

        vec![Self{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            int_type: InteractType::LennardJones(ptl_size),
            mtype   : argument.mtype,
            itype   : argument.itype.clone(),
            dim     : dim,
            pos     : pos,
            ptl_size:ptl_size,
            strength: strength,
            coeff_pot : coeff_pot,
            coeff_force : coeff_force,
        }; argument.num_searcher]
    }
}

impl SearcherCore<f64> for ContPassiveLJSearcher{
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
        vec.clear();
        vec.mut_add(&other.pos)?;
        vec.mut_sub(&self.pos)?;
        let distance : f64 = vec.norm();
        vec.mut_scalar_mul(1f64 / distance);
        return Ok(distance);
    }

    fn mutual_distance(&self, other : &Self) -> Result<f64, Error>{
        self.pos().distance(other.pos())
    }
}

impl Passive<f64, f64> for ContPassiveLJSearcher{
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

impl Interaction<f64, f64> for ContPassiveLJSearcher{
    fn potential(&self, r : f64) -> f64{
        let x = self.ptl_size / r;
        let x6 = x.powi(6);

        self.coeff_pot * x6 * (x6 - 1f64)
    }

    fn force(&self, r : f64) -> f64{
        let x = self.ptl_size / r;
        let x6 = x.powi(6);

        self.coeff_force * x6 * x * (2f64 * x6 - 1f64)
    }
}



#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new(){
        let pos = Position::<f64>::new(vec![0.0, 0.0]);
        let searcher1 = ContPassiveLJSearcher::new(InteractType::LennardJones(1f64),
            MoveType::Brownian(1f64), pos.clone(), 0f64);
        assert_eq!(searcher1, ContPassiveLJSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            int_type: InteractType::LennardJones(1f64),
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::SpecificPosition(pos.clone()),
            dim     : 2,
            pos     : pos.clone(),
            ptl_size   : 1f64,
            strength: 0f64,
            coeff_pot: 0f64,
            coeff_force:0f64,
        });
    }

    #[test]
    #[should_panic]
    fn test_invalid_argument(){
        let pos = Position::<f64>::new(vec![0.0]);
        let _searcher1 = ContPassiveLJSearcher::new(InteractType::Exponential(1, 2f64),
            MoveType::Brownian(1f64), pos.clone(), 0f64);
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

        let searcher1 = ContPassiveLJSearcher::new_uniform(&system, &target, &mut rng1,
            InteractType::LennardJones(1f64), MoveType::Brownian(1f64), 0f64);

        let mut pos = system.position_out_of_system();
        while !system.check_inclusion(&pos)? || target.check_find(&pos)?{
            pos.clear();
            get_uniform_to_vec_nonstandard(&mut rng2, &mut pos, -10.0, 10.0);
        }

        assert_eq!(searcher1?, ContPassiveLJSearcher{
            searcher_type : SearcherType::ContinuousPassiveInteracting,
            int_type: InteractType::LennardJones(1f64),
            mtype   : MoveType::Brownian(1f64),
            itype   : InitType::Uniform,
            dim     : 2,
            pos     : pos.clone(),
            ptl_size   : 1f64,
            strength: 0f64,
            coeff_pot: 0f64,
            coeff_force:0f64,
        });

        Ok(())
    }

    #[test]
    fn test_interaction_trait() -> Result<(), Error>{
        let int_type = InteractType::LennardJones(1f64);
        let mtype = MoveType::Brownian(1f64);

        let mut searcher1 = ContPassiveLJSearcher::new(int_type, mtype, Position::<f64>::new(vec![0.0, 0.0]), 0f64);
        let searcher2 = ContPassiveLJSearcher::new(int_type, mtype, Position::<f64>::new(vec![2.5, 0.0]), 0f64);

        let test1 = searcher1.mutual_displacement(&searcher2)?;
        assert_eq!(test1, (Position::<f64>::new(vec![1.0, 0.0]), 2.5));

        let mut vec = Position::<f64>::new(vec![0.0, 0.0]);
        let test2 = searcher1.mutual_displacement_to_vec(&searcher2, &mut vec)?;
        assert_eq!(vec, Position::<f64>::new(vec![1.0, 0.0]));
        assert_eq!(test2, 2.5);

        let (force, dt) = (1f64, 0.5f64);
        let mut temp = Position::<f64>::new(vec![0.0, 0.0]);
        vec.mut_scalar_mul(force * dt);
        temp.mut_add(&vec)?;
        assert_eq!(temp, Position::<f64>::new(vec![0.5, 0.0]));
        searcher1.pos.mut_add(&temp)?;
        assert_eq!(searcher1.pos, Position::<f64>::new(vec![0.5,0.0]));

        return Ok(());
    }
}
