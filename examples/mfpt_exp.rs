
// use std::env;
use rts::prelude::*;

fn main() -> Result<(), Error>{
    // System arguments : (sys_size) (dim)
    // Target arguments : (target_pos) (target_size)
    // Searcher arguments : (mtype) (itype) (gamma) (exp_dim) (strength) (num_searcher)
    // Time Iterator arguments : (dt_min) (dt_max) (length) (tmax)
    // Variable Simulation arguments : (num_ensemble) (idx_set) (seed) (output_dir)

    // let args : Vec<String> = ["10", "2", "0:0", "1", "1.0", "Uniform", "0.2", "1.0", "1000", "1e-10", "1e-5", "10", "100", "100", "1", "12314123", "datas/benchmark"].iter().map(|x| x.to_string()).collect();

    simulation!("RTS_N_PTL_EXP_SEARCHER", TimeAnalysis,
        ContCircSystem, ContBulkTarget, ContPassiveExpSearcher,
        ExponentialStep, VariableSimulation);

    if dim != exp_dim{
        panic!("Invalid arguments input : system dimension is differ from interaction dimension");
    }

    // Hash seed and generate random number generator
    let seed : u128 = seed + (628_398_227f64 * sys_size +
                              431_710_567f64 * dim as f64 +
                              277_627_711f64 * target_size +
                              719_236_607f64 * num_searcher as f64 +
                              917_299_259f64 * strength +
                              367_276_621f64 * gamma +
                              570_914_867f64 * idx_set as f64).floor() as u128;
    let mut rng : Pcg64 = rng_seed(seed);

    let mut distance : f64;
    let mut force : f64;
    let mut displacement = Position::new(vec![0f64; dim]);
    let mut single_moves = LinkedList::from(vec![Position::new(vec![0f64; dim]); num_searcher]);
    let mut list_searchers : LinkedList<ContPassiveExpSearcher> = LinkedList::from(vec_searchers);

    let limit : f64 = if 0.1 * target_size > gamma { gamma } else { 0.1 * target_size };

    for _i in 0..num_ensemble{
        let mut fpt : f64 = 0f64;

        for s in &mut list_searchers.contents{
            s.renew_uniform(&sys, &target, &mut rng)?;
        }
        list_searchers.connect_all()?;

        'outer : for (time, dt) in timeiter.into_diff().skip(1){
            single_moves.clear();
            list_searchers.into_double_iter();
            while let Some((idx1, s1, idx2, s2)) = list_searchers.enumerate_double(){
                distance = s1.mutual_displacement_to_vec(&s2, &mut displacement)?;
                force = s1.force(distance) * dt;
                displacement.mut_scalar_mul(force);

                single_moves.contents[idx1].mut_sub(&displacement);
                single_moves.contents[idx2].mut_add(&displacement);
            }


            list_searchers.into_iter();
            while let Some((idx, searcher)) = list_searchers.enumerate_mut(){
                let single_move = &mut single_moves.contents[idx];
                searcher.random_move_to_vec(&mut rng, dt, single_move)?;

                // limit maximum displacement
                let disp = single_move.norm() / limit;
                if disp > 1f64 {
                    single_move.mut_scalar_mul(1f64 / disp);
                }

                sys.check_bc(&mut searcher.pos, single_move)?;
                if target.check_find(&searcher.pos)?{
                    fpt = time;
                    break 'outer;
                }
            }
        }

        // Export FPT data
        write!(&mut writer, "{0:.5e}\n", fpt).map_err(Error::make_error_io)?;
        writer.flush().map_err(Error::make_error_io)?;
    }

    return Ok(());
}
