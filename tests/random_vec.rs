// Confirm the Random number generators

use rts::error::Error;
use rts::system_mod::{SystemCore, cont_circ::ContCircSystem};
use rts::random_mod::{rng_seed, get_uniform_vec, get_gaussian_vec, get_uniform_to_vec, get_gaussian_to_vec};
use rts::position::Position;

#[test]
#[ignore]
fn test_cont_circ_random_pos_contour() -> Result<(), Error>{
    // random_pos 결과가 잘 나오는지 위치를 다 찍어주는 테스트
    // plot 프로그램으로 시스템 결과를 확인해볼 수 있다.
    use std::fs::File;
    use std::io::Write;

    let sys = ContCircSystem::new(3.0, 2);
    let mut rng = rng_seed(1314212314);

    let n : usize = 10000;
    let mut file = File::create("tests/images/cont_circ_random_pos.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        let pos : &Vec<f64> = &sys.random_pos(&mut rng)?.coordinate;
        let (x, y) = (pos[0], pos[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}

#[test]
#[ignore]
fn test_cont_circ_random_pos_to_vec_contour() -> Result<(), Error>{
    use std::fs::File;
    use std::io::Write;

    let sys = ContCircSystem::new(3.0, 2);
    let mut rng = rng_seed(1314212314);
    let mut pos : Position<f64> = Position::new(vec![0.0, 0.0]);

    let n : usize = 10000;
    let mut file = File::create("tests/images/cont_circ_random_pos_to_vec.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        sys.random_pos_to_vec(&mut rng, &mut pos)?;
        let (x, y) = (pos.coordinate[0], pos.coordinate[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}

#[test]
#[ignore]
fn test_get_uniform_vec() -> Result<(), Error>{
    // uniform vector가 정말 uniform하게 나오는지 개별 데이터를 출력해주는 테스트
    use std::fs::File;
    use std::io::Write;

    let mut rng = rng_seed(1314212314);

    let n : usize = 10000;
    let mut file = File::create("tests/images/get_uniform_vec.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        let pos : &Vec<f64> = &get_uniform_vec(&mut rng, 2).coordinate;
        let (x, y) = (pos[0], pos[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}

#[test]
#[ignore]
fn test_get_uniform_to_vec() -> Result<(), Error>{
    // uniform vector가 정말 uniform하게 나오는지 개별 데이터를 출력해주는 테스트
    use std::fs::File;
    use std::io::Write;

    let mut rng = rng_seed(1314212314);
    let mut pos : Position<f64> = Position::new(vec![0.0, 0.0]);

    let n : usize = 10000;
    let mut file = File::create("tests/images/get_uniform_to_vec.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        get_uniform_to_vec(&mut rng, &mut pos);
        let (x, y) = (pos.coordinate[0], pos.coordinate[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}

#[test]
#[ignore]
fn test_get_gaussian_vec() -> Result<(), Error>{
    // gaussian vector가 정말 gaussian하게 나오는지 개별 데이터를 출력해주는 테스트
    use std::fs::File;
    use std::io::Write;

    let mut rng = rng_seed(1314212314);

    let n : usize = 10000;
    let mut file = File::create("tests/images/get_gaussian_vec.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        let pos : &Vec<f64> = &get_gaussian_vec(&mut rng, 2).coordinate;
        let (x, y) = (pos[0], pos[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}

#[test]
#[ignore]
fn test_get_gaussian_to_vec() -> Result<(), Error>{
    /// gaussian vector가 정말 uniform하게 나오는지 개별 데이터를 출력해주는 테스트
    use std::fs::File;
    use std::io::Write;

    let mut rng = rng_seed(1314212314);
    let mut pos : Position<f64> = Position::new(vec![0.0, 0.0]);

    let n : usize = 10000;
    let mut file = File::create("tests/images/get_gaussian_to_vec.dat").map_err(Error::make_error_io)?;

    for _i in 0..n{
        get_gaussian_to_vec(&mut rng, &mut pos);
        let (x, y) = (pos.coordinate[0], pos.coordinate[1]);
        write!(&mut file, "{}\t{}\n", x, y).map_err(Error::make_error_io)?;
    }

    Ok(())
}
