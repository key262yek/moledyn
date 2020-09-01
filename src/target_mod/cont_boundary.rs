// Module for target at boundary of continous system
// To do
// boundary surface의 형태에 따라 target의 모양도 바뀐다.
// 이를 잘 구현할 방법이 필요.

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContBoundaryTarget{
    pub target_type : TargetType,
    pub target_pos : Position::<f64>,
    pub target_size : f64,
}

impl ContBoundaryTarget{
    // Generate Target
    pub fn new(pos : Position::<f64>, r : f64) -> ContBoundaryTarget{
        ContBoundaryTarget{
            target_type : TargetType::ContinuousAtBoundary,
            target_pos : pos,
            target_size : r,
        }
    }

    // Distance between target and given position
    pub fn distance(&self, other_pos: &Position<f64>) -> Result<f64, Error>{
        self.target_pos.distance(other_pos)
    }
}

impl_argument_trait!(ContBoundaryTarget, "Target", ContBoundaryTargetArguments, 2,
    target_type, TargetType, TargetType::ContinuousInBulk;
    target_pos, Position::<f64>, "Position of Target ex) 0:0 = (0,0), 1.0:2.0 = (1.0, 2.0)",
    target_size, f64, "Size of Target");

impl ContBoundaryTarget{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ContBoundaryTargetArguments) -> Self{
        Self{
            target_type : argument.target_type,
            target_pos  : argument.target_pos.clone(),
            target_size : argument.target_size,
        }
    }
}


impl TargetCore<f64> for ContBoundaryTarget{
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
        let target : ContBoundaryTarget = ContBoundaryTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.target_pos, Position::<f64>::new(vec![0.0, 0.0]));
    }

    #[test]
    fn test_radius(){
        let target : ContBoundaryTarget = ContBoundaryTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.target_size, 3.0);
    }

    #[test]
    fn test_distance(){
        let target : ContBoundaryTarget = ContBoundaryTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        let pos : Position<f64> = Position::<f64>::new(vec![3.0, 4.0]);

        assert_eq!(target.distance(&pos), Ok(5.0));
    }

    #[test]
    fn test_ttype(){
        let target : ContBoundaryTarget = ContBoundaryTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        assert_eq!(target.target_type(), TargetType::ContinuousInBulk);
    }

    #[test]
    fn test_check_find(){
        let target : ContBoundaryTarget = ContBoundaryTarget::new(Position::<f64>::new(vec![0.0, 0.0]), 3.0);
        let pos : Position<f64> = Position::<f64>::new(vec![3.0, 4.0]);
        let pos2 : Position<f64> = Position::<f64>::new(vec![1.0, 2.0]);
        let pos3 : Position<f64> = Position::<f64>::new(vec![1.0, 2.0, 3.0]);

        assert_eq!(target.check_find(&pos), Ok(false));
        assert_eq!(target.check_find(&pos2), Ok(true));
        assert_eq!(target.check_find(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }
}
