// Module for target configuration
//
// Target should be defined with size, position (at bulk, or at boundary)
// It provides function a 'check_find' which check whether a position vector v is near the target
//

use std::fmt::{Display, Formatter, self};
use crate::error::Error;
use crate::position::Position;

pub mod cont_bulk;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TargetType{
    ContinousInBulk,
    ContinousAtBoundary,
    LatticeInBulk,
    LatticeAtBoundary,
    NetworkSingleNode,
}

impl Display for TargetType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            TargetType::ContinousInBulk => {
                write!(f, "Target in Bulk of Continous System.")
            }
            TargetType::ContinousAtBoundary => {
                write!(f, "Target at Boundary of Continous System.")
            }
            TargetType::LatticeInBulk => {
                write!(f, "Target in Bulk of Lattice System.")
            }
            TargetType::LatticeAtBoundary => {
                write!(f, "Target at Boundary of Lattice System.")
            }
            TargetType::NetworkSingleNode => {
                write!(f, "Target is a Single Node in Network.")
            }
        }
    }
}

pub trait TargetCore<T>{
    // Return the type of target
    fn ttype(&self) -> TargetType;

    // Check whether a position is near at target
    fn check_find(&self, pos: &Position<T>) -> Result<bool, Error>;
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt(){
        assert_eq!(format!("{}", TargetType::ContinousInBulk).as_str(),
            "Target in Bulk of Continous System.");
        assert_eq!(format!("{}", TargetType::ContinousAtBoundary).as_str(),
            "Target at Boundary of Continous System.");
        assert_eq!(format!("{}", TargetType::LatticeInBulk).as_str(),
            "Target in Bulk of Lattice System.");
        assert_eq!(format!("{}", TargetType::LatticeAtBoundary).as_str(),
            "Target at Boundary of Lattice System.");
        assert_eq!(format!("{}", TargetType::NetworkSingleNode).as_str(),
            "Target is a Single Node in Network.");
    }
}








