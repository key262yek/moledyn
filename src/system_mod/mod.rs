// Module for system configuration
//
// System should be defined with size, dimension, shape, discreteness
// It provides function a 'check_inclusion' which check whether a position vector v is in the system.the
//

use crate::prelude::*;

// =====================================================================================
// ===  Implement System ===============================================================
// =====================================================================================

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

pub mod cont_circ;
pub mod cont_cubic;
pub mod cont_cyl;


// =====================================================================================
// ===  Implement SystemType ===========================================================
// =====================================================================================


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SystemType{                        // System type
    ContinuousCircular,
    ContinuousRectangular,
    ContinuousCylindrical(usize),
    Lattice,
    Network,
}

// Formatting
impl Display for SystemType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            SystemType::ContinuousCircular => write!(f, "Continuous Circular system."),
            SystemType::ContinuousRectangular => write!(f, "Continuous Rectangular system."),
            SystemType::ContinuousCylindrical(d) => write!(f, "Continuous Cylindrical system. 0..{0:} : Circular, {0:}.. : Rectangular", d),
            SystemType::Lattice => write!(f, "Lattice system."),
            SystemType::Network => write!(f, "Network system."),
        }
    }
}

// From String to Type
impl FromStr for SystemType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        match split[0]{
            "Continuous" => {
                match split[1]{
                    "Circular" => Ok(SystemType::ContinuousCircular),
                    "Rectangular" => Ok(SystemType::ContinuousRectangular),
                    "Cylindrical" => {
                        let d = split[3][3..].parse::<usize>().map_err(|_y| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                        Ok(SystemType::ContinuousCylindrical(d))
                    }
                    _ => Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput)),
                }
            },
            "Lattice" => Ok(SystemType::Lattice),
            "Network" => Ok(SystemType::Network),
            "Circular" => Ok(SystemType::ContinuousCircular),
            "Rectangular" => Ok(SystemType::ContinuousRectangular),
            string =>{
                let d = string.parse::<usize>().map_err(|_y| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                if d <= 0{
                    Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
                else{
                    Ok(SystemType::ContinuousCylindrical(d))
                }
            }
        }
    }
}

impl Default for SystemType{
    fn default() -> Self{
        SystemType::ContinuousCircular
    }
}


// =====================================================================================
// ===  Implement BoundaryCond =========================================================
// =====================================================================================


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum BoundaryCond{                      // Boundary condition
    Periodic,                               // Only valid for Rectanuglar system or Lattice
    Reflection,                             //
    Mixed(usize),                           // Cylindrical system 등에서 각 dimension마다 boundary condition이 다를 수도 있다
}

// Formatting
impl Display for BoundaryCond{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            BoundaryCond::Periodic => write!(f, "Periodic Boundary Condition"),
            BoundaryCond::Reflection => write!(f, "Reflective Boundary Condtion"),
            BoundaryCond::Mixed(dim) => write!(f, "Mixed Boundary Condition. 0..{0:} : Reflection, {0:}.. : Periodic", dim),
        }
    }
}



impl FromStr for BoundaryCond{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        match split[0]{
            "Reflective"    => Ok(BoundaryCond::Reflection),
            "Periodic"      => Ok(BoundaryCond::Periodic),
            "Mixed"         => {
                let x = split[3][3..].parse::<usize>().map_err(|_y| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                Ok(BoundaryCond::Mixed(x))
            },
            string => {
                let x = string.parse::<usize>().map_err(|_y| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                if x <= 0{
                    Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
                else{
                    Ok(BoundaryCond::Mixed(x))
                }
            },
        }
    }
}
impl Default for BoundaryCond{
    fn default() -> Self{
        BoundaryCond::Reflection
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::{impl_fmt_test, impl_fromstr_test};

    impl_fmt_test!(test_fmt_systemtype,
        SystemType::ContinuousCircular => "Continuous Circular system.",
        SystemType::ContinuousRectangular => "Continuous Rectangular system.",
        SystemType::ContinuousCylindrical(1) => "Continuous Cylindrical system. 0..1 : Circular, 1.. : Rectangular",
        SystemType::Lattice => "Lattice system.",
        SystemType::Network => "Network system.");

    impl_fmt_test!(test_fmt_boundarycond,
        BoundaryCond::Periodic => "Periodic Boundary Condition",
        BoundaryCond::Reflection => "Reflective Boundary Condtion",
        BoundaryCond::Mixed(1) => "Mixed Boundary Condition. 0..1 : Reflection, 1.. : Periodic");

    impl_fromstr_test!(test_fromstr_systemtype,
        SystemType,
        SystemType::ContinuousCircular => "Continuous Circular system.",
        SystemType::ContinuousRectangular => "Continuous Rectangular system.",
        SystemType::ContinuousCylindrical(1) => "Continuous Cylindrical system. 0..1 : Circular, 1.. : Rectangular",
        SystemType::Lattice => "Lattice system.",
        SystemType::Network => "Network system.");

    impl_fromstr_test!(test_fromstr_boundarycond,
        BoundaryCond,
        BoundaryCond::Periodic => "Periodic Boundary Condition",
        BoundaryCond::Reflection => "Reflective Boundary Condtion",
        BoundaryCond::Mixed(1) => "Mixed Boundary Condition. 0..1 : Reflection, 1.. : Periodic");
}


