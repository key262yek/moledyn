
use moledyn::system_mod::{SystemCore, cont_circ::ContCircSystem};
use moledyn::position::{Position, Numerics};
use moledyn::error::{Error, ErrorCode};
use std::f64::consts::PI;


#[test]
#[ignore]
fn test_cont_circ_reflection() -> Result<(), Error>{
    // Reflective boundary condition check가 잘 이뤄지는지 확인하는 테스트
    let sys : ContCircSystem = ContCircSystem::new(1.0, 2);

    for i in 0..10{
        let x : f64 = 0.05f64 + i as f64 * 0.1f64;
        let pos : Position<f64> = Position::new(vec![x, 0.0]);

        for j in 0..30{
            let y : f64 = j as f64 * 0.001f64;
            for k in 0..10{
                let t : f64 = k as f64 * PI * 0.1f64;
                let dp : Position<f64> = Position::new(vec![y * t.cos(), y * t.sin()]);
                single_test_cont_circ_exact(sys, &pos, &dp)?;
            }
        }
    }

    return Ok(());
}

#[test]
#[ignore]
fn test_check_bc_realization() -> Result<(), Error>{
    // 실제 데이터를 확인하기 위해 reflection 결과를 출력해주는 test
    use std::fs::File;
    use std::io::Write;
    use moledyn::system_mod::cont_circ::check_bc_exact;

    let mut file = File::create("tests/images/check_bc_realization.dat").map_err(Error::make_error_io)?;
    let sys : ContCircSystem = ContCircSystem::new(1.0, 2);

    for i in 9..10{
        let x : f64 = 0.05f64 + i as f64 * 0.1f64;
        let pos : Position<f64> = Position::new(vec![x, 0.0]);

        for j in 0..80{
            let y : f64 = j as f64 * 0.01f64;
            for k in 0..10{
                let t : f64 = k as f64 * PI * 0.1f64;
                let dp : Position<f64> = Position::new(vec![y * t.cos(), y * t.sin()]);

                let mut pos1 = pos.clone();
                let mut dp1 = dp.clone();
                check_bc_exact(sys, &mut pos1, &mut dp1)?;

                let mut pos2 = pos.clone();
                let mut dp2 = dp.clone();
                sys.check_bc(&mut pos2, &mut dp2)?;

                let dx2 : f64 = (&pos1 - &pos2).norm();

                if dx2 < 1e-10{
                    continue;
                }

                write!(&mut file, "{0:.5e} {1:.5e} {2:.5e} {3:.5e}\n", pos, (&pos + &dp), pos1, pos2)
                        .map_err(Error::make_error_io)?;
            }
        }
    }

    return Ok(());
}

#[test]
#[ignore]
fn test_error_check_bc()-> Result<(), Error>{
    // 여러 버젼의 boundary condition check들간의 차이를 확인하는 테스트
    use std::fs::File;
    use std::io::Write;
    use moledyn::system_mod::cont_circ::{check_bc_exact, check_bc_first_order};

    let mut file = File::create("tests/images/check_bc_errors.dat").map_err(Error::make_error_io)?;
    let sys : ContCircSystem = ContCircSystem::new(1.0, 2);

    for i in 0..10{
        let x : f64 = 0.05f64 + i as f64 * 0.1f64;
        let pos : Position<f64> = Position::new(vec![x, 0.0]);

        for j in 0..10{
            let y : f64 = j as f64 * 0.001f64;
            for k in 0..10{
                let t : f64 = k as f64 * PI * 0.1f64;
                let dp : Position<f64> = Position::new(vec![y * t.cos(), y * t.sin()]);

                let mut pos1 = pos.clone();
                let mut dp1 = dp.clone();
                check_bc_exact(sys, &mut pos1, &mut dp1)?;

                let mut pos2 = pos.clone();
                let mut dp2 = dp.clone();
                sys.check_bc(&mut pos2, &mut dp2)?;

                let mut pos3 = pos.clone();
                let mut dp3 = dp.clone();
                check_bc_first_order(sys, &mut pos3, &mut dp3)?;


                let dx : f64 = (&pos1 - &pos2).norm();
                let dx2 : f64 = (&pos1 - &pos3).norm();

                if dx < 1e-10 && dx2 < 1e-10{
                    continue;
                }
                write!(&mut file, "{2:.5e} {3:.5e} {0:.5e} {1:.5e}\n", pos.norm(), dp.norm(), dx , dx2)
                        .map_err(Error::make_error_io)?;
            }
        }
    }

    return Ok(());
}

fn single_test_cont_circ_exact(sys: ContCircSystem, pos: &Position<f64>, dp: &Position<f64>) -> Result<(), Error>{
    // 가장 정확한 boundary condition checker의 각 계산 단계를 검증하는 테스트
    let pos2 : Position<f64> = pos + dp;
    if sys.check_inclusion(&pos2)?{
        return Ok(());
    }

    let r0 = sys.sys_size;
    let r = pos.norm();
    let dr = dp.norm();
    let rdr = pos.inner_product(dp)?;
    let t =  (- rdr + (rdr * rdr + dr * dr * (r0 * r0 - r * r)).sqrt()) / (dr * dr);
    let k = (1f64 - t) * (rdr + t * dr * dr) / (r0 * r0);

    // s = pos + t dp should on the boundary.
    let s : Position<f64> = pos + &dp.scalar_mul(t);
    assert!((s.norm() - r0).abs() < 1e-10);

    // (1-t)dp - k s should be perpendicular to s
    let dx : Position<f64> = &dp.scalar_mul(1f64 - t) - &s.scalar_mul(k);
    let inner : f64 = dx.inner_product(&s)?;
    assert!(inner.abs() < 1e-10);

    let pos2 = &pos.scalar_mul(1f64 - 2f64 * k) + &dp.scalar_mul(1f64 - 2f64 * k * t);
    if sys.check_inclusion(&pos2)?{
        return Ok(());
    }
    return Err(Error::make_error_syntax(ErrorCode::TooLargeTimeStep));
}

