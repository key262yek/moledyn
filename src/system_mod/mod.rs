// Module for system configuration
//
// System should be defined with size, dimension, shape, discreteness
// It provides function a 'check_inclusion' which check whether a position vector v is in the system.the
//

use crate::prelude::*;

pub mod cont_circ;


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum SystemType{                        // System type
    ContinuousCircular,
    ContinuousRectangular,
    Lattice,
    Network,
}

// Formatting
impl_fmt_for_type!(SystemType,
    SystemType::ContinuousCircular => "Continuous Circular system.",
    SystemType::ContinuousRectangular => "Continuous Rectangular system.",
    SystemType::Lattice => "Lattice system.",
    SystemType::Network => "Network system.");

// From String to Type
impl_fromstr_for_type!(SystemType,
    SystemType::ContinuousCircular => "Continuous Circular system.",
    SystemType::ContinuousRectangular => "Continuous Rectangular system.",
    SystemType::Lattice => "Lattice system.",
    SystemType::Network => "Network system.");


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum BoundaryCond{                      // Boundary condition
    Periodic,                               // Only valid for Rectanuglar system or Lattice
    Reflection,
}

// Formatting
impl_fmt_for_type!(BoundaryCond,
    BoundaryCond::Periodic => "Periodic Boundary Condition",
    BoundaryCond::Reflection => "Reflective Boundary Condtion");

// From String to Type
impl_fromstr_for_type!(BoundaryCond,
    BoundaryCond::Periodic => "Periodic Boundary Condition",
    BoundaryCond::Reflection => "Reflective Boundary Condtion");

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
    use crate::{impl_fmt_test, impl_fromstr_test};

    impl_fmt_test!(test_fmt_systemtype,
        SystemType::ContinuousCircular => "Continuous Circular system.",
        SystemType::ContinuousRectangular => "Continuous Rectangular system.",
        SystemType::Lattice => "Lattice system.",
        SystemType::Network => "Network system.");

    impl_fmt_test!(test_fmt_boundarycond,
        BoundaryCond::Periodic => "Periodic Boundary Condition",
        BoundaryCond::Reflection => "Reflective Boundary Condtion");

    impl_fromstr_test!(test_fromstr_systemtype,
        SystemType,
        SystemType::ContinuousCircular => "Continuous Circular system.",
        SystemType::ContinuousRectangular => "Continuous Rectangular system.",
        SystemType::Lattice => "Lattice system.",
        SystemType::Network => "Network system.");

    impl_fromstr_test!(test_fromstr_boundarycond,
        BoundaryCond,
        BoundaryCond::Periodic => "Periodic Boundary Condition",
        BoundaryCond::Reflection => "Reflective Boundary Condtion");
}


