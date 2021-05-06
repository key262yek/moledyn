// Functionality for analysis


// =====================================================================================
// ===  Implement DataSet ==============================================================
// =====================================================================================

use crate::prelude::*;

pub trait DataSet{
    // Read data set info fromfile
    fn from_file<P>(path : P) -> Result<(Self, Lines<BufReader<File>>), Error>
        where Self : Sized, P : AsRef<Path>;

    // File name corresponding to dataset
    fn export_file(&self, prefix : &str) -> String;

    // File name corresponding to dataset
    fn export_file_removed_idx(&self, prefix : &str) -> String;

    // Explaination of each column in data file
    fn export_form(width: usize) -> String;

    // Data value export
    fn export_data(&self, prec : usize) -> Result<String, Error>;
}


#[macro_export]
#[allow(unused_macros)]
macro_rules! hash_input{
    // System Types
    (ContCircSystem) => {
        ContCircSystem, sys_arg, ContCircSystemArguments, [sys_size, f64, dim, usize]
    };
    (ContCubicSystem) => {
        ContCubicSystem, sys_arg, ContCubicSystemArguments, [sys_size, f64, dim, usize]
    };
    (ContCylindricalSystem) => {
        ContCylindricalSystem, sys_arg, ContCylindricalSystemArguments, [radius, f64, length, f64, dim, usize]
    };

    // Target Types
    (ContBoundaryTarget) => {
        ContBoundaryTarget, target_arg, ContBoundaryTargetArguments, [target_size, f64]
    };
    (ContBulkTarget) => {
        ContBulkTarget, target_arg, ContBulkTargetArguments, [target_size, f64]
    };

    // Searcher Types
    (ContPassiveIndepSearcher) => {
        ContPassiveIndepSearcher, searcher_arg, ContPassiveIndepSearcherArguments, [num_searcher, usize]
    };
    (ContPassiveMergeSearcher) => {
        ContPassiveMergeSearcher, searcher_arg, ContPassiveMergeSearcherArguments, [radius, f64, alpha, f64, num_searcher, usize]
    };
    (ContPassiveExpSearcher) => {
        ContPassiveExpSearcher, searcher_arg, ContPassiveExpSearcherArguments, [gamma, f64, strength, f64, num_searcher, usize]
    };

    // TimeStep Types
    (ConstStep) => {
        ConstStep, time_arg, ConstStepArguments, [dt, f64, tmax, f64]
    };
    (ExponentialStep) => {
        ExponentialStep, time_arg, ExponentialStepArguments, [dt_min, f64, dt_max, f64, length, f64, tmax, f64]
    };

    // Simulation Types
    (Simulation) => {
        Simulation, sim_arg, SimulationArguments, [idx_set, usize]
    };

    ($name : ty, $($rem : ty), +) => {
        hash_input!($name); hash_input!($($rem), +)
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_dataset_recursive{
    ([$($t : tt), *; $t2 : tt]) => {
        construct_dataset!($($t), * ; $t2);
    };
    ($($name : ty),+ ; $sim : ty) => {
        construct_dataset_recursive!($($name),+ ; $sim :ty [SimulationData]);
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_dataset {
    ( $name:ident, $( $struct_type:ty, $arg_name:ident, $arg_type:ty, [$($var:ident, $t:ty),*] );*
        $(;{$sim_type:ty, $sim_arg_name:ident, $sim_arg_type:ty, [$($sim_var:ident, $sim_t:ty),*]})?) => {

        define_structure_wo_eq!($name; $($($var, $t,)*)* $($($sim_var, $sim_t,)*)?);

        impl $name{
            // Since argument infos are different for different data form
            // we should define a function 'new' out of trait
            #[allow(dead_code, unused_variables)]
            pub fn new($($arg_name : &$arg_type), * $(,$sim_arg_name : &$sim_arg_type)?) -> Self{
                $name{
                    $(
                        $($var : $arg_name.$var,
                            )*
                    )*
                    $($(
                        $sim_var : $sim_arg_name.$sim_var,
                    )*)?
                }
            }
        }

        impl DataSet for $name{
            #[allow(dead_code)]
            fn from_file<P>(path : P) -> Result<(Self, Lines<BufReader<File>>), Error>
                where P : AsRef<Path>{
                let f = File::open(path).map_err(Error::make_error_io)?;
                let f = BufReader::new(f);
                let mut lines = f.lines();
                lines.next();

                $(
                    let $arg_name = <$struct_type>::read_args_from_lines(&mut lines)?;
                )*
                $(let $sim_arg_name = <$sim_type>::read_args_from_lines(&mut lines)?;)?


                Ok(($name::new($(&$arg_name),* $(,&$sim_arg_name)?), lines))
            }

            #[allow(dead_code)]
            fn export_file(&self, prefix : &str) -> String{
                let mut string = String::from(prefix);
                $(
                    $(
                        string.push_str(format!("{}", format_args!("_{}_{}", stringify!($var), self.$var)).as_str());
                    )*
                )*
                $($(
                    string.push_str(format!("{}", format_args!("_{}_{}", stringify!($sim_var), self.$sim_var)).as_str());
                )*)?
                string.push_str(".dat");
                return string;
            }

            #[allow(dead_code)]
            fn export_file_removed_idx(&self, prefix : &str) -> String{
                let mut string = String::from(prefix);
                $(
                    $(
                        string.push_str(format!("{}", format_args!("_{}_{}", stringify!($var), self.$var)).as_str());
                    )*
                )*
                string.push_str(".dat");
                return string;
            }

            export_form!(export_form $($(, $var)*)*);
            export_data!(export_data $($(, $var)*)*);
        }

        impl Copy for $name{
        }

        derive_hash!($name $($(, $var)*)*);

        impl PartialEq for $name{
            fn eq(&self, other: &Self) -> bool {
                fn calculate_hash<T: Hash>(t: &T) -> u64 {
                    let mut s = std::collections::hash_map::DefaultHasher::new();
                    t.hash(&mut s);
                    s.finish()
                }
                calculate_hash(&self) == calculate_hash(other)
            }
        }

        impl Eq for $name{
        }
    };
}



// =====================================================================================
// ===  Trait for Analysis =============================================================
// =====================================================================================

pub trait Bin
    where Self : Sized{
    // Define Analysis from bin sizes
    fn update_from_bin_size(&mut self, min_time : f64, max_time : f64, bin_size : f64, lbin_size : f64) -> Result<(), Error>;

    // Define Analysis from number of bin
    fn update_from_num_bin(&mut self, min_time : f64, max_time : f64, num_bin : usize) -> Result<(), Error>;

    // Convert number of bins to bin sizes
    fn convert_num_bin_to_bin_size(min_time : f64, max_time : f64, num_bin : usize) -> Result<(f64, f64), Error>;

    // Find binning position of data
    fn bin_pos(&self, value : f64) -> Option<usize>;
    fn lbin_pos(&self, value : f64) -> Option<usize>;

    // Allocate vectors
    fn allocate_vectors(&mut self);
}


#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_trait_bin {
() => {
        #[allow(dead_code)]
        fn update_from_bin_size(&mut self, min_time : f64, max_time : f64, bin_size : f64, lbin_size : f64) -> Result<(), Error>{
            if min_time < 0f64 || max_time < min_time || bin_size < 0f64 || lbin_size < 0f64{
                return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
            }

            let time : f64;
            if min_time < 1e-15{
                time = 1e-15;
            }
            else{
                time = min_time;
            }

            let num_bin : usize = ((max_time - time) / bin_size + 0.5f64).ceil() as usize;
            let num_lbin : usize = ((max_time / time).log2() / lbin_size.log2() + 0.5f64).ceil() as usize;

            self.min_time = time;
            self.max_time = max_time;
            self.bin_size = bin_size;
            self.lbin_size = lbin_size;
            self.num_bin = num_bin;
            self.num_lbin = num_lbin;
            return Ok(());
        }

        #[allow(dead_code)]
        fn update_from_num_bin(&mut self, min_time : f64, max_time : f64, num_bin : usize) -> Result<(), Error>{
            if min_time < 0f64 || max_time < min_time || num_bin < 10{
                return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
            }

            let time : f64;
            if min_time < 1e-15{
                time = 1e-15;
            }
            else{
                time = min_time;
            }

            let (bin_size, lbin_size) : (f64, f64) = Self::convert_num_bin_to_bin_size(time, max_time, num_bin)?;

            self.min_time = time;
            self.max_time = max_time;
            self.bin_size = bin_size;
            self.lbin_size = lbin_size;
            self.num_bin = num_bin;
            self.num_lbin = num_bin;
            return Ok(());
        }

        fn convert_num_bin_to_bin_size(min_time : f64, max_time : f64, num_bin : usize) -> Result<(f64, f64), Error>{
            if min_time < 0f64 || max_time < min_time || num_bin < 10{
                return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
            }

            let time : f64;
            if min_time < 1e-15{
                time = 1e-15;
            }
            else{
                time = min_time;
            }

            let bin_size : f64 = (max_time - time) / (num_bin as f64);
            let lbin_size : f64 = (max_time / time).powf(1f64 / (num_bin as f64));
            return Ok((bin_size, lbin_size));
        }

        #[allow(dead_code)]
        fn bin_pos(&self, fpt : f64) -> Option<usize>{
            if fpt < self.min_time || self.max_time < fpt{
                return None;
            }
            else{
                return Some(((fpt - self.min_time) / self.bin_size).floor() as usize);
            }
        }

        #[allow(dead_code)]
        fn lbin_pos(&self, fpt : f64) -> Option<usize>{
            if fpt < self.min_time || self.max_time < fpt{
                return None;
            }
            else{
                return Some(((fpt / self.min_time).log2() / self.lbin_size.log2()).floor() as usize);
            }
        }
    }
}

pub trait Var1
    where Self : Sized + Bin{
    // Single variable analysis.

    // Add ensemble data
    fn add_ensemble(&mut self, value : f64);

    // Draw distribution from histogram
    fn draw(&mut self);
}

pub trait VarN
    where Self : Sized + Bin{
    // N variable analysis.

    // Add ensemble data
    fn add_ensemble(&mut self, values : Vec<f64>);

    // Draw distribution from histogram
    fn draw(&mut self);
}

pub trait VarTime
    where Self : Sized + Bin{
    // time vs variable analysis.

    fn add_ensemble(&mut self);

    // Add data
    fn add_pair(&mut self, pair : Pair<f64>);

    // Draw distribution
    fn draw(&mut self);
}


pub trait Analysis{
    const NUM_ARGS : usize;

    // Clear data
    fn clear(&mut self);

    // Export analysis info - needed inputs
    fn info(width : usize) -> String;

    // Brief info for analysis
    fn brief_info() -> String;

    // export form for summary file (explaination of each column in summary file)
    fn export_form(width : usize) -> String;

    // Export datas
    fn export_mean_stddev(&self, prec: usize) -> Result<String, Error>;
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>;
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>;

    // export data to summary file
    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>;

    // analysis
    fn analyze<H : Hash + Eq + Copy + DataSet>(args : &[String], width : usize, prefix : &str) -> Result<(), Error>;
}

// =====================================================================================
// ===  Implement TimeAnalysis =========================================================
// =====================================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct TimeAnalysis{
    mean : f64,             // Sum of fpt data
    stddev : f64,           // Sum of fpt stddevs
    ensemble : usize,       // Number of ensemble
    min_time : f64,         // Minimal time for logarithmic histogram
    max_time : f64,         // Maximal time for storing
    bin_size : f64,         // Size of bin for linear histogram
    lbin_size : f64,        // Size of bin for logarithmic histogram
    num_bin : usize,        // Number of bin for linear histogram
    num_lbin : usize,       // Number of bin for logarithmic histogram
    hist : Vec<f64>,        // Linear Histogram. find f(t) s.t. f(t)dt = P(t < s < t + dt)
    lhist : Vec<f64>,       // Logarithmic Histogram  find f(t) with logarithmic binning
    count : Vec<usize>,     // Count of ensemble in linear range
    lcount : Vec<usize>,    // Count of ensemble in logarithmic range
}

impl TimeAnalysis{
    #[allow(dead_code)]
    fn new() -> Self{
        Self{
            mean : 0f64,
            stddev : 0f64,
            ensemble : 0usize,
            min_time : 0f64,
            max_time : 0f64,
            bin_size : 0f64,
            lbin_size : 0f64,
            num_bin : 0usize,
            num_lbin : 0usize,
            hist : Vec::new(),
            lhist : Vec::new(),
            count : Vec::new(),
            lcount : Vec::new(),
        }
    }
}

impl Default for TimeAnalysis{
    fn default() -> Self{
        TimeAnalysis{
            mean : 0f64,
            stddev : 0f64,
            ensemble : 0usize,
            min_time : 1e-15f64,
            max_time : 1e13f64,
            bin_size : 1e10f64,
            lbin_size : 1.2f64,
            num_bin : 1001usize,
            num_lbin : 355,
            hist : vec![0.0f64; 1001],
            lhist : vec![0.0f64; 355],
            count : vec![0; 1001],
            lcount : vec![0; 355],
        }
    }
}

impl Bin for TimeAnalysis{
    construct_trait_bin!();

    fn allocate_vectors(&mut self){
        self.hist = vec![0.0f64; self.num_bin];
        self.lhist = vec![0.0f64; self.num_lbin];
        self.count = vec![0; self.num_bin];
        self.lcount = vec![0; self.num_lbin];
    }
}



impl Var1 for TimeAnalysis{
    #[allow(dead_code)]
    fn add_ensemble(&mut self, fpt: f64){

        self.ensemble += 1;
        self.mean += fpt;
        self.stddev += fpt * fpt;

        match self.bin_pos(fpt){
            Some(idx) => {self.count[idx] += 1;},
            None => ()
        }
        match self.lbin_pos(fpt){
            Some(idx) => {self.lcount[idx] += 1;},
            None => ()
        }
    }

    #[allow(dead_code)]
    fn draw(&mut self){
        let en : f64= self.ensemble as f64;

        self.mean = self.mean / en;
        self.stddev = ((self.stddev / en) - self.mean.powi(2)).sqrt();

        let d_bin : f64 = en * (self.bin_size as f64);
        let lbin : f64 = self.lbin_size;
        let mut d_lbin : f64 = self.min_time * en * (lbin - 1f64);

        for (i, &x) in self.count.iter().enumerate(){
            self.hist[i] = (x as f64) / d_bin;
        }

        for (i, &x) in self.lcount.iter().enumerate(){
            self.lhist[i] = (x as f64) / d_lbin;
            d_lbin *= lbin;
        }
    }
}


impl Analysis for TimeAnalysis{
    const NUM_ARGS : usize = 4;

    #[allow(dead_code)]
    fn clear(&mut self){
        self.mean = 0f64;
        self.stddev = 0f64;
        self.ensemble = 0usize;
        self.count = vec![0; self.num_bin];
        self.lcount = vec![0; self.num_lbin];
    }

    impl_fn_brief_info!(brief_info, "Single Time", min_time, max_time, bin_size, lbin_size, output_dir);
    impl_fn_info!(info,
                  min_time, "Minimal time for Histogram",
                  max_time, "Maximal time for Histogram",
                  bin_size, "Bin size for Linear Histogram",
                  lbin_size, "Bin size for Logarithmic Histogram",
                  output_dir, "Directory for data files");
    export_form!(export_form, ensemble, mean, stddev);


    #[allow(dead_code)]
    fn export_mean_stddev(&self, prec : usize) -> Result<String, Error>{
        Ok(format!("{}", format_args!("{1:<0$e}\t{2:<0$e}\t{3:<0$}", prec, self.mean, self.stddev, self.ensemble)))
    }

    #[allow(dead_code)]
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>{
        let bin_size = self.bin_size;
        let mut time = self.min_time - bin_size / 2f64;
        for (i, &x) in self.hist.iter().enumerate(){
            time += bin_size;
            if x.abs() < 1e-15{
                continue;
            }
            let n = self.count[i];
            writer.write_fmt(format_args!("{1:0$e}\t{2:0$}\t{3:0$e}\n", prec, time, n, x)).map_err(Error::make_error_io)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>{
        let r = self.lbin_size;
        let mut time = self.min_time / r.sqrt();

        for (i, &x) in self.lhist.iter().enumerate(){
            time *= r;
            if x.abs() < 1e-15{
                continue;
            }
            let n = self.lcount[i];
            writer.write_fmt(format_args!("{1:0$e}\t{2:0$}\t{3:0$e}\n", prec, time, n, x)).map_err(Error::make_error_io)?;
        }
        Ok(())
    }

    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>{
        // Export datas
        brief_data.write(format!("{}", format_args!("{}\n", self.export_mean_stddev(prec)?)).as_bytes()).map_err(Error::make_error_io)?;

        // Export linear histogram
        let linear_filename = format!("{}", format_args!("{}/linear_distribution/{}", export_dir, filename));
        let linear = File::create(linear_filename).map_err(Error::make_error_io)?;
        let mut linear = BufWriter::new(linear);
        self.export_distribution(prec, linear.get_mut())?;

        // Export logarithmic histogram
        let log_filename = format!("{}", format_args!("{}/logarithmic_distribution/{}", export_dir, filename));
        let log = File::create(log_filename).map_err(Error::make_error_io)?;
        let mut log = BufWriter::new(log);
        self.export_log_scaled_distribution(prec, log.get_mut())?;

        Ok(())
    }

    fn analyze<H : Hash + Eq + Copy + DataSet>(args : &[String], width : usize, prefix : &str) -> Result<(), Error>{
        use chrono::offset::Utc;

        let min_time : f64;
        let max_time : f64;
        let num_bin : usize;
        let bin_size : f64;
        let lbin_size : f64;
        let data_dir : String;

        match args.len(){
            4 => {
                let mut idx : usize = 0;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                num_bin  = args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();

                let bin_info =  Self::convert_num_bin_to_bin_size(min_time, max_time, num_bin)?;
                bin_size = bin_info.0;
                lbin_size = bin_info.1;
            },
            5 => {
                let mut idx : usize = 0;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                bin_size = args[idx].parse().unwrap();      idx+=1;
                lbin_size= args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();
            },
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments))
            }
        }

        let mut hashmap : HashMap<H, Self> = HashMap::new();
        let mut summary_dir : String = format!("{}", format_args!("{}/analysis_{}",
                                    data_dir, Utc::today().format("%Y%m%d").to_string()));

        if Path::new(&summary_dir).exists(){
            let mut i : usize = 2;
            let mut new : String;
            loop{
                new = format!("{}", format_args!("{}_{}", summary_dir, i));
                if Path::new(&new).exists(){
                    i += 1;
                }
                else{
                    break;
                }
            }
            summary_dir = new.clone();
        }

        let summary_file : String = format!("{}/brief_result.dat", summary_dir);

        fs::create_dir_all(&summary_dir).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/linear_distribution", &summary_dir)).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/logarithmic_distribution", &summary_dir)).map_err(Error::make_error_io)?;

        let summary = File::create(summary_file).map_err(Error::make_error_io)?;
        let mut summary = BufWriter::new(summary);

        summary.write_fmt(format_args!("{}{}\n", H::export_form(width), Self::export_form(width)))
               .map_err(Error::make_error_io)?;

        for entry in fs::read_dir(&data_dir).map_err(Error::make_error_io)?{
            let entry = entry.map_err(Error::make_error_io)?;
            let path = entry.path();
            if path.is_dir(){
                continue;
            }
            println!("File read start : {:?}", path.clone());

            let (dataset, mut lines) : (H, Lines<BufReader<File>>) = match H::from_file(path.clone()){
                Ok(ds) => ds,
                Err(_err) => {continue;},
            };

            let analysis = match hashmap.get_mut(&dataset){
                Some(x) => x,
                None => {
                    let mut x = Self::new();
                    x.update_from_bin_size(min_time, max_time, bin_size, lbin_size)?;
                    x.allocate_vectors();
                    hashmap.insert(dataset, x);
                    hashmap.get_mut(&dataset).unwrap()
                },
            };
            lines.next();

            for line in lines{
                let line = line.map_err(Error::make_error_io)?;
                let time : f64 = line.trim().parse().unwrap();
                analysis.add_ensemble(time);
            }

            println!("File read end : {:?}", path.clone());
        }

        for (dataset, analysis) in hashmap.iter_mut(){
            analysis.draw();

            let hist_filename = dataset.export_file_removed_idx(prefix);

            summary.write_fmt(format_args!("{}", dataset.export_data(width)?)).map_err(Error::make_error_io)?;
            analysis.export(width, &mut summary, &summary_dir, &hist_filename)?;
        }
        return Ok(());
    }
}



// =====================================================================================
// ===  Implement TimeVecAnalysis ====================================================
// =====================================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct TimeVecAnalysis{       // Merge Time Analysis
    means : Vec<f64>,                     // Sum of data
    stddevs : Vec<f64>,                  // Sum of squares
    ensemble : usize,               // Number of ensemble
    min_time : f64,                 // Minimal time for logarithmic histogram
    max_time : f64,                 // Maximal time for storing
    bin_size : f64,                 // Size of bin for linear histogram
    lbin_size : f64,                // Size of bin for logarithmic histogram
    num_bin : usize,                // Number of bin for linear histogram
    num_lbin : usize,               // Number of bin for logarithmic histogram
    num_var : usize,                // Number of merging time to record. 1st to num_var-th
    hist : Vec<Vec<f64>>,           // Linear Histogram. find f(t) s.t. f(t)dt = P(t < s < t + dt)
    lhist : Vec<Vec<f64>>,          // Logarithmic Histogram  find f(t) with logarithmic binning
    count : Vec<Vec<usize>>,        // Count of ensemble in linear range
    lcount : Vec<Vec<usize>>,       // Count of ensemble in logarithmic range
}

impl TimeVecAnalysis{
    #[allow(dead_code)]
    fn new(num_var : usize) -> Self{
        Self{
            means : vec![0f64; num_var],
            stddevs : vec![0f64; num_var],
            ensemble : 0usize,
            min_time : 0f64,
            max_time : 0f64,
            bin_size : 0f64,
            lbin_size : 0f64,
            num_bin : 0usize,
            num_lbin : 0usize,
            num_var : num_var,
            hist : Vec::new(),
            lhist : Vec::new(),
            count : Vec::new(),
            lcount : Vec::new(),
        }
    }
}

impl Bin for TimeVecAnalysis{
    construct_trait_bin!();

    fn allocate_vectors(&mut self){
        self.means = vec![0f64; self.num_var];
        self.stddevs = vec![0f64; self.num_var];

        self.hist = vec![vec![0.0f64; self.num_bin]; self.num_var];
        self.lhist = vec![vec![0.0f64; self.num_lbin]; self.num_var];
        self.count = vec![vec![0; self.num_bin]; self.num_var];
        self.lcount = vec![vec![0; self.num_lbin]; self.num_var];
    }
}

impl Default for TimeVecAnalysis{
    fn default() -> Self{
        Self{
            means : vec![0f64; 10],
            stddevs : vec![0f64; 10],
            ensemble : 0usize,
            min_time : 1e-15f64,
            max_time : 1e13f64,
            bin_size : 1e10f64,
            lbin_size : 1.2f64,
            num_bin : 1001usize,
            num_lbin : 355,
            num_var : 10,
            hist : vec![vec![0.0f64; 1001]; 10],
            lhist : vec![vec![0.0f64; 355]; 10],
            count : vec![vec![0; 1001]; 10],
            lcount : vec![vec![0; 355]; 10],
        }
    }
}

impl VarN for TimeVecAnalysis{
     // Add ensemble data
    fn add_ensemble(&mut self, values : Vec<f64>){
        if self.num_var == 0{
            self.num_var = values.len();
            self.allocate_vectors();
        }
        else if self.num_var != values.len(){
            panic!("Invalid Data File");
        }

        self.ensemble += 1;
        for (idx, &time) in values.iter().enumerate(){
            self.means[idx] += time;
            self.stddevs[idx] += time * time;

            match self.bin_pos(time){
                Some(idx2) => {self.count[idx][idx2] += 1;},
                None => ()
            }
            match self.lbin_pos(time){
                Some(idx2) => {self.lcount[idx][idx2] += 1;},
                None => ()
            }
        }
    }

    // Draw distribution from histogram
    fn draw(&mut self){
        let en : f64= self.ensemble as f64;
        let d_bin : f64 = en * (self.bin_size as f64);
        let lbin : f64 = self.lbin_size;

        for idx in 0..self.num_var{
            self.means[idx] = self.means[idx] / en;
            self.stddevs[idx] = ((self.stddevs[idx] / en) - self.means[idx].powi(2)).sqrt();

            let mut d_lbin : f64 = self.min_time * en * (lbin - 1f64);

            let count = &self.count[idx];
            let hist = &mut self.hist[idx];
            for (i, &x) in count.iter().enumerate(){
                hist[i] = (x as f64) / d_bin;
            }

            let lcount = &self.lcount[idx];
            let lhist = &mut self.lhist[idx];
            for (i, &x) in lcount.iter().enumerate(){
                lhist[i] = (x as f64) / d_lbin;
                d_lbin *= lbin;
            }
        }
    }
}

impl Analysis for TimeVecAnalysis{
    const NUM_ARGS : usize = 4;

    // Clear data
    fn clear(&mut self){
        self.means = vec![0f64; self.num_var];
        self.stddevs = vec![0f64; self.num_var];
        self.ensemble = 0;

        self.count = vec![vec![0; self.num_bin]; self.num_var];
        self.lcount = vec![vec![0; self.num_lbin]; self.num_var];
    }

    impl_fn_brief_info!(brief_info, "Vector of Time", num_var, min_time, max_time, bin_size, lbin_size, output_dir);
    impl_fn_info!(info,
                  num_var, "Number of data for each ensemble",
                  min_time, "Minimal time for Histogram",
                  max_time, "Maximal time for Histogram",
                  bin_size, "Bin size for Linear Histogram",
                  lbin_size, "Bin size for Logarithmic Histogram",
                  output_dir, "Directory for data files");
    export_form!(export_form, ensemble, meantime, stddev);

    #[allow(dead_code)]
    fn export_mean_stddev(&self, prec : usize) -> Result<String, Error>{
        let mut string = String::new();
        string.push_str(format!("{}", format_args!("{1:<0$}\t", prec, self.ensemble)).as_str());
        for (mean, stddev) in self.means.iter().zip(self.stddevs.iter()){
            string.push_str(format!("{}", format_args!("{1:<0$e}\t{2:<0$e}\t", prec, mean, stddev)).as_str());
        }
        Ok(string)
    }

    #[allow(dead_code)]
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>{
        let bin_size = self.bin_size;
        let mut time = self.min_time + bin_size / 2f64;
        let mut i : usize = 0;

        while time <= self.max_time{
            let mut string = String::new();
            let mut check = false;

            string.push_str(format!("{}", format_args!("{1:0$e}\t", prec, time)).as_str());
            for idx in 0..self.num_var{
                let n = self.count[idx][i];
                let x = self.hist[idx][i];

                if x > 1e-15{
                    check = true;
                }
                string.push_str(format!("{}", format_args!("{1:0$}\t{2:0$e}\t", prec, n, x)).as_str());
            }
            if check{
                writer.write_fmt(format_args!("{}\n", string)).map_err(Error::make_error_io)?;
            }

            time += bin_size;
            i += 1;

        }
        Ok(())
    }

    #[allow(dead_code)]
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>{
        let lbin_size = self.lbin_size;
        let mut time = self.min_time * lbin_size.sqrt();
        let mut i : usize = 0;

        while time <= self.max_time{
            let mut string = String::new();
            let mut check = false;

            string.push_str(format!("{}", format_args!("{1:0$e}\t", prec, time)).as_str());
            for idx in 0..self.num_var{
                let n = self.lcount[idx][i];
                let x = self.lhist[idx][i];

                if x > 1e-15{
                    check = true;
                }
                string.push_str(format!("{}", format_args!("{1:0$}\t{2:0$e}\t", prec, n, x)).as_str());
            }
            if check{
                writer.write_fmt(format_args!("{}\n", string)).map_err(Error::make_error_io)?;
            }

            time *= lbin_size;
            i += 1;
        }
        Ok(())
    }

    // export data to summary file
    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>{

        brief_data.write(format!("{}", format_args!("{}\n", self.export_mean_stddev(prec)?)).as_bytes()).map_err(Error::make_error_io)?;

        // Export linear histogram
        let linear_filename = format!("{}", format_args!("{}/linear_distribution/{}", export_dir, filename));
        let linear = File::create(linear_filename).map_err(Error::make_error_io)?;
        let mut linear = BufWriter::new(linear);
        self.export_distribution(prec, linear.get_mut())?;

        // Export logarithmic histogram
        let log_filename = format!("{}", format_args!("{}/logarithmic_distribution/{}", export_dir, filename));
        let log = File::create(log_filename).map_err(Error::make_error_io)?;
        let mut log = BufWriter::new(log);
        self.export_log_scaled_distribution(prec, log.get_mut())?;

        Ok(())
    }

    // analysis
    fn analyze<H : Hash + Eq + Copy + DataSet>(args : &[String], width : usize, prefix : &str) -> Result<(), Error>{
        use chrono::offset::Utc;

        let num_var : usize;
        let min_time : f64;
        let max_time : f64;
        let num_bin : usize;
        let bin_size : f64;
        let lbin_size : f64;
        let data_dir : String;

        match args.len(){
            5 => {
                let mut idx : usize = 0;
                num_var  = args[idx].parse().unwrap();      idx+=1;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                num_bin  = args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();

                let bin_info =  Self::convert_num_bin_to_bin_size(min_time, max_time, num_bin)?;
                bin_size = bin_info.0;
                lbin_size = bin_info.1;
            },
            6 => {
                let mut idx : usize = 0;
                num_var  = args[idx].parse().unwrap();      idx+=1;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                bin_size = args[idx].parse().unwrap();      idx+=1;
                lbin_size= args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();
            },
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments))
            }
        }

        let mut hashmap : HashMap<H, Self> = HashMap::new();
        let mut summary_dir : String = format!("{}", format_args!("{}/analysis_{}",
                                    data_dir, Utc::today().format("%Y%m%d").to_string()));

        if Path::new(&summary_dir).exists(){
            let mut i : usize = 2;
            let mut new : String;
            loop{
                new = format!("{}", format_args!("{}_{}", summary_dir, i));
                if Path::new(&new).exists(){
                    i += 1;
                }
                else{
                    break;
                }
            }
            summary_dir = new.clone();
        }

        let summary_file : String = format!("{}/brief_result.dat", summary_dir);

        fs::create_dir_all(&summary_dir).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/linear_distribution", &summary_dir)).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/logarithmic_distribution", &summary_dir)).map_err(Error::make_error_io)?;

        let summary = File::create(summary_file).map_err(Error::make_error_io)?;
        let mut summary = BufWriter::new(summary);

        summary.write_fmt(format_args!("{}{}\n", H::export_form(width), Self::export_form(width)))
               .map_err(Error::make_error_io)?;

        for entry in fs::read_dir(&data_dir).map_err(Error::make_error_io)?{
            let entry = entry.map_err(Error::make_error_io)?;
            let path = entry.path();
            if path.is_dir(){
                continue;
            }
            println!("File read start : {:?}", path.clone());

            let (dataset, mut lines) : (H, Lines<BufReader<File>>) = match H::from_file(path.clone()){
                Ok(ds) => ds,
                Err(_err) => {continue;},
            };

            let analysis = match hashmap.get_mut(&dataset){
                Some(x) => x,
                None => {
                    let mut x = Self::new(num_var);
                    x.update_from_bin_size(min_time, max_time, bin_size, lbin_size)?;
                    x.allocate_vectors();
                    hashmap.insert(dataset, x);
                    hashmap.get_mut(&dataset).unwrap()
                },
            };
            lines.next();

            for line in lines{
                let line = line.map_err(Error::make_error_io)?;
                let values = line.trim().split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
                analysis.add_ensemble(values);
            }

            println!("File read end : {:?}", path.clone());
        }

        for (dataset, analysis) in hashmap.iter_mut(){
            if analysis.ensemble == 0{
                continue;
            }
            analysis.draw();

            let hist_filename = dataset.export_file_removed_idx(prefix);

            summary.write_fmt(format_args!("{}", dataset.export_data(width)?)).map_err(Error::make_error_io)?;
            analysis.export(width, &mut summary, &summary_dir, &hist_filename)?;
        }
        return Ok(());
    }
}

// =====================================================================================
// ===  Implement Data Pair ============================================================
// =====================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Pair<T>(T, T);

impl<T> FromStr for Pair<T>
    where T : FromStr + Copy{
    type Err = crate::error::Error;

    fn from_str(s : &str) -> Result<Self, Self::Err>{
        let mut trim = s.to_string();
        trim.retain(|c| c != '(' && c != ')');
        let splitted : Vec<T> = trim.split(',').map(|t| t.parse::<T>().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput)).unwrap())
                           .collect();
        if splitted.len() != 2{
            return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments));
        }
        return Ok(Pair(splitted[0], splitted[1]));
    }
}

// =====================================================================================
// ===  Implement ProcessAnalysis =================================================
// =====================================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct ProcessAnalysis{         // analysis for process measurement
    long_term_mean : f64,           // Mean of variable within whole process
    long_term_stddev : f64,         // Stddev of variable within whole process
    tot_point: usize,               // total number of points
    ensemble : usize,               // Number of ensemble
    min_time : f64,                 // Minimal time for logarithmic histogram
    max_time : f64,                 // Maximal time for storing
    bin_size : f64,                 // Size of bin for linear histogram
    lbin_size : f64,                // Size of bin for logarithmic histogram
    num_bin : usize,                // Number of bin for linear histogram
    num_lbin : usize,               // Number of bin for logarithmic histogram
    mean : Vec<f64>,                // mean value at time t < x(t) > with linear binning
    stddev : Vec<f64>,              // stddev of x(t) in linear binning
    count : Vec<usize>,             // Count of data in linear bin
    log_mean : Vec<f64>,            // mean value at time t < x(t) > with logarithmic binning
    log_stddev : Vec<f64>,          // stddev of x(t) in logarithmic binning
    lcount : Vec<usize>,            // Count of data in logarithmic bin
}

impl ProcessAnalysis{
    #[allow(dead_code)]
    fn new() -> Self{
        Self{
            long_term_mean : 0f64,
            long_term_stddev : 0f64,
            tot_point : 0usize,
            ensemble : 0usize,
            min_time : 0f64,
            max_time : 0f64,
            bin_size : 0f64,
            lbin_size : 0f64,
            num_bin : 0usize,
            num_lbin : 0usize,
            mean : Vec::new(),
            stddev : Vec::new(),
            count : Vec::new(),
            log_mean : Vec::new(),
            log_stddev : Vec::new(),
            lcount : Vec::new(),
        }
    }
}

impl Bin for ProcessAnalysis{
    construct_trait_bin!();

    fn allocate_vectors(&mut self){
        self.mean = vec![0.0f64; self.num_bin];
        self.stddev = vec![0.0f64; self.num_bin];
        self.count = vec![0; self.num_bin];

        self.log_mean = vec![0.0f64; self.num_lbin];
        self.log_stddev = vec![0.0f64; self.num_lbin];
        self.lcount = vec![0; self.num_lbin];
    }
}

impl Default for ProcessAnalysis{
    fn default() -> Self{
        Self{
            long_term_mean : 0f64,
            long_term_stddev : 0f64,
            tot_point : 0usize,
            ensemble : 0usize,
            min_time : 1e-15f64,
            max_time : 1e13f64,
            bin_size : 1e10f64,
            lbin_size : 1.2f64,
            num_bin : 1001usize,
            num_lbin : 355,
            mean : vec![0.0f64; 1001],
            stddev : vec![0.0f64; 1001],
            count : vec![0; 1001],
            log_mean : vec![0.0f64; 355],
            log_stddev : vec![0.0f64; 355],
            lcount : vec![0; 355],
        }
    }
}

impl VarTime for ProcessAnalysis{
    fn add_ensemble(&mut self){
        self.ensemble += 1;
    }

     // Add ensemble data
    fn add_pair(&mut self, pair : Pair<f64>){

        self.tot_point += 1;
        let (time, value) = (pair.0, pair.1);

        self.long_term_mean += value;
        self.long_term_stddev += value * value;

        match self.bin_pos(time){
            Some(idx) => {
                self.count[idx] += 1;
                self.mean[idx] += value;
                self.stddev[idx] += value * value;
            },
            None => ()
        }
        match self.lbin_pos(time){
            Some(idx) => {
                self.lcount[idx] += 1;
                self.log_mean[idx] += value;
                self.log_stddev[idx] += value * value;
            },
            None => ()
        }
    }

    // Draw distribution from histogram
    fn draw(&mut self){
        let en = self.tot_point as f64;

        self.long_term_mean = self.long_term_mean / en;
        self.long_term_stddev = ((self.long_term_stddev / en) - self.long_term_mean.powi(2)).sqrt();

        let count = &self.count;
        let mean = &mut self.mean;
        let stddev = &mut self.stddev;
        for (i, &x) in count.iter().enumerate(){
            mean[i] = mean[i] / (x as f64);
            stddev[i] = (stddev[i] / (x as f64) - mean[i].powi(2)).sqrt();
        }

        let lcount = &self.lcount;
        let log_mean = &mut self.log_mean;
        let log_stddev = &mut self.log_stddev;
        for (i, &x) in lcount.iter().enumerate(){
            log_mean[i] = log_mean[i] / (x as f64);
            log_stddev[i] = (log_stddev[i] / (x as f64) - log_mean[i].powi(2)).sqrt();
        }
    }
}

impl Analysis for ProcessAnalysis{
    const NUM_ARGS : usize = 4;

    // Clear data
    fn clear(&mut self){
        self.long_term_mean = 0f64;
        self.long_term_stddev = 0f64;
        self.ensemble = 0;

        self.mean = vec![0f64; self.num_bin];
        self.stddev = vec![0f64; self.num_bin];
        self.count = vec![0; self.num_bin];

        self.log_mean = vec![0f64; self.num_lbin];
        self.log_stddev = vec![0f64; self.num_lbin];
        self.lcount = vec![0; self.num_lbin];
    }

    impl_fn_brief_info!(brief_info, "Random Process analysis", min_time, max_time, bin_size, lbin_size, output_dir);
    impl_fn_info!(info,
                  min_time, "Minimal time for Histogram",
                  max_time, "Maximal time for Histogram",
                  bin_size, "Bin size for Linear Histogram",
                  lbin_size, "Bin size for Logarithmic Histogram",
                  output_dir, "Directory for data files");
    export_form!(export_form, ensemble, long_term_mean, long_term_stddev);

    #[allow(dead_code)]
    fn export_mean_stddev(&self, prec : usize) -> Result<String, Error>{
        let mut string = String::new();
        string.push_str(format!("{}", format_args!("{1:<0$e}\t{2:<0$e}\t {3:<0$}\t",
                        prec, self.long_term_mean, self.long_term_stddev, self.ensemble)).as_str());
        Ok(string)
    }

    #[allow(dead_code)]
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>{
        let bin_size = self.bin_size;
        let mut time = self.min_time + bin_size / 2f64;
        let mut i : usize = 0;

        while time <= self.max_time{

            let n = self.count[i];
            let x = self.mean[i];
            let dx = self.stddev[i];

            if x > 1e-15{
                writer.write_fmt(format_args!("{1:0$e}\t{2:0$}\t{3:0$e}\t{4:0$e}\n", prec, time, n, x, dx)).map_err(Error::make_error_io)?;
            }

            time += bin_size;
            i += 1;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>{
        let lbin_size = self.lbin_size;
        let mut time = self.min_time * lbin_size.sqrt();
        let mut i : usize = 0;

        while time <= self.max_time{
            let n = self.lcount[i];
            let x = self.log_mean[i];
            let dx = self.log_stddev[i];

            if x > 1e-15{
                writer.write_fmt(format_args!("{1:0$e}\t{2:0$}\t{3:0$e}\t{4:0$e}\n", prec, time, n, x, dx)).map_err(Error::make_error_io)?;
            }

            time *= lbin_size;
            i += 1;
        }
        Ok(())
    }

    // export data to summary file
    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>{

        brief_data.write(format!("{}", format_args!("{}\n", self.export_mean_stddev(prec)?)).as_bytes()).map_err(Error::make_error_io)?;

        // Export linear histogram
        let linear_filename = format!("{}", format_args!("{}/linear_distribution/{}", export_dir, filename));
        let linear = File::create(linear_filename).map_err(Error::make_error_io)?;
        let mut linear = BufWriter::new(linear);
        self.export_distribution(prec, linear.get_mut())?;

        // Export logarithmic histogram
        let log_filename = format!("{}", format_args!("{}/logarithmic_distribution/{}", export_dir, filename));
        let log = File::create(log_filename).map_err(Error::make_error_io)?;
        let mut log = BufWriter::new(log);
        self.export_log_scaled_distribution(prec, log.get_mut())?;

        Ok(())
    }

    // analysis
    fn analyze<H : Hash + Eq + Copy + DataSet>(args : &[String], width : usize, prefix : &str) -> Result<(), Error>{
        use chrono::offset::Utc;

        let min_time : f64;
        let max_time : f64;
        let num_bin : usize;
        let bin_size : f64;
        let lbin_size : f64;
        let data_dir : String;

        match args.len(){
            4 => {
                let mut idx : usize = 0;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                num_bin  = args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();

                let bin_info =  Self::convert_num_bin_to_bin_size(min_time, max_time, num_bin)?;
                bin_size = bin_info.0;
                lbin_size = bin_info.1;
            },
            5 => {
                let mut idx : usize = 0;
                min_time = args[idx].parse().unwrap();      idx+=1;
                max_time = args[idx].parse().unwrap();      idx+=1;
                bin_size = args[idx].parse().unwrap();      idx+=1;
                lbin_size= args[idx].parse().unwrap();      idx+=1;
                data_dir = args[idx].clone();
            },
            _ => {
                return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments))
            }
        }

        let mut hashmap : HashMap<H, Self> = HashMap::new();
        let mut summary_dir : String = format!("{}", format_args!("{}/analysis_{}",
                                    data_dir, Utc::today().format("%Y%m%d").to_string()));

        if Path::new(&summary_dir).exists(){
            let mut i : usize = 2;
            let mut new : String;
            loop{
                new = format!("{}", format_args!("{}_{}", summary_dir, i));
                if Path::new(&new).exists(){
                    i += 1;
                }
                else{
                    break;
                }
            }
            summary_dir = new.clone();
        }

        let summary_file : String = format!("{}/analysis_merge_time.dat", summary_dir);

        fs::create_dir_all(&summary_dir).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/linear_distribution", &summary_dir)).map_err(Error::make_error_io)?;
        fs::create_dir_all(format!("{}/logarithmic_distribution", &summary_dir)).map_err(Error::make_error_io)?;

        let summary = File::create(summary_file).map_err(Error::make_error_io)?;
        let mut summary = BufWriter::new(summary);

        summary.write_fmt(format_args!("{}{}\n", H::export_form(width), Self::export_form(width)))
               .map_err(Error::make_error_io)?;

        for entry in fs::read_dir(&data_dir).map_err(Error::make_error_io)?{
            let entry = entry.map_err(Error::make_error_io)?;
            let path = entry.path();
            if path.is_dir(){
                continue;
            }
            println!("File read start : {:?}", path.clone());

            let (dataset, mut lines) : (H, Lines<BufReader<File>>) = match H::from_file(path.clone()){
                Ok(ds) => ds,
                Err(_err) => {continue;},
            };

            let analysis = match hashmap.get_mut(&dataset){
                Some(x) => x,
                None => {
                    let mut x = Self::new();
                    x.update_from_bin_size(min_time, max_time, bin_size, lbin_size)?;
                    x.allocate_vectors();
                    hashmap.insert(dataset, x);
                    hashmap.get_mut(&dataset).unwrap()
                },
            };
            lines.next();

            for line in lines{
                let line = line.map_err(Error::make_error_io)?;
                let splitted : Vec<&str> = line.trim().split_whitespace().collect();
                for s in splitted{
                    let pair : Pair<f64> = s.parse()?;
                    analysis.add_pair(pair);
                }
                analysis.add_ensemble();
            }

            println!("File read end : {:?}", path.clone());
        }

        for (dataset, analysis) in hashmap.iter_mut(){
            if analysis.ensemble == 0{
                continue;
            }
            analysis.draw();

            let hist_filename = dataset.export_file_removed_idx(prefix);

            summary.write_fmt(format_args!("{}", dataset.export_data(width)?)).map_err(Error::make_error_io)?;
            analysis.export(width, &mut summary, &summary_dir, &hist_filename)?;
        }
        return Ok(());
    }
}







#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;
    use crate::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
    use crate::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};


    #[test]
    fn test_constuct_dataset(){

        define_structure!(DataSet;sys_size, f64, dim, usize, target_size, f64, searcher_type, SearcherType,);

        let x = DataSet{
            sys_size : 0f64,
            dim : 2usize,
            target_size : 1f64,
            searcher_type : SearcherType::ContinuousPassiveIndependent,
        };

        assert_eq!(x.sys_size, 0f64);
        assert_eq!(x.dim, 2usize);
        assert_eq!(x.target_size, 1f64);
        assert_eq!(x.searcher_type, SearcherType::ContinuousPassiveIndependent);
    }

    #[test]
    fn test_construct_dataset2(){
        // Dataset
        construct_dataset!(TestData, ContCircSystem, _sys_arg, ContCircSystemArguments,
                        [];
                        ContBulkTarget, target_arg, ContBulkTargetArguments,
                        [target_size, f64]);
    }

    #[test]
    fn test_new(){

        construct_dataset!(TestData, ContCircSystem, sys_arg, ContCircSystemArguments,
                            [sys_size, f64, dim, usize];
                            ContBulkTarget, target_arg, ContBulkTargetArguments,
                            [target_size, f64]);

        let sys_arg = ContCircSystemArguments::new(10f64, 2usize);
        let target_arg = ContBulkTargetArguments::new(Position::<f64>::new(vec![0f64,0f64]), 1f64);
        let test = TestData::new(&sys_arg, &target_arg);

        let res = TestData{
            sys_size : 10f64,
            dim : 2usize,
            target_size : 1f64,
        };

        assert_eq!(res, test);
    }

    // #[test]
    // fn test_from_file(){

    //     construct_dataset!( ContCircSystem, sys_arg, ContCircSystemArguments,
    //                         [sys_size, f64, dim, usize];
    //                         ContBulkTarget, target_arg, ContBulkTargetArguments,
    //                         [target_size, f64]);

    //     let filename = "tests/images/RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_10_DIM_2_TARGET_SIZE_1_NUMBER_OF_SEARCHER_1_SET_1.dat";
    //     let test = DataSet::from_file(filename);
    //     let res = DataSet{
    //         sys_size : 10f64,
    //         dim : 2usize,
    //         target_size : 1f64,
    //     };

    //     assert_eq!(Ok(res), test);
    // }

    #[test]
    fn test_export_file(){
        construct_dataset!(TestData, ContCircSystem, sys_arg, ContCircSystemArguments,
                            [sys_size, f64, dim, usize];
                            ContBulkTarget, target_arg, ContBulkTargetArguments,
                            [target_size, f64]);

        let sys_arg = ContCircSystemArguments::new(10f64, 2);
        let target_arg = ContBulkTargetArguments::new(Position::<f64>::new(vec![0f64;2]), 1f64);

        let dataset = TestData::new(&sys_arg, &target_arg);
        let filename = dataset.export_file("Test_file");

        assert_eq!(filename, "Test_file_sys_size_10_dim_2_target_size_1.dat");
    }

    #[test]
    fn test_hashmap(){
        construct_dataset!(TestData, ContCircSystem, sys_arg, ContCircSystemArguments,
                            [sys_size, f64, dim, usize];
                            ContBulkTarget, target_arg, ContBulkTargetArguments,
                            [target_size, f64]);

        let mut hashmap : HashMap<TestData, usize> = HashMap::new();

        let sys1 = ContCircSystemArguments::new(10f64, 2);
        let sys2 = ContCircSystemArguments::new(10f64, 3);
        let sys3 = ContCircSystemArguments::new(12f64, 2);

        let pos = Position::<f64>::new(vec![0f64; 2]);
        let target1 = ContBulkTargetArguments::new(pos.clone(), 1f64);
        let target2 = ContBulkTargetArguments::new(pos, 2f64);

        let key1 = TestData::new(&sys1, &target1);
        let key1_rep = TestData::new(&sys1, &target1);
        let key2 = TestData::new(&sys1, &target2);
        let key3 = TestData::new(&sys2, &target1);
        let key4 = TestData::new(&sys2, &target2);
        let key5 = TestData::new(&sys3, &target1);
        let key6 = TestData::new(&sys3, &target2);

        assert_eq!(hashmap.len(), 0);
        hashmap.insert(key1, 1);
        assert_eq!(hashmap.len(), 1);
        hashmap.insert(key2, 2);
        assert_eq!(hashmap.len(), 2);
        hashmap.insert(key3, 3);
        assert_eq!(hashmap.len(), 3);
        hashmap.insert(key4, 4);
        assert_eq!(hashmap.len(), 4);
        hashmap.insert(key5, 5);
        assert_eq!(hashmap.len(), 5);

        assert_eq!(hashmap.get(&key1_rep), Some(&1));
        assert_eq!(hashmap.get(&key6), None);
    }

    #[test]
    fn test_mfpt_analysis(){
        use crate::random_mod::get_gaussian;


        let mut rng = rng_seed(100);
        let n : usize = 100000;
        let thr :f64 = (n as f64).powf(-0.5);

        let mut analysis : TimeAnalysis = Default::default();
        analysis.add_ensemble(3f64);
        analysis.add_ensemble(5f64);
        assert!((analysis.mean - 8f64).abs() < thr);
        assert!((analysis.stddev - 34f64).abs() < thr);
        assert_eq!(analysis.ensemble, 2);

        analysis.clear();
        assert!((analysis.mean - 0f64).abs() < thr);
        assert!((analysis.stddev - 0f64).abs() < thr);
        assert_eq!(analysis.ensemble, 0);

        for _i in 0..n{
            let x = get_gaussian(&mut rng);
            analysis.add_ensemble(x);
        }

        analysis.draw();
        let en      : usize = analysis.ensemble;

        assert!(analysis.mean.abs() < thr);
        assert!((analysis.stddev - 1f64).abs() < thr);
        assert_eq!(en, n);
    }

    #[test]
    fn test_histogram() -> Result<(), Error>{
        use crate::random_mod::{get_gaussian};
        let mut rng = rng_seed(100);
        let n : usize = 100000;

        let mut analysis = TimeAnalysis::new();
        analysis.update_from_bin_size(0f64, 10f64, 0.05f64, 1.05f64)?;
        analysis.allocate_vectors();
        for _i in 0..n{
            let x = get_gaussian(&mut rng) + 5f64;
            analysis.add_ensemble(x);
        }
        analysis.draw();
        std::fs::create_dir_all("tests/images").map_err(Error::make_error_io)?;

        let hist_file = File::create("tests/images/analysis_histogram_test.dat").map_err(Error::make_error_io)?;
        let mut hist_buff = BufWriter::new(hist_file);
        analysis.export_distribution(20, hist_buff.get_mut())?;
        hist_buff.flush().map_err(Error::make_error_io)?;

        let log_hist_file = File::create("tests/images/analysis_log_histogram_test.dat").map_err(Error::make_error_io)?;
        let mut log_hist_buff = BufWriter::new(log_hist_file);
        analysis.export_log_scaled_distribution(20, log_hist_buff.get_mut())?;
        log_hist_buff.flush().map_err(Error::make_error_io)?;

        Ok(())
    }

    #[test]
    fn test_convert_num_bin_to_bin_size() -> Result<(), Error>{
        let min : f64 = 1f64;
        let max : f64 = 101f64;
        let num_bin : usize = 100;

        let bs : (f64, f64) = TimeAnalysis::convert_num_bin_to_bin_size(min, max, num_bin)?;
        assert_eq!(bs, (1f64, 1.0472327459898225));

        Ok(())
    }

    #[test]
    fn test_varn() -> Result<(), Error>{
        let mut an = TimeVecAnalysis::new(0);
        assert_eq!(an, TimeVecAnalysis{
            means : Vec::new(),
            stddevs : Vec::new(),
            ensemble : 0usize,
            min_time : 0f64,
            max_time : 0f64,
            bin_size : 0f64,
            lbin_size : 0f64,
            num_bin : 0usize,
            num_lbin : 0usize,
            num_var : 0usize,
            hist : Vec::new(),
            lhist : Vec::new(),
            count : Vec::new(),
            lcount : Vec::new(),
        });

        an.update_from_num_bin(1f64, 20f64, 19)?;

        let thr : f64 = 1e-5;
        assert!((an.min_time - 1f64).abs() < thr);
        assert!((an.max_time- 20f64).abs() < thr);
        assert!((an.bin_size - 1f64).abs() < thr);
        assert!((an.lbin_size - 1.170779913f64).abs() <thr);

        an.num_var = 3;
        an.allocate_vectors();
        assert_eq!(an.means.len(), 3);
        assert_eq!(an.stddevs.len(), 3);
        assert_eq!(an.hist.len(), 3);
        assert_eq!(an.hist[0].len(), 19);

        let input = vec![1.5f64, 2.5f64, 3.5f64];
        an.add_ensemble(input);
        assert_eq!(an.ensemble, 1);
        assert_eq!(an.means[0], 1.5f64);
        assert_eq!(an.means[1], 2.5f64);
        assert_eq!(an.means[2], 3.5f64);
        assert_eq!(an.stddevs[0], 2.25f64);
        assert_eq!(an.stddevs[1], 6.25f64);
        assert_eq!(an.stddevs[2], 12.25f64);
        assert_eq!(an.count[0][0], 1);
        assert_eq!(an.count[1][1], 1);
        assert_eq!(an.count[2][2], 1);
        assert_eq!(an.lcount[0][2], 1);
        assert_eq!(an.lcount[1][5], 1);
        assert_eq!(an.lcount[2][7], 1);

        an.draw();
        let string = an.export_mean_stddev(5)?;
        assert_eq!(string, "1    \t1.5e0\t0e0  \t2.5e0\t0e0  \t3.5e0\t0e0  \t".to_string());
        Ok(())
    }

    #[test]
    fn test_varn_histogram() -> Result<(), Error>{
        use crate::random_mod::{get_gaussian_vec};
        use crate::position::Position;
        let mut rng = rng_seed(100);
        let n : usize = 100000;
        let num_var : usize = 3;

        let mut analysis = TimeVecAnalysis::new(0);
        analysis.update_from_bin_size(0f64, 10f64, 0.05f64, 1.05f64)?;
        analysis.num_var = num_var;
        analysis.allocate_vectors();

        let bias = Position::new(vec![5f64; num_var]);
        for _i in 0..n{
            let x = &get_gaussian_vec(&mut rng, num_var) + &bias;
            analysis.add_ensemble(x.coordinate);
        }
        analysis.draw();
        std::fs::create_dir_all("tests/images").map_err(Error::make_error_io)?;

        let hist_file = File::create("tests/images/analysis_varn_histogram_test.dat").map_err(Error::make_error_io)?;
        let mut hist_buff = BufWriter::new(hist_file);
        analysis.export_distribution(20, hist_buff.get_mut())?;
        hist_buff.flush().map_err(Error::make_error_io)?;

        let log_hist_file = File::create("tests/images/analysis_varn_log_histogram_test.dat").map_err(Error::make_error_io)?;
        let mut log_hist_buff = BufWriter::new(log_hist_file);
        analysis.export_log_scaled_distribution(20, log_hist_buff.get_mut())?;
        log_hist_buff.flush().map_err(Error::make_error_io)?;

        Ok(())
    }

    #[test]
    fn test_pair() -> Result<(), Error>{
        let s : Pair<f64> = "(12.1231,12312.21)".parse().unwrap();
        assert_eq!(s, Pair(12.1231, 12312.21));

        Ok(())
    }
}
