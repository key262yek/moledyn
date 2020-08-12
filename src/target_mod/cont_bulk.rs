// Module for target in bulk of continous system

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContBulkTarget{
    pub target_type : TargetType,
    pub target_pos : Position::<f64>,
    pub target_size : f64,
}

impl ContBulkTarget{
    // Generate Target
    pub fn new(pos : Position::<f64>, r : f64) -> ContBulkTarget{
        ContBulkTarget{
            target_type : TargetType::ContinuousInBulk,
            target_pos : pos,
            target_size : r,
        }
    }

    // Distance between target and given position
    pub fn distance(&self, other_pos: &Position<f64>) -> Result<f64, Error>{
        self.target_pos.distance(other_pos)
    }
}

impl_argument_trait!(ContBulkTarget, "Target", ContBulkTargetArguments, 2,
    target_type, TargetType, TargetType::ContinuousInBulk;
    target_pos, Position::<f64>, "Position of Target",
    target_size, f64, "Size of Target");

impl Convert<ContBulkTargetArguments> for ContBulkTarget{
    fn convert_from(argument : &ContBulkTargetArguments) -> Self{
        Self{
            target_type : argument.target_type,
            target_pos  : argument.target_pos.clone(),
            target_size : argument.target_size,
        }
    }
}


impl TargetCore<f64> for ContBulkTarget{
    // Return the type of target
    fn target_type(&self) -> TargetType{
        self.target_type.clone()
    }

    // Check whether a searcher finds the target
    fn check_find(&self, pos: &Position<f64>) -> Result<bool, Error>{
        let d = self.distance(pos)?;
        let rad : f64 = self.target_size;

        if d < rad{
            return Ok(true);
        }
        return Ok(false);
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::error::ErrorCode;

    #[test]
    fn test_pos(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.target_pos, Position::<f64>::new(vec![0.0, 0.0]));
    }

    #[test]
    fn test_radius(){
        let target : ContBulkTarget = ContBulkTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.target_size, 3.0);
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
        assert_eq!(target.target_type(), TargetType::ContinuousInBulk);
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
