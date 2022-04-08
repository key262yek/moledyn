
use crate::prelude::*;


#[macro_export]
// #[allow(unused_macros)]
macro_rules! export_simulation_item{
    ($name_args:ident, $analysis:ty $(,$var:ty)*) =>{
        eprint!("{} arguments given.\nGiven Arguments : ", $name_args.len() - NUM_SKIP);
        for x in $name_args.iter().skip(NUM_SKIP){
            eprint!("{}  ", x);
        }
        eprintln!("\nYou should give {} arguments like below", TOTAL_NUM_ARGS);
        eprintln!("======================= OVERVIEW OF ARGUMENTS ==========================");
        $(eprintln!("{}", <$var>::brief_info());
            )*
        eprintln!("========================    DESCRIPTIONS     ==========================");
        $(eprintln!("{}", <$var>::info(WIDTH));
            )*
        eprintln!("========================    FOR ANALYSIS    ===========================");
        eprintln!("{}", <$analysis>::brief_info());
        eprint!("{}", <$analysis>::info(WIDTH));
        return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments));
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! define_total_num_args{
    ($($type:ty),*) =>{
        const TOTAL_NUM_ARGS : usize = $(<$type>::NUM_ARGS + )* 0;
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! read_arguments {
    ($name_args:ident $(, $ident_arg:ident, $argument_type:ty)*) => {
        let mut _idx : usize = NUM_SKIP;
        $(
            let $ident_arg = <$argument_type>::read_args_from_vec(&$name_args[_idx.._idx+<$argument_type>::NUM_ARGS])?;
            _idx += <$argument_type>::NUM_ARGS;
            )*
    };
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! export_simulation_info {
    ($dataset:ident, $dir:ident, $writer:ident, $width:ident, $prefix:expr  $(, $struct_type:ty, $struct_name:ident, $argument_name:ident)*) => {

        $(
            let $struct_name = <$struct_type>::convert_from(&$argument_name);
            )*

        fs::create_dir_all(&$dir).map_err(Error::make_error_io)?;
        let filename : String = $dataset.export_file($prefix);
        let output = fs::File::create(format!("{}/{}", $dir, filename)).map_err(Error::make_error_io)?;
        let mut $writer = BufWriter::new(&output);

        write!(&mut $writer, "========================    DESCRIPTIONS    ==========================\n")
                    .map_err(Error::make_error_io)?;
        $(
            write!(&mut $writer, "{}", $argument_name.print_configuration($width)).map_err(Error::make_error_io)?;
            )*
        write!(&mut $writer, "{}", "========================     DATA STARTS    ==========================\n")
                    .map_err(Error::make_error_io)?;
        $writer.flush().map_err(Error::make_error_io)?;
    };
}


#[macro_export]
// #[allow(unused_macros)]
macro_rules! setup_simulation{
    ($args:ident, $width:expr, $skip:expr, $analysis:ty, $prefix:expr, $ds_name:ident, $dataset:ty $(, $arg_name:ident, $struct_type:ty)*) =>{

        let $args : Vec<String> = std::env::args().collect();
        const WIDTH : usize = $width;
        const NUM_SKIP : usize = $skip;

        define_total_num_args!($($struct_type),*);
        if $args.len() - NUM_SKIP == <$analysis>::NUM_ARGS || $args.len() - NUM_SKIP == <$analysis>::NUM_ARGS + 1{
                return <$analysis>::analyze::<$dataset>(&$args[NUM_SKIP..], WIDTH, $prefix);
        }
        else if $args.len() - NUM_SKIP != TOTAL_NUM_ARGS{
            export_simulation_item!($args, $analysis $(, $struct_type)*);
        }

        read_arguments!($args $(, $arg_name, $struct_type)*);
        let $ds_name = <$dataset>::new($(&$arg_name),*);
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! setup_simulation_fixed{
    ($args:ident, $width:expr, $skip:expr, $analysis:ty, $prefix:expr, $ds_name:ident, $dataset:ty $(, $arg_name:ident, $struct_type:ty)*) =>{

        const WIDTH : usize = $width;
        const NUM_SKIP : usize = $skip;

        define_total_num_args!($($struct_type),*);

        if $args.len() - NUM_SKIP == <$analysis>::NUM_ARGS || $args.len() - NUM_SKIP == <$analysis>::NUM_ARGS + 1{
                return <$analysis>::analyze::<$dataset>(&$args, WIDTH, $prefix);
        }
        else if $args.len() - NUM_SKIP != TOTAL_NUM_ARGS{
            export_simulation_item!($args, $analysis $(, $struct_type)*);
        }

        read_arguments!($args $(, $arg_name, $struct_type)*);
        let $ds_name = <$dataset>::new($(&$arg_name),*);
    }
}

pub struct VariableSimulation{
    pub num_ensemble : usize,
    pub idx_set : usize,
    pub seed : u128,
    pub output_dir : String,
}

impl_argument_trait!(VariableSimulation, "Variable Simulation", VariableSimulationArguments, 4;
    num_ensemble, usize, "Number of Ensemble",
    idx_set, usize, "Index of Ensemble Set",
    seed, u128, "Initial Seed for Random Number Generator",
    output_dir, String, "Directory containing output file");

impl VariableSimulation{
    #[allow(dead_code)]
    pub fn convert_from(argument : &VariableSimulationArguments) -> Self{
        Self{
            num_ensemble    : argument.num_ensemble,
            idx_set         : argument.idx_set,
            seed            : argument.seed,
            output_dir      : argument.output_dir.clone(),
        }
    }
}

pub struct ParVariableSimulation{
    pub num_ensemble : usize,
    pub seed : u128,
    pub n_thrd : usize,
    pub output_dir : String,
}

impl_argument_trait!(ParVariableSimulation, "Variable Simulation", ParVariableSimulationArguments, 4;
    num_ensemble, usize, "Number of Ensemble",
    seed, u128, "Initial Seed for Random Number Generator",
    n_thrd, usize, "Number of Thread",
    output_dir, String, "Directory containing output file");

impl ParVariableSimulation{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ParVariableSimulationArguments) -> Self{
        Self{
            num_ensemble    : argument.num_ensemble,
            seed            : argument.seed,
            n_thrd          : argument.n_thrd,
            output_dir      : argument.output_dir.clone(),
        }
    }
}

pub struct ProcessSimulation{
    pub num_ensemble : usize,
    pub period : f64,
    pub idx_set : usize,
    pub seed : u128,
    pub output_dir : String,
}

impl_argument_trait!(ProcessSimulation, "Process Simulation", ProcessSimulationArguments, 5;
    num_ensemble, usize, "Number of Ensemble",
    period, f64, "Time between recording",
    idx_set, usize, "Index of Ensemble Set",
    seed, u128, "Initial Seed for Random Number Generator",
    output_dir, String, "Directory containing output file");

impl ProcessSimulation{
    #[allow(dead_code)]
    pub fn convert_from(argument : &ProcessSimulationArguments) -> Self{
        Self{
            num_ensemble    : argument.num_ensemble,
            period          : argument.period,
            idx_set         : argument.idx_set,
            seed            : argument.seed,
            output_dir      : argument.output_dir.clone(),
        }
    }
}



#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // #[ignore]
    // fn test_hash_input(){
    //     // macro 결과를 확인하기 편하게 하기 위해 먼저 macro 내부 코드 내용에 println!을 씌워 확인하기로 하였다.
    //     assert_eq!(hash_input!(ContCircSystem), "ContCircSystem, sys_arg, ContCircSystemArguments, [sys_size, f64, dim, usize]");
    //     assert_eq!(hash_input!(ContCubicSystem), "ContCubicSystem, sys_arg, ContCubicSystemArguments, [bctype, BoundaryCond, sys_size, f64, dim, usize]");
    // }
}

