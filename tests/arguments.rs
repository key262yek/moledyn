// Random Target Search with n ptls
// System : Continuous Circular System
// Target : Target in Bulk of Continuous system (Exactly at center (0; dim))
// Searcher : Passive(Brownian) Independent searchers
// Argument Test이자, Simulation Example 의 역할

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


fn main() -> Result<(), Error>{
    setup_simulation!(args, 15, 1, MFPTAnalysis, dataset, SimulationData,
        sys_arg, ContCircSystem, target_arg, ContBulkTarget,
        searcher_arg, ContPassiveIndepSearcher, time_arg, ConstStep, sim_arg, Simulation);

    let sys_size    = sys_arg.sys_size;
    let dim         = sys_arg.dim;

    let _target_pos  = target_arg.target_pos.clone();
    let target_size = target_arg.target_size;

    let _mtype       = searcher_arg.mtype;
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
    export_simulation_info!(dataset, output_dir, writer, WIDTH, "RTS_N_PTL_MERGEABLE_SEARCHER",
                            ContCircSystem, sys, sys_arg,
                            ContBulkTarget, target, target_arg,
                            ContPassiveIndepSearcher, vec_searchers, searcher_arg,
                            ConstStep, timeiter, time_arg,
                            Simulation, simulation, sim_arg);

    let mut single_move = Position::<f64>::new(vec![0f64; dim]);
    let mut list_searchers : LinkedList<ContPassiveIndepSearcher> = LinkedList::from(vec_searchers);

    for _i in 0..num_ensemble{
        let mut fpt : f64 = 0f64;

        for s in &mut list_searchers.contents{
            s.renew_uniform(&sys, &target, &mut rng)?;
        }
        list_searchers.connect_all()?;

        'outer : for time in timeiter{
            list_searchers.into_iter();
            while let Some(searcher) = list_searchers.get_mut(){
                single_move.clear();
                searcher.random_move_to_vec(&mut rng, dt, &mut single_move)?;
                sys.check_bc(&mut searcher.pos, &mut single_move)?;
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


