// Module for system configuration
//
// System should be defined with size, dimension, shape, discreteness
// It provides function a 'check_inclusion' which check whether a position vector v is in the system.the
//

use std::fmt;
use crate::error::Error;
use crate::position::Position;
use rand_pcg::Pcg64;

pub mod cont_circ;


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SystemType{                        // System type
    ContinousCircular,
    ContinousRectangular,
    Lattice,
    Network,
}

impl fmt::Display for SystemType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            SystemType::ContinousCircular => write!(f, "Continous Circular system."),
            SystemType::ContinousRectangular => write!(f, "Continous Rectangular system."),
            SystemType::Lattice => write!(f, "Lattice system."),
            SystemType::Network => write!(f, "Network system."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum BoundaryCond{                      // Boundary condition
    Periodic,                               // Only valid for Rectanuglar system or Lattice
    Reflection,
}

impl fmt::Display for BoundaryCond{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            BoundaryCond::Periodic => write!(f, "Periodic Boundary Condition"),
            BoundaryCond::Reflection => write!(f, "Reflective Boundary Condtion"),
        }
    }
}

pub trait SystemCore<T>{
    // Return whether a position vector is in the system
    fn check_inclusion(&self, pos: &Position<T>) -> Result<bool, Error>;

    // Boundary condition을 확인하고, 그에 맞는 위치를 다시 반환.
    // pos : 원래 위치, dp : 미소변위, 결과물 : bc 만족하도록 하는 새 위치
    // Error : 변위가 너무 커서 bc를 계산하는 것이 의미가 없어지는 경우.
    fn check_bc(&self, pos: &mut Position<T>, dp: &mut Position<T>) -> Result<(), Error>;

    // System 내부의 임의의 위치를 uniform하게 뽑아 반환
    fn random_pos(&self, rng: &mut Pcg64) -> Result<Position<T>, Error>;

    // System 내부의 임의의 위치를 uniform하게 뽑아서 mutable reference에 기입
    fn random_pos_to_vec(&self, rng: &mut Pcg64, vec: &mut Position<T>) -> Result<(), Error>;

    // system 밖의 점을 하나 출력해주는 함수
    fn position_out_of_system(&self) -> Position<T>;

    // system 밖의 점을 하나 vector에 적어주는 함수
    fn position_out_of_system_to_vec(&self, vec: &mut Position<T>) -> Result<(), Error>;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt(){
        assert_eq!(format!("{}", SystemType::ContinousCircular).as_str(), "Continous Circular system.");
        assert_eq!(format!("{}", SystemType::ContinousRectangular).as_str(), "Continous Rectangular system.");
        assert_eq!(format!("{}", SystemType::Lattice).as_str(), "Lattice system.");
        assert_eq!(format!("{}", SystemType::Network).as_str(), "Network system.");

        assert_eq!(format!("{}", BoundaryCond::Periodic).as_str(), "Periodic Boundary Condition");
        assert_eq!(format!("{}", BoundaryCond::Reflection).as_str(), "Reflective Boundary Condtion");
    }
}


