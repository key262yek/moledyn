use rts::prelude::*;
use rts::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
use rts::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};
use rts::searcher_mod::{Passive, cont_passive_indep::{ContPassiveIndepSearcher, ContPassiveIndepSearcherArguments}};

pub struct Simulation{
    num_searcher : usize,
    dt : f64,
    num_ensemble : usize,
    idx_set : usize,
    seed : u128,
    output_dir : String,
}

impl_argument_trait!(Simulation, "Simulation", SimulationArguments, 6;
    num_searcher, usize, "Number of Searcher",
    dt, f64, "Dimensionless Time Step Size",
    num_ensemble, usize, "Number of Ensemble",
    idx_set, usize, "Index of Ensemble Set",
    seed, u128, "Initial Seed for Random Number Generator",
    output_dir, String, "Directory containing output file");

impl Convert<SimulationArguments> for Simulation{
    fn convert_from(argument : &SimulationArguments) -> Self{
        Self{
            num_searcher    : argument.num_searcher,
            dt              : argument.dt,
            num_ensemble    : argument.num_ensemble,
            idx_set         : argument.idx_set,
            seed            : argument.seed,
            output_dir      : argument.output_dir.clone(),
        }
    }
}


// Dataset
construct_dataset!(SimulationData, ContCircSystem, sys_arg, ContCircSystemArguments,
                [sys_size, f64, dim, usize ];
                ContBulkTarget, target_arg, ContBulkTargetArguments,
                [target_size, f64];
                ContPassiveIndepSearcher, searcher_arg, ContPassiveIndepSearcherArguments, [];
                Simulation, sim_arg, SimulationArguments,
                [num_searcher, usize]);

#[test]
fn test_setup() -> Result<(), Error>{
    // let args: Vec<String> = vec!["10", "2", "0,0", "1", "1.0", "Uniform", "100", "1e-3", "100", "1", "12314", "tests/images/test_setup"].iter().map(|x| x.to_string()).collect();
    let args : Vec<String> = vec!["1e-10", "1", "100", "tests/images/test_setup"].iter().map(|x| x.to_string()).collect();

    setup_simulation_fixed!(args, 15, 0, MFPTAnalysis, SimulationData,
        sys_arg, ContCircSystem, target_arg, ContBulkTarget,
        searcher_arg, ContPassiveIndepSearcher, sim_arg, Simulation);

    let sys_size    = sys_arg.sys_size;
    let dim         = sys_arg.dim;

    let _target_pos  = target_arg.target_pos.clone();
    let target_size = target_arg.target_size;

    let mtype       = searcher_arg.mtype;
    let _itype       = searcher_arg.itype.clone();

    let num_searcher= sim_arg.num_searcher;
    let dt          = sim_arg.dt;
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

    // Create output directory, file
    fs::create_dir_all(&output_dir).map_err(Error::make_error_io)?;
    let filename : String = format!("{}", format_args!("RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_{}_DIM_{}_TARGET_SIZE_{}_NUMBER_OF_SEARCHER_{}_SET_{}.dat", sys_size, dim, target_size, num_searcher, idx_set));
    let output = fs::File::create(format!("{}/{}", output_dir, filename)).map_err(Error::make_error_io)?;
    let mut writer = BufWriter::new(&output);

    // System initiation
    export_simulation_info!(writer, WIDTH, ContCircSystem, sys, sys_arg,
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
