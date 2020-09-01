// Simulation에서는 time scale 조절이 매우 중요한데
// 이 모듈에서는 여러 가능한 time scale variation들을 제공할 예정이다.

use crate::prelude::*;

// To do
// Time trait 구성
// Constant time scale
// Exponentially increase time scale

// Input에 따라 Time iterator를 골라주는 함수

pub trait TimeIterator
    where Self : Sized{
    fn current_time(&self) -> f64;

    fn dt(&self) -> f64;

    fn renew(&mut self);

    fn set_tmax(&mut self, tmax : f64) -> Result<(), Error>;

    fn into_diff(&self) -> TimeDiffIterator<Self>;
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeDiffIterator<T>{
    pub timeiter : T,
}


impl<T : Iterator + TimeIterator> Iterator for TimeDiffIterator<T>{
    type Item = (T::Item, f64);

    fn next(&mut self) -> Option<Self::Item>{
        match self.timeiter.next(){
            Some(time) => Some((time, self.timeiter.dt())),
            None => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum TimeType{
    Constant(f64),
    Exponential(f64, f64, usize),
}


impl Display for TimeType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            TimeType::Constant(dt) => write!(f, "Constant Time step with size {0:.5e}", dt),
            TimeType::Exponential(dt_min, dt_max, length) => write!(f, "{}", format_args!("Exponential time step increase from {0:.5e} to {1:.5e} Each increase occurs in every {2:} step", dt_min, dt_max, length)),
        }
    }
}


impl FromStr for TimeType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        match split[0]{
            "Constant" => Ok(TimeType::Constant(split[5].parse::<f64>().expect("Failed to parse"))),
            "Exponential" => {
                let dt_min = split[5].parse::<f64>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                let dt_max = split[7].parse::<f64>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                let length = split[13].parse::<usize>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                Ok(TimeType::Exponential(dt_min, dt_max, length))
            },
            _ => {
                if split.len() == 1{
                    split[0].parse::<f64>().map(|c| TimeType::Constant(c))
                                   .map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
                else if split.len() == 3{
                    let dt_min = split[0].parse::<f64>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    let dt_max = split[1].parse::<f64>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    let length = split[2].parse::<usize>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    Ok(TimeType::Exponential(dt_min, dt_max, length))
                }
                else{
                    Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
            },
        }

    }
}

// =============================================================================

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ConstStep{
    pub titype  : TimeType,
    pub current : f64,
    pub dt      : f64,
    pub tmax    : f64,
}

impl ConstStep{
    #[allow(dead_code)]
    pub fn new(dt : f64) -> Result<Self, Error>{
        if dt < 1e-15{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        Ok(Self{
            titype  : TimeType::Constant(dt),
            current : 0f64,
            dt      : dt,
            tmax    : std::f64::MAX,
        })
    }
}

impl TimeIterator for ConstStep{
    fn current_time(&self) -> f64{
        self.current
    }

    fn dt(&self) -> f64{
        self.dt
    }

    fn renew(&mut self){
        self.current = 0f64;
    }

    fn set_tmax(&mut self, tmax : f64) -> Result<(), Error>{
        if tmax < 0f64{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        else if tmax < 1e-10{
            self.tmax = std::f64::MAX;
        }
        else{
            self.tmax = tmax;
        }

        Ok(())
    }

    fn into_diff(&self) -> TimeDiffIterator<Self>{
        TimeDiffIterator{
            timeiter : self.clone(),
        }
    }
}

impl Iterator for ConstStep{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item>{
        let time = self.current;
        if time <= self.tmax{
            self.current += self.dt;
            return Some(time);
        }
        else{
            return None;
        }
    }
}

impl_argument_trait!(ConstStep, "Time Iterator", ConstStepArguments, 2;
    dt, f64, "Time step size",
    tmax, f64, "Upper limit of time for computing. ex) 1.0, 0 means INFINITE");

impl ConstStep{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ConstStepArguments) -> Self{
        let tmax : f64;

        if argument.tmax < 0f64 || argument.dt < 0f64{
            panic!("{}", ErrorCode::InvalidArgumentInput);
        }
        else if argument.tmax < 1e-10{
            tmax = std::f64::MAX;
        }
        else{
            tmax = argument.tmax;
        }

        Self{
            titype  : TimeType::Constant(argument.dt),
            current : 0f64,
            dt      : argument.dt,
            tmax    : tmax,
        }
    }
}

// =============================================================================


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ExponentialStep{
    pub titype  : TimeType,             // Type of time step
    pub current : f64,                  // current time
    pub dt      : f64,                  // current time step
    pub inc     : f64,                  // increase ratio of step size
    pub tmax    : f64,                  // maximum time
    pub count   : usize,                // count for next time step
}

impl ExponentialStep{
    #[allow(dead_code)]
    pub fn new(dt_min : f64, dt_max : f64, length : usize) -> Result<Self, Error>{
        if dt_min < 1e-15 || dt_min > dt_max || length == 0{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }

        let inc : f64;
        if length == 1{
            inc = 1.2f64;
        }
        else if length < 10{
            inc = (length as f64) / 2f64;
        }
        else{
            inc = 10f64;            // default
        }

        Ok(Self{
            titype : TimeType::Exponential(dt_min, dt_max, length),
            current : 0f64,
            dt      : dt_min,
            inc     : inc,
            tmax    : std::f64::MAX,
            count   : 0,
        })
    }

    #[allow(dead_code)]
    pub fn set_inc(&mut self, inc : f64) -> Result<(), Error>{
        if inc <= 1f64{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        match self.titype{
            TimeType::Exponential(_min, _max, length) =>{
                if inc * 2f64 > length as f64{
                    self.inc = (length as f64) / 2f64;
                }
                else{
                    self.inc = inc;
                }
            }
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidType));
            }
        }

        Ok(())
    }
}


impl TimeIterator for ExponentialStep{
    fn current_time(&self) -> f64{
        self.current
    }

    fn dt(&self) -> f64{
        self.dt
    }

    fn renew(&mut self){
        self.current = 0f64;
        self.count = 0;

        match self.titype{
            TimeType::Exponential(dt_min, _dt_max, _length) => {
                self.dt = dt_min;
            }
            _ => {
                panic!(format!("{}", ErrorCode::InvalidType));
            }
        }
    }

    fn set_tmax(&mut self, tmax : f64) -> Result<(), Error>{
        if tmax < 0f64{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        else if tmax < 1e-15{
            self.tmax = std::f64::MAX;
        }
        else{
            self.tmax = tmax;
        }

        Ok(())
    }

    fn into_diff(&self) -> TimeDiffIterator<Self>{
        TimeDiffIterator{
            timeiter : self.clone(),
        }
    }
}

impl Iterator for ExponentialStep{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item>{
        match self.titype{
            TimeType::Exponential(_dt_min, dt_max, length) => {
                if self.dt < dt_max && self.count == length{
                    self.count = 0;
                    self.dt *= self.inc;
                    if self.dt > dt_max{
                        self.dt = dt_max;
                    }
                }
                let time = self.current;
                if time <= self.tmax{
                    self.current += self.dt;
                    self.count += 1;
                    return Some(time);
                }
                else{
                    return None;
                }
            },
            _ => {
                panic!(format!("{}", ErrorCode::InvalidType));
            }
        }
    }
}

impl_argument_trait!(ExponentialStep, "Time Iterator", ExponentialStepArguments, 4;
    dt_min, f64, "Initial time step size",
    dt_max, f64, "Maximal time step size",
    length, usize, "Number of step between time step size increase",
    tmax, f64, "Upper limit of time for computing. ex) 1.0, 0 means INFINITE");

impl ExponentialStep{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ExponentialStepArguments) -> Self{

        let dt_min : f64 = argument.dt_min;
        let dt_max : f64 = argument.dt_max;
        let length : usize = argument.length;
        let mut tmax : f64 = argument.tmax;

        if tmax < 0f64 || dt_min < 0f64
            || dt_min > dt_max || length < 10{
            panic!("{}", ErrorCode::InvalidArgumentInput);
        }
        else if tmax < 1e-15{
            tmax = std::f64::MAX;
        }

        Self{
            titype  : TimeType::Exponential(dt_min, dt_max, length),
            current : 0f64,
            dt      : dt_min,
            inc     : 1.2f64,
            tmax    : tmax,
            count   : 0,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_display_fromstr(){
        let res = format!("{}", TimeType::Constant(1f64));
        let res = res.as_str();
        assert_eq!(res, "Constant Time step with size 1.00000e0");
        assert_eq!(TimeType::from_str(res), Ok(TimeType::Constant(1f64)));

        let res = format!("{}", TimeType::Exponential(1e-10, 1e-5, 10));
        let res = res.as_str();
        assert_eq!(res, "Exponential time step increase from 1.00000e-10 to 1.00000e-5 Each increase occurs in every 10 step");
        assert_eq!(TimeType::from_str(res), Ok(TimeType::Exponential(1e-10, 1e-5, 10)));
    }

    #[test]
    fn test_const_step() -> Result<(), Error>{
        let mut timeiter = ConstStep::new(1f64)?;
        timeiter.set_tmax(5f64)?;

        let mut res = String::new();

        for time in timeiter{
            res.push_str(format!("{} ", time).as_str());
        }
        assert_eq!(res, "0 1 2 3 4 5 ");
        Ok(())
    }

    #[test]
    fn test_const_step_arguments(){
        let res = ConstStep::info(5);
        assert_eq!(res, "dt   : Time step size\ntmax : Upper limit of time for computing. ex) 1.0, 0 means INFINITE\n");

        let res = ConstStep::brief_info();
        assert_eq!(res, "Time Iterator arguments : (dt) (tmax) ");

        let args = ["1".to_string(), "0.1".to_string(), "0.0".to_string()];
        let res = ConstStep::read_args_from_vec(&args[1..]);
        assert_eq!(res, Ok(ConstStepArguments{dt : 0.1, tmax : 0.0}));
    }

    #[test]
    #[ignore]
    fn test_const_euler_process() -> Result<(), Error>{
        use crate::random_mod::{get_gaussian, rng_seed};
        use rand_pcg::Pcg64;
        // Solution for dXt = aXt dt + b Xt dWt is
        // Xt = X0 exp((a - b^2/2)t + b Wt)
        // and its mean value is Xt = X0 exp(at) with
        // mean square <Xt^2> = X0^2 exp((2a + b^2)t)

        let mut rng : Pcg64 = rng_seed(21312412314);
        let dt : f64 = 1e-5;
        let mut timeiter = ConstStep::new(dt)?;
        let (x0, a, b, tmax) : (f64, f64, f64, f64) = (1f64, 0.5f64, 1f64, 0.5f64);
        timeiter.set_tmax(tmax)?;
        let (mut mean_x, mut square_x) : (f64, f64) = (0f64, 0f64);
        let ensemble : usize = 10000;

        for _i in 0..ensemble{
            let mut x : f64 = x0;
            timeiter.renew();

            for _time in timeiter{
                x += a * x * dt + b * x * get_gaussian(&mut rng) * dt.sqrt();
            }
            mean_x += x;
            square_x += x * x;
        }

        mean_x /= ensemble as f64;
        square_x /= ensemble as f64;

        let expect_x = x0 * (a * tmax).exp();
        let expect_x2 = x0.powi(2) * ((2f64 * a + b * b) * tmax).exp();

        println!("{} {}\n", mean_x, expect_x);
        println!("{} {}\n", square_x, expect_x2);
        assert!(((mean_x - expect_x) / expect_x).abs() < (ensemble as f64).powf(-0.5));
        assert!(((square_x - expect_x2) / expect_x2).abs() < (ensemble as f64).powf(-0.5));

        Ok(())
    }

    #[test]
    fn test_exponential() -> Result<(), Error>{
        let timeiter = ExponentialStep::new(1f64, 100f64, 10);
        assert_eq!(timeiter, Ok(ExponentialStep{
            titype : TimeType::Exponential(1f64, 100f64, 10),
            current : 0f64,
            dt      : 1f64,
            inc     : 10f64,
            tmax    : std::f64::MAX,
            count   : 0,
        }));
        let mut timeiter = timeiter.unwrap();

        timeiter.set_tmax(1.2)?;
        assert_eq!(timeiter.tmax, 1.2f64);

        timeiter.set_tmax(0f64)?;
        assert_eq!(timeiter.tmax, std::f64::MAX);

        timeiter.set_tmax(100f64)?;
        assert_eq!(timeiter.tmax, 100f64);

        timeiter.set_inc(1.2f64)?;
        assert_eq!(timeiter.inc, 1.2f64);

        timeiter.set_inc(10f64)?;           // Too large increment
        assert_eq!(timeiter.inc, 5f64);

        let mut res = String::new();
        for time in timeiter{
            res.push_str(format!("{} ", time).as_str());
        }
        assert_eq!(res, "0 1 2 3 4 5 6 7 8 9 10 15 20 25 30 35 40 45 50 55 60 85 ");

        Ok(())
    }

    #[test]
    fn test_exponential_argument() -> Result<(), Error>{
        let res = ExponentialStep::info(10);
        assert_eq!(res, "dt_min    : Initial time step size
dt_max    : Maximal time step size
length    : Number of step between time step size increase
tmax      : Upper limit of time for computing. ex) 1.0, 0 means INFINITE\n");

        let res = ExponentialStep::brief_info();
        assert_eq!(res, "Time Iterator arguments : (dt_min) (dt_max) (length) (tmax) ");

        let args : Vec<String> = ["1", "1e-10", "1e-5", "10", "0.0"].iter().map(|x| x.to_string()).collect();
        let res = ExponentialStep::read_args_from_vec(&args[1..]);
        assert_eq!(res, Ok(ExponentialStepArguments{dt_min : 1e-10, dt_max : 1e-5, length : 10, tmax : 0.0}));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_exponential_euler_process() -> Result<(), Error>{
        use crate::random_mod::{get_gaussian, rng_seed};
        use rand_pcg::Pcg64;
        // Solution for dXt = aXt dt + b Xt dWt is
        // Xt = X0 exp((a - b^2/2)t + b Wt)
        // and its mean value is Xt = X0 exp(at) with
        // mean square <Xt^2> = X0^2 exp((2a + b^2)t)

        let mut rng : Pcg64 = rng_seed(21312412314);
        let mut timeiter = ExponentialStep::new(1e-10, 1e-5, 10)?;
        let (x0, a, b, tmax) : (f64, f64, f64, f64) = (1f64, 0.5f64, 1f64, 0.5f64);
        timeiter.set_tmax(tmax)?;
        let (mut mean_x, mut square_x) : (f64, f64) = (0f64, 0f64);
        let ensemble : usize = 10000;

        for _i in 0..ensemble{
            let mut x : f64 = x0;
            timeiter.renew();

            for (_time, dt) in timeiter.into_diff(){
                x += a * x * dt + b * x * get_gaussian(&mut rng) * dt.sqrt();
            }
            mean_x += x;
            square_x += x * x;
        }

        mean_x /= ensemble as f64;
        square_x /= ensemble as f64;

        let expect_x = x0 * (a * tmax).exp();
        let expect_x2 = x0.powi(2) * ((2f64 * a + b * b) * tmax).exp();

        println!("{} {}\n", mean_x, expect_x);
        println!("{} {}\n", square_x, expect_x2);
        assert!(((mean_x - expect_x) / expect_x).abs() < (ensemble as f64).powf(-0.5));
        assert!(((square_x - expect_x2) / expect_x2).abs() < (ensemble as f64).powf(-0.5));

        Ok(())
    }
}









