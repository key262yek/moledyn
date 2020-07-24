// Module for Position

use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};
use std::fmt::{self, Display, Write, Formatter, LowerExp};
use crate::error::{Error, ErrorCode};
use std::default::Default;


#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Position<T>{                 // vector 연산을 정의하기 위해서 wrapping한 structure
    pub coordinate : Vec<T>             // generic type T를 이용해 확장성을 노렸다.
}                                       // T가 정수형이면 discrete system, 실수형이면 continuous system이 되는 것

impl<T : Clone> Position<T>{
    pub fn new(vec : Vec<T>) -> Self{   // vector를 새로 정의하는 함수
        Position::<T>{
            coordinate : vec.clone()    // 벡터는 copy가 되지 않으므로, clone을 해야함.
        }                               // 그래서 T도 clone trait이 정의된 type이어야 하는 것.
    }
}

impl<T> Position<T>{                    // 일반적인 position vector가 가져야 하는 함수들
    pub fn dim(&self) -> usize{
        // dimension을 출력해주는 함수
        self.coordinate.len()
    }
}

impl<T : Default> Position<T>{
    pub fn clear(&mut self){
        // 종종 vector를 initialize해야할 필요가 있다.
        // 모두 0으로 만들어주는 함수
        // rust에서는 정수형에서의 0과 실수형에서의 0이 다르다.
        // 따라서 default trait을 이용해 0으로 바꿔주는 함수

        for x in &mut self.coordinate{
            *x = Default::default();
        }
    }
}

impl Position<f64>{                                 // 실수형 벡터의 함수들
    pub fn norm(&self) -> f64{
        // Norm function
        let mut res : f64 = 0f64;
        for &x in self.coordinate.iter(){
            res += x * x;
        }
        return res.sqrt();
    }

    pub fn distance(&self, other : &Self) -> Result<f64, Error>{
        // distance between self and other
        // norm을 이용하면 간단할 수 있지만,
        // 그 경우 벡터를 새로 정의해서 그 벡터의 norm을 구해야하는 문제가 생김
        // allocation, free가 시간을 많이 잡아먹기 때문에 아래와 같이 구성함.

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
        // inner product
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
    // 종종 가독성을 위해 그냥 덧셈을 정의하는 것이 좋을 때도 있다.
    // 다만 이 경우들은 모두 새로운 벡터를 정의하고 있기 때문에,
    // allocation, free 시간을 소모해야함
    // 그래서 최대한 쓰지 않으려 한다.

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
    // 위와 같은 함수인데, mutable reference에 대해서 따로 정리해줘야함

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
    // 종종 가독성을 위해 그냥 뺄셈을 정의하는 것이 좋을 때도 있다.
    // 다만 이 경우들은 모두 새로운 벡터를 정의하고 있기 때문에,
    // allocation, free 시간을 소모해야함
    // 그래서 최대한 쓰지 않으려 한다.

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
    // 위와 같은 함수지만 mutable reference에 대해 따로 정의해줘야 함.

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
    // Numerics를 정의한 trait
    // 대부분의 벡터 연산은 이들을 이용할 것.

    // Scalar Multiplication
    fn scalar_mul(&self, scalar : T) -> Position<T>;

    // 스칼라곱. 계산 결과를 새로운 벡터로 출력하는 것이 아니라 주어진 벡터를 바꾸는 방식
    fn mut_scalar_mul(&mut self, scalar : T);

    // 덧셈. 계산 결과를 새로운 벡터로 출력하는 것이 아니라 주어진 벡터를 바꾸는 방식
    fn mut_add(&mut self, other: &Self) -> Result<(), Error>;

    // 뺄셈. 계산 결과를 새로운 벡터로 출력하는 것이 아니라 주어진 벡터를 바꾸는 방식
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



