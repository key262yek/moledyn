// Module for random numbers
//
// Provides uniformly distributed random number, normal random variable,
// random vectors
//
// 기본적으로 Pcg64 algorithm을 통해 random number generation을 함.
// https://www.pcg-random.org/pdf/toms-oneill-pcg-family-v1.02.pdf
// 설명에서 알 수 있듯이 64bit integer를 출력해주는 이 알고리즘의 period는 2^128.
// 차고 넘친다고 할 수 있다.


use crate::prelude::*;
use rand_distr::StandardNormal;
use rand::distributions::Open01;
use rand::Rng;


pub fn rng_seed(seed : u128) -> Pcg64{
    // PCG family algorithm을 기반해서 random number를 만드는 generator를 만들어주는 함수.
    // seed : random number를 결정지을 seed.

    const INC: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    rand_pcg::Pcg64::new(seed, INC)
}


pub fn get_uniform(rng : &mut Pcg64) -> f64{
    // (0, 1) 범위의 random number를 출력해주는 함수
    // rng : random number generator

    rng.sample(Open01)
}

pub fn get_uniform_vec(rng: &mut Pcg64, dim: usize) -> Position<f64>{
    // 원하는 dimension의 random vector를 선언. 이때 random number는 uniform dist.
    // rng : random number generator
    // dim : dimension of random vector

    let mut buff : Vec<f64> = vec![0f64; dim];
    for x in &mut buff{
        *x = rng.sample(Open01);
    }
    return Position::<f64>::new(buff);
}

pub fn get_uniform_to_vec(rng: &mut Pcg64, vec: &mut Position<f64>){
    // 매번 vector를 allocate해서 반환하면 allocation, free의 시간이 굉장히 많이 쌓이게 됨
    // Performance에 지대한 영향을 주기 때문에, 미리 선언한 vec에 random number를 건네주는 역할을 함
    // rng : random number generator
    // vec : 결과를 저장할 vector의 reference

    vec.clear();
    for x in &mut vec.coordinate{
        *x = rng.sample(Open01);
    }
}

pub fn get_uniform_to_vec_nonstandard(rng: &mut Pcg64, vec: &mut Position<f64>, min: f64, max: f64){
    // 앞서 to_vec에서의 concept에 더해서
    // 종종 여러 벡터의 값을 합한 결과가 필요할 수 있다.
    // 이때는 추후 random number만을 가지고 연산을 하기 힘들어짐을 의미하고
    // 따라서 random number generating 과정에서 random number가 가져야하는 characteristic을 가지고 있어야 한다.

    // rng : random number generator
    // vec : 결과를 저장할 vector의 reference
    // min : uniform distribution의 최솟값
    // max : uniform distribution의 최댓값

    for x in &mut vec.coordinate{
        let r : f64 = rng.sample(Open01);
        *x = *x + r * (max - min) + min;
    }
}

pub fn get_gaussian(rng : &mut Pcg64) -> f64{
    // standard normal random number를 만들어주는 함수
    // rng : random number generator

    rng.sample(StandardNormal)
}

pub fn get_gaussian_vec(rng: &mut Pcg64, dim: usize) -> Position<f64>{
    // 원하는 dimension의 random vector를 선언. 이때 random number는 standard_normal
    // rng : random number generator
    // dim : dimension of random vector

    let mut buff : Vec<f64> = vec![0f64; dim];
    for x in &mut buff{
        *x = rng.sample(StandardNormal);
    }
    return Position::<f64>::new(buff);
}

pub fn get_gaussian_to_vec(rng: &mut Pcg64, vec: &mut Position<f64>){
    // get_**_vec 꼴의 함수들은 모두 새로운 vector를 allocate하고 free하는 작업이 필요하다.
    // random number generation이 굉장히 빈번히 사용되는데 여기서 malloc이 시간을 매우 잡아먹음.
    // mutable reference에 덮어씌우는 방식으로 시간을 매우 개선할 수 있었다.
    // rng : random number generator
    // vec : 결과값을 저장할 vector의 reference

    vec.clear();
    for x in &mut vec.coordinate{
        *x = rng.sample(StandardNormal);
    }
}

pub fn get_gaussian_to_vec_nonstandard(rng: &mut Pcg64, vec: &mut Position<f64>, mean: f64, stddev: f64){
    // 가장 최선의 사용을 위해서 single_move (여기선 vec) 위에
    // random number도 더하고, force term도 계속 더하는 방식을 취할 수 있으면 편할거라 생각했다.
    // 값을 자동으로 clear하지 않고 random number를 더해주는 함수.
    // 따라서 이 random number를 여러 mean, stddev에 대해서 사용하고 싶다면 더하기 전에 미리 곱해줘야할 필요성이 있음.
    // rng : random number generator
    // vec : 결과값을 저장할 vector의 reference
    // mean : gaussian의 평균값
    // stddev : gaussian의 표준편차

    for x in &mut vec.coordinate{
        let r : f64 = rng.sample(StandardNormal);
        *x = *x + r * stddev + mean;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_uniform(){
        // uniform distribution 범위 안에 잘 들어오는가 확인하는 테스트
        let mut rng = rng_seed(3123412314);
        let mut buff = [0f64; 10];

        for x in &mut buff{
            *x = get_uniform(&mut rng);
        }

        for &x in buff.iter(){
            assert_eq!(0.0 < x && x < 1.0, true);   // 범위가 (0, 1)에 있는지?
        }
    }

    #[test]
    fn test_sequence(){
        // seed가 sequentially 잘 작동하는지 확인하는 test

        let n : usize = 100;

        // uniform만
        let mut rng = rng_seed(3123412314);
        let mut x1 = 0f64;

        for _i in 0..n{
            x1 = get_uniform(&mut rng);
        }

        // gaussian이랑 번갈아가면서
        let mut rng2 = rng_seed(3123412314);
        let mut x2 = 0f64;

        for _i in 0..n/2{
            get_gaussian(&mut rng2);
            x2 = get_uniform(&mut rng2);
        }

        // uniform만 n번 뽑은 경우와, gaussian과 번갈아 뽑은 경우
        // 그래도 횟수만 같으면 seed가 같을 것이기 때문에 같은 random number를 주어야 함.
        assert!((x1 - x2).abs() < 1e-10f64);
    }

    #[test]
    fn test_random_vec(){
        // random vec 함수들 테스트
        // 다른 컴퓨터에서 테스트 하더라도 random number generator의 algorithm이 잘 작동한다면 아래와 같은 벡터들이 생성되어야 한다.
        let mut rng = rng_seed(3123412314);

        // uniform vector
        let pos1 = get_uniform_vec(&mut rng, 2);
        let pos2 = get_uniform_vec(&mut rng, 3);

        assert_eq!(pos1, Position::<f64>::new(vec![0.902588585455231, 0.7100941485044704]));
        assert_eq!(pos2, Position::<f64>::new(vec![0.05524656756602109, 0.1639200697192441, 0.6241346531407977]));

        // gaussian vector
        let pos3 = get_gaussian_vec(&mut rng, 2);
        let pos4 = get_gaussian_vec(&mut rng, 3);

        assert_eq!(pos3, Position::<f64>::new(vec![-0.0629443920960169, 0.45927525382639517]));
        assert_eq!(pos4, Position::<f64>::new(vec![-0.1368207211245114, -1.7150386023894229, 1.1485182266457186]));

    }
}
