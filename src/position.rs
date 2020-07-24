// Module for Position

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use std::fmt::{self, Display, Write, Formatter, LowerExp};
use crate::error::{Error, ErrorCode};
use std::default::Default;


#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Position<T>{
    pub coordinate : Vec<T>
}

impl<T : Clone> Position<T>{
    pub fn new(vec : Vec<T>) -> Self{
        Position::<T>{
            coordinate : vec.clone()
        }
    }
}

impl<T> Position<T>{
    pub fn dim(&self) -> usize{
        self.coordinate.len()
    }
}

impl<T : Default> Position<T>{
    pub fn clear(&mut self){
        for x in &mut self.coordinate{
            *x = Default::default();
        }
    }
}

impl Position<f64>{
    pub fn norm(&self) -> f64{
        let mut res : f64 = 0f64;
        for &x in self.coordinate.iter(){
            res += x * x;
        }
        return res.sqrt();
    }

    pub fn distance(&self, other : &Self) -> Result<f64, Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }
        let mut r : f64 = 0.0f64;
        for i in 0..self.dim(){
            let x : f64 = self.coordinate[i];
            let y : f64 = other.coordinate[i];
            r += (x - y) *  (x - y);
        }
        return Ok(r.sqrt());
    }

    pub fn inner_product(&self, other : &Self) -> Result<f64, Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut res : f64 = 0.0;
        for (i, &x) in self.coordinate.iter().enumerate(){
            let y = other.coordinate[i];
            res += x * y;
        }
        return Ok(res);
    }
}

impl Position<i32>{
    pub fn norm(&self) -> f64{
        let mut res : f64 = 0f64;
        for &x in self.coordinate.iter(){
            res += x as f64 * x as f64;
        }
        return res.sqrt();
    }

    pub fn distance(&self, other : &Self) -> Result<f64, Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut r : f64 = 0.0f64;
        for i in 0..self.dim(){
            let x : f64 = self.coordinate[i] as f64;
            let y : f64 = other.coordinate[i] as f64;
            r += (x - y) *  (x - y);
        }
        return Ok(r.sqrt());
    }

    pub fn taxi_distance(&self, other : &Self) -> Result<i32, Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut r : i32 = 0;
        for i in 0..self.dim(){
            let x : i32 = self.coordinate[i];
            let y : i32 = other.coordinate[i];
            r += (x - y).abs();
        }
        return Ok(r);
    }
}

impl<'a, 'b, T> Add<&'b Position<T>> for &'b Position<T>
    where T : Add<Output = T> + Clone + Copy{
    type Output = Position<T>;

    fn add(self, other: &'b Position<T>) -> Position<T>{
        if self.dim() != other.dim(){
            panic!("panic! {}", Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut vec : Vec<T> = Vec::<T>::new();
        for i in 0..self.dim(){
            let x : T = self.coordinate[i];
            let y : T = other.coordinate[i];
            vec.push(x + y);
        }

        return Position::new(vec.clone());
    }
}

impl<'a, 'b, T> Add<&'b mut Position<T>> for &'b mut Position<T>
    where T : Add<Output = T> + Clone + Copy{
    type Output = Position<T>;

    fn add(self, other: &'b mut Position<T>) -> Position<T>{
        if self.dim() != other.dim(){
            panic!("panic! {}", Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut vec : Vec<T> = Vec::<T>::new();
        for i in 0..self.dim(){
            let x : T = self.coordinate[i];
            let y : T = other.coordinate[i];
            vec.push(x + y);
        }

        return Position::new(vec.clone());
    }
}

impl<'a, 'b, T> Sub<&'b Position<T>> for &'b Position<T>
    where T : Sub<Output = T> + Clone + Copy{
    type Output = Position<T>;

    fn sub(self, other: &'b Position<T>) -> Position<T>{
        if self.dim() != other.dim(){
            panic!("panic! {}", Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut vec : Vec<T> = Vec::<T>::new();

        for i in 0..self.dim(){
            let x : T = self.coordinate[i];
            let y : T = other.coordinate[i];
            vec.push(x - y);
        }
        return Position::new(vec.clone());
    }
}

impl<'a, 'b, T> Sub<&'b mut Position<T>> for &'b mut Position<T>
    where T : Sub<Output = T> + Clone + Copy{
    type Output = Position<T>;

    fn sub(self, other: &'b mut Position<T>) -> Position<T>{
        if self.dim() != other.dim(){
            panic!("panic! {}", Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        let mut vec : Vec<T> = Vec::<T>::new();

        for i in 0..self.dim(){
            let x : T = self.coordinate[i];
            let y : T = other.coordinate[i];
            vec.push(x - y);
        }
        return Position::new(vec.clone());
    }
}

pub trait Numerics<T>{
    // Scalar Multiplication
    fn scalar_mul(&self, scalar : T) -> Position<T>;

    fn mut_scalar_mul(&mut self, scalar : T);

    // addition by mutation
    fn mut_add(&mut self, other: &Self) -> Result<(), Error>;

    // subtraction by mutation
    fn mut_sub(&mut self, other: &Self) -> Result<(), Error>;
}

impl<T> Numerics<T> for Position<T>
    where T : Add<Output = T> + AddAssign
                + Sub<Output = T> + SubAssign
                + Mul<Output = T> + Clone + Copy + Default{

    fn scalar_mul(&self, scalar : T) -> Position<T>{
        let mut vec : Vec<T> = Vec::new();

        for x in self.coordinate.iter(){
            vec.push(*x * scalar);
        }

        return Position::<T>::new(vec);
    }

    fn mut_scalar_mul(&mut self, scalar : T){
        for x in &mut self.coordinate{
            *x = *x * scalar;
        }
    }

    fn mut_add(&mut self, other: &Self) -> Result<(), Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        for i in 0..self.dim(){
            self.coordinate[i] += other.coordinate[i];
        }

        Ok(())
    }

    fn mut_sub(&mut self, other: &Self) -> Result<(), Error>{
        if self.dim() != other.dim(){
            return Err(Error::make_error_syntax(ErrorCode::InvalidDimension));
        }

        for i in 0..self.dim(){
            self.coordinate[i] -= other.coordinate[i];
        }

        Ok(())
    }
}

impl<T: Display> Display for Position<T>{
    fn fmt(&self, f : &mut Formatter) -> fmt::Result{
        let mut string = String::new();

        write!(&mut string, "{}", self.coordinate[0])?;
        for x in self.coordinate.iter().skip(1){
            write!(&mut string, ", {}", x)?;
        }
        write!(f, "{}", string)
    }
}

impl<T: LowerExp> LowerExp for Position<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        // write!(f, "(")?;
        LowerExp::fmt(&self.coordinate[0], f)?;
        for x in self.coordinate.iter().skip(1){
            write!(f, ", ")?;
            LowerExp::fmt(x, f)?;
        }
        // write!(f, ")")?;
        Ok(())
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let pos : Position<f64>;
        {
            let vec : Vec<f64> = vec![0f64, 0f64];
            pos = Position::new(vec);
        }
        assert_eq!(format!("{}", pos), "0, 0");
    }

    #[test]
    fn test_fmt(){
        assert_eq!(format!("{}", Position::<f64>{coordinate : vec![0.0, 1.1]}), "0, 1.1");
        assert_eq!(format!("{}", Position::<i32>{coordinate : vec![0, 2]}), "0, 2");
        assert_eq!(format!("{}", Position::<usize>{coordinate : vec![0, 2]}), "0, 2");

        assert_eq!(format!("{:e}", Position::<f64>{coordinate : vec![0.0, 1.0]}), "0e0, 1e0");
        assert_eq!(format!("{:05e}", Position::<f64>{coordinate : vec![0.0, 1.0]}), "000e0, 001e0");
    }

    #[test]
    fn test_f64_func(){
        let pos1 = Position::<f64>::new(vec![0.0, 0.0]);
        let pos2 = Position::<f64>::new(vec![1.0, 0.0]);
        let pos3 = Position::<f64>::new(vec![0.0, 0.0, 0.0]);
        let pos4 = Position::<f64>::new(vec![2.0, 0.0]);

        // dim test
        assert_eq!(pos1.dim(), 2);
        assert_eq!(pos3.dim(), 3);

        // norm
        assert_eq!(pos1.norm(), 0.0);
        assert_eq!(pos2.norm(), 1.0);
        assert_eq!(pos3.norm(), 0.0);
        assert_eq!(pos4.norm(), 2.0);

        // distance test
        assert_eq!(pos1.distance(&pos2), Ok(1.0));
        assert_eq!(pos1.distance(&pos1), Ok(0.0));
        assert_eq!(pos1.distance(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));

        // inner_product
        assert_eq!(pos1.inner_product(&pos2), Ok(0.0));
        assert_eq!(pos2.inner_product(&pos4), Ok(2.0));
        assert_eq!(pos2.inner_product(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }

    #[test]
    fn test_i32_func(){
        let pos1 = Position::<i32>::new(vec![0, 0]);
        let pos2 = Position::<i32>::new(vec![1, 0]);
        let pos3 = Position::<i32>::new(vec![0, 0, 0]);

        // dim test
        assert_eq!(pos1.dim(), 2);
        assert_eq!(pos3.dim(), 3);

        // norm
        assert_eq!(pos1.norm(), 0.0);
        assert_eq!(pos2.norm(), 1.0);
        assert_eq!(pos3.norm(), 0.0);

        // distance test
        assert_eq!(pos1.distance(&pos2), Ok(1.0));
        assert_eq!(pos1.distance(&pos1), Ok(0.0));
        assert_eq!(pos1.distance(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));

        // taxi distance test
        assert_eq!(pos1.taxi_distance(&pos2), Ok(1));
        assert_eq!(pos1.taxi_distance(&pos1), Ok(0));
        assert_eq!(pos1.taxi_distance(&pos3), Err(Error::make_error_syntax(ErrorCode::InvalidDimension)));
    }

    #[test]
    fn test_num_ops() -> Result<(), Error>{
        assert_eq!(&Position::<f64>::new(vec![0.0, 0.0]) + &Position::<f64>::new(vec![1.0, 2.0]),
            Position::<f64>::new(vec![1.0, 2.0]));
        assert_eq!(&Position::<f64>::new(vec![0.0, 0.0]) - &Position::<f64>::new(vec![1.0, 2.0]),
            Position::<f64>::new(vec![-1.0, -2.0]));
        assert_eq!(Position::<f64>::new(vec![1.0, 2.0]).scalar_mul(2.0),
            Position::<f64>::new(vec![2.0, 4.0]));

        let mut pos = Position::<f64>::new(vec![1.0, 2.0]);
        let pos2 = Position::<f64>::new(vec![2.0, 3.0]);

        pos.mut_scalar_mul(2.0);
        assert_eq!(pos, Position::<f64>::new(vec![2.0, 4.0]));

        pos.mut_add(&pos2)?;
        assert_eq!(pos, Position::<f64>::new(vec![4.0, 7.0]));

        pos.mut_sub(&pos2)?;
        assert_eq!(pos, Position::<f64>::new(vec![2.0, 4.0]));

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_num_ops_panic(){
        let _pos = &Position::<f64>::new(vec![0.0, 0.0]) + &Position::<f64>::new(vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_clear(){
        let mut pos = Position::<f64>::new(vec![3.0, 2.0]);
        pos.clear();
        assert_eq!(pos, Position::<f64>::new(vec![0.0, 0.0]));
    }
}



