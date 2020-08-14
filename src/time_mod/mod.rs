// Simulation에서는 time scale 조절이 매우 중요한데
// 이 모듈에서는 여러 가능한 time scale variation들을 제공할 예정이다.

use crate::prelude::*;

// To do
// Time trait 구성
// Constant time scale
// Exponentially increase time scale

// Input에 따라 Time iterator를 골라주는 함수

pub trait TimeIterator{
    fn current_time(&self) -> f64;

    fn renew(&mut self);
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TimeType{
    Constant(f64),
    Exponential(f64, f64, usize),
}

// ============================

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ConstStep{
    pub titype  : TimeType,
    pub current : f64,
    pub dt      : f64,
}

impl ConstStep{
    #[allow(dead_code)]
    fn new(dt : f64) -> Self{
        Self{
            titype  : TimeType::Constant(dt),
            current : 0f64,
            dt      : dt,
        }
    }
}

impl TimeIterator for ConstStep{
    fn current_time(&self) -> f64{
        self.current
    }

    fn renew(&mut self){
        self.current = 0f64;
    }
}

impl Iterator for ConstStep{
    type Item = f64;

    fn next(&mut self) -> Option<f64>{
        let time = self.current;
        self.current += self.dt;
        Some(time)
    }
}

impl_argument_trait!(ConstStep, "Time Iterator", ConstStepArguments, 1;
    dt, f64, "Time step size");

impl ConstStep{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ConstStepArguments) -> Self{
        Self{
            titype  : TimeType::Constant(argument.dt),
            current : 0f64,
            dt      : argument.dt,
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_const_step(){
        let timeiter = ConstStep::new(1f64);

        let mut idx : usize = 0;
        let mut res = String::new();

        for time in timeiter{
            res.push_str(format!("{} ", time).as_str());
            idx += 1;
            if idx == 5{
                break;
            }
        }
        assert_eq!(res, "0 1 2 3 4 ");
    }

    #[test]
    fn test_const_step_arguments(){
        let res = ConstStep::info(3);
        assert_eq!(res, "dt : Time step size\n");

        let res = ConstStep::brief_info();
        assert_eq!(res, "Time Iterator arguments : (dt) ");

        let args = ["1".to_string(), "0.1".to_string()];
        let res = ConstStep::read_args_from_vec(&args[1..]);
        assert_eq!(res, Ok(ConstStepArguments{dt : 0.1}));

    }

    #[test]
    #[ignore]
    fn test_euler_process(){
        use crate::random_mod::{get_gaussian, rng_seed};
        use rand_pcg::Pcg64;
        // Solution for dXt = aXt dt + b Xt dWt is
        // Xt = X0 exp((a - b^2/2)t + b Wt)
        // and its mean value is Xt = X0 exp(at) with
        // mean square <Xt^2> = X0^2 exp((2a + b^2)t)

        let mut rng : Pcg64 = rng_seed(21312412314);
        let dt : f64 = 5e-3;
        let mut timeiter = ConstStep::new(dt);
        let (x0, a, b, tmax) : (f64, f64, f64, f64) = (1f64, 1f64, 2f64, 1f64);
        let (mut mean_x, mut square_x) : (f64, f64) = (0f64, 0f64);
        let ensemble : usize = 1000;

        for _i in 0..ensemble{
            let mut x : f64 = x0;
            timeiter.renew();

            for time in timeiter{
                if time > tmax{
                    mean_x += x;
                    square_x += x * x;
                    break;
                }

                x += a * x * dt + b * x * get_gaussian(&mut rng) * dt.sqrt();
            }
        }

        mean_x /= ensemble as f64;
        square_x /= ensemble as f64;

        let expect_x = x0 * (a * tmax).exp();
        let expect_x2 = x0.powi(2) * ((2f64 * a + b * b) * tmax).exp();

        assert!(((mean_x - expect_x) / expect_x).abs() < (ensemble as f64).powf(-0.5));
        println!("{} {}\n", mean_x, expect_x);
        println!("{} {}\n", square_x, expect_x2);
        assert!(((square_x - expect_x2) / expect_x2).abs() < (ensemble as f64).powf(-0.5));
    }

}









