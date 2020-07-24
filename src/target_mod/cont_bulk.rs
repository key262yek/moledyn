// Module for target in bulk of continous system

use crate::error::{Error};
use crate::target_mod::{TargetType, TargetCore};
use crate::position::Position;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContBulkTarget{
    pub ttype : TargetType,
    pub pos : Position::<f64>,
    pub radius : f64,
}

impl ContBulkTarget{
    // Generate Target
    pub fn new(pos : Position::<f64>, radius : f64) -> ContBulkTarget{
        ContBulkTarget{
            ttype : TargetType::ContinousInBulk,
            pos : pos,
            radius : radius,
        }
    }

    // Distance between target and given position
    pub fn distance(&self, other_pos: &Position<f64>) -> Result<f64, Error>{
        self.pos.distance(other_pos)
    }
}

impl TargetCore<f64> for ContBulkTarget{
    // Return the type of target
    fn ttype(&self) -> TargetType{
        self.ttype.clone()
    }

    // Check whether a position is near at target
    fn check_find(&self, pos: &Position<f64>) -> Result<bool, Error>{
        let d = self.distance(pos)?;
        let rad : f64 = self.radius;

        if d < rad{
            return Ok(true);
        }
        return Ok(false);
    }
}

impl Display for ContBulkTarget{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        write!(f, "{}\nPos : ({}), Radius : {}", self.ttype, self.pos, self.radius)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(format!("{}", target).as_str(),
            "Target in Bulk of Continous System.\nPos : (0, 0), Radius : 3");
    }

    #[test]
    fn test_pos(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.pos, Position::<f64>::new(vec![0.0, 0.0]));
    }

    #[test]
    fn test_radius(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.radius, 3.0);
    }

    #[test]
    fn test_distance(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        let pos : Position<f64> = Position::<f64>::new(vec![3.0, 4.0]);

        assert_eq!(target.distance(&pos), Ok(5.0));
    }

    #[test]
    fn test_ttype(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.ttype(), TargetType::ContinousInBulk);
    }

    #[test]
    fn test_check_find(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        let pos : Position<f64> = Position::<f64>::new(vec![3.0, 4.0]);
        let pos2 : Position<f64> = Position::<f64>::new(vec![1.0, 2.0]);
        let pos3 : Position<f64> = Position::<f64>::new(vec![1.0, 2.0, 3.0]);

        assert_eq!(target.check_find(&pos), Ok(false));
        assert_eq!(target.check_find(&pos2), Ok(true));
        assert_eq!(target.check_find(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }
}
