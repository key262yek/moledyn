use rts::prelude::*;
use rts::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
use rts::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};
use rts::searcher_mod::{Passive, cont_passive_indep::{ContPassiveIndepSearcher, ContPassiveIndepSearcherArguments}};
use rts::time_mod::{ConstStep, ConstStepArguments};

// Dataset
construct_dataset!(SimulationData, ContCircSystem, sys_arg, ContCircSystemArguments,
                [sys_size, f64, dim, usize ];
                ContBulkTarget, target_arg, ContBulkTargetArguments,
                [target_size, f64];
                ContPassiveIndepSearcher, searcher_arg, ContPassiveIndepSearcherArguments,
                [num_searcher, usize];
                ConstStep, time_arg, ConstStepArguments,
                [dt, f64];
                Simulation, sim_arg, SimulationArguments,
                [idx_set, usize]);

#[test]
fn test_setup() -> Result<(), Error>{
    // let args: Vec<String> = vec!["10", "2", "0,0", "1", "1.0", "Uniform", "100", "1e-3", "100", "1", "12314", "tests/images/test_setup"].iter().map(|x| x.to_string()).collect();
    let args : Vec<String> = vec!["1e-10", "1", "100", "tests/images/test_setup"].iter().map(|x| x.to_string()).collect();

    setup_simulation_fixed!(args, 15, 0, MFPTAnalysis, dataset, SimulationData,
        sys_arg, ContCircSystem, target_arg, ContBulkTarget,
        searcher_arg, ContPassiveIndepSearcher,
        time_arg, ConstStep, sim_arg, Simulation);

    let sys_size    = sys_arg.sys_size;
    let dim         = sys_arg.dim;

    let _target_pos  = target_arg.target_pos.clone();
    let target_size = target_arg.target_size;

    let mtype       = searcher_arg.mtype;
    let _itype       = searcher_arg.itype.clone();
    let num_searcher= searcher_arg.num_searcher;

    let dt          = time_arg.dt;

    let num_ensemble= sim_arg.num_ensemble;
    let idx_set     = sim_arg.idx_set;
    let seed        = sim_arg.seed;
    let output_dir  = sim_arg.output_dir.clone();

    // Hash seed and generate random number generator
    let seed : u128 = seed + (628_398_227f64 * sys_size +
                              431_710_567f64 * dim as f64 +
                              277_627_711f64 * target_size +
                              719_236_607f64 * num_searcher as f64 +
                              570_914_867f64 * idx_set as f64).floor() as u128;
    let mut rng : Pcg64 = rng_seed(seed);

    // System initiation
    export_simulation_info!(dataset, output_dir, writer, WIDTH, "RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE",
                            ContCircSystem, sys, sys_arg,
                            ContBulkTarget, target, target_arg,
                            ContPassiveIndepSearcher, searcher, searcher_arg,
                            Simulation, simulation, sim_arg);

    let mut single_move = Position::<f64>::new(vec![0f64; dim]);

    for _i in 0..num_ensemble{
        let mut fpt : f64 = std::f64::MAX;  // First Passage Time
        for _j in 0..num_searcher{          // Ordered statistic
            let mut time : f64 = 0f64;      // Time to find target of single ptl
            let mut searcher = ContPassiveIndepSearcher::new_uniform(&sys, &target, &mut rng, mtype)?;

            while !target.check_find(&searcher.pos)? && time < fpt{
                single_move.clear();                                                    // Clear Displacement
                searcher.random_move_to_vec(&mut rng, dt, &mut single_move)?;               // Get random walk
                sys.check_bc(&mut searcher.pos, &mut single_move)?;                      // Check bc and move
                time += dt;                                                              // Time flows
            }

            if time < fpt{
                fpt = time;
            }
        }

        // Export FPT data
        write!(&mut writer, "{0:.5e}\n", fpt).map_err(Error::make_error_io)?;
        writer.flush().map_err(Error::make_error_io)?;
    }
    return Ok(());
}
