// setup 하는데 걸리는 시간이 얼마나 차이나는지 확인


use criterion::{criterion_group, criterion_main, Criterion};
use rts::time_mod::{TimeIterator, ExponentialStep};
use rts::error::{Error, ErrorCode};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Exp{
    dt_min : f64,
    dt_max : f64,
    length : usize,
    current : f64,
    dt : f64,
    inc : f64,
    tmax : f64,
    count : usize,
}


impl Exp{
    #[allow(dead_code)]
    pub fn new(dt_min : f64, dt_max : f64, length : usize) -> Result<Self, Error>{
        if dt_min < 1e-15 || dt_min > dt_max || length == 0{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        Ok(Self{
            dt_min : dt_min,
            dt_max : dt_max,
            length : length,
            current : 0f64,
            dt      : dt_min,
            inc     : 1.2f64,   // default
            tmax    : std::f64::MAX,
            count   : 0,
        })
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn set_inc(&mut self, inc : f64) -> Result<(), Error>{
        if inc <= 1f64{
            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        }
        else {
            self.inc = inc;
        }

        Ok(())
    }
}


impl TimeIterator for Exp{
    fn current_time(&self) -> f64{
        self.current
    }

    fn renew(&mut self){
        self.current = 0f64;
        self.count = 0;
        self.dt = self.dt_min;
    }
}

impl Iterator for Exp{
    type Item = f64;

    fn next(&mut self) -> Option<f64>{
        if self.dt < self.dt_max && self.count == self.length{
            self.count = 0;
            self.dt *= self.inc;
            if self.dt > self.dt_max{
                self.dt = self.dt_max;
            }
        }
        let time = self.current;
        self.count += 1;
        if time < self.tmax{
            self.current += self.dt;
            return Some(time);
        }
        else{
            return None;
        }
    }
}

fn enum_renew(timeiter : &mut ExponentialStep){
    timeiter.renew();
}

fn enum_iter(timeiter : &mut ExponentialStep){
    for _time in timeiter.into_iter(){
    }
    timeiter.renew();
}

fn struct_renew(timeiter : &mut Exp){
    timeiter.renew();
}

fn struct_iter(timeiter : &mut Exp){
    for _time in timeiter.into_iter(){
    }
    timeiter.renew();
}


fn bench_renew(c : &mut Criterion){
    // renew 는 비교 불가한 시간차이를 보여주었음.
    // enum : few ns / struct : 0ps 수준의 차이

    let mut time_enum = ExponentialStep::new(1e-10, 1e-5, 10).unwrap();
    let mut time_struct = Exp::new(1e-10, 1e-5, 10).unwrap();

    let mut group = c.benchmark_group("Renew");
    group.bench_function("enum", |b|  b.iter(|| enum_renew(&mut time_enum)));
    group.bench_function("struct", |b|  b.iter(|| struct_renew(&mut time_struct)));
}

fn bench_iter(c : &mut Criterion){
    // iteration은 다른 부분들이 많아서 오히려 시간 차이가 크게 나지 않음
    // enum : 236 us
    // struct : 113 us
    let mut time_enum = ExponentialStep::new(1e-10, 1e-5, 100).unwrap();
    time_enum.set_tmax(1f64).unwrap();

    let mut time_struct = Exp::new(1e-10, 1e-5, 100).unwrap();
    time_struct.set_tmax(1f64).unwrap();

    let mut group = c.benchmark_group("Iter");
    group.bench_function("enum", |b|  b.iter(|| enum_iter(&mut time_enum)));
    group.bench_function("struct", |b|  b.iter(|| struct_iter(&mut time_struct)));
}

criterion_group!(benches, bench_renew, bench_iter);
criterion_main!(benches);
