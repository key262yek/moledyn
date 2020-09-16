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

    // Explaination of each column in data file
    fn export_form(width: usize) -> String;

    // Data value export
    fn export_data(&self, prec : usize) -> Result<String, Error>;
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
                // $($(
                //     string.push_str(format!("{}", format_args!("_{}_{}", stringify!($sim_var), self.$sim_var)).as_str());
                // )*)?
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
    }
}

// =====================================================================================
// ===  Trait for Analysis =============================================================
// =====================================================================================

pub trait MFPT
    where Self : Sized{
    // Define Analysis from bin sizes
    fn from_bin_size(min_time : f64, max_time : f64, bin_size : f64, lbin_size : f64) -> Result<Self, Error>;

    // Define Analysis from number of bin
    fn from_num_bin(min_time : f64, max_time : f64, num_bin : usize) -> Result<Self, Error>;

    // Convert number of bins to bin sizes
    fn convert_num_bin_to_bin_size(min_time : f64, max_time : f64, num_bin : usize) -> Result<(f64, f64), Error>;

    // Add ensemble data
    fn add_ensemble(&mut self, fpt : f64);

    // return Mean First Passage Time
    fn mfpt(&self) -> f64;

    // return Standard Deviation of First Passage Time
    fn stddev(&self) -> f64;

    // Find binning position of data
    fn bin_pos(&self, fpt : f64) -> Option<usize>;
    fn lbin_pos(&self, fpt : f64) -> Option<usize>;

    // Draw distribution from histogram
    fn draw(&mut self);

    // Export datas
    fn export_mean_stddev(&self, prec: usize) -> Result<String, Error>;
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>;
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>;

    // clear analysis
    fn clear_mfpt_data(&mut self);

    impl_fn_brief_info!(mfpt_breif_info, "MFPT", min_time, max_time, bin_size, lbin_size, output_dir);
    impl_fn_info!(mpft_info,
                  min_time, "Minimal time for Histogram",
                  max_time, "Maximal time for Histogram",
                  bin_size, "Bin size for Linear Histogram",
                  lbin_size, "Bin size for Logarithmic Histogram",
                  output_dir, "Directory for data files");
    export_form!(mfpt_export_form, mfpt, stddev, ensemble);
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

    // export data to summary file
    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>;

    // analysis
    fn analyze<H : Hash + Eq + Copy + DataSet>(args : &[String], width : usize, prefix : &str) -> Result<(), Error>;
}



// =====================================================================================
// ===  Implement Analysis =============================================================
// =====================================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct MFPTAnalysis{
    sum_fpt : f64,          // Sum of fpt data
    square_fpt : f64,       // Sum of fpt squares
    ensemble : usize,       // Number of ensemble
    min_time : f64,         // Minimal time for logarithmic histogram
    max_time : f64,         // Maximal time for storing
    bin_size : f64,         // Size of bin for linear histogram
    lbin_size : f64,        // Size of bin for logarithmic histogram
    num_bin : usize,        // Number of bin for linear histogram
    num_lbin : usize,       // Number of bin for logarithmic histogram
    hist : Vec<f64>,        // Linear Histogram
    lhist : Vec<f64>,       // Logarithmic Histogram
}

impl Analysis for MFPTAnalysis{
    const NUM_ARGS : usize = 4;

    #[allow(dead_code)]
    fn clear(&mut self){
        self.clear_mfpt_data();
    }

    fn info(width : usize) -> String{
        let mut string = String::new();
        string.push_str(format!("{}", MFPTAnalysis::mpft_info(width)).as_str());
        string.push_str("You can give only number of bin instead of bin sizes\n");
        return string;
    }

    fn brief_info() -> String{
        let mut string = String::new();
        string.push_str(format!("{}\n", MFPTAnalysis::mfpt_breif_info()).as_str());
        return string;
    }

    fn export_form(width: usize) -> String{
        let mut string = String::new();
        string.push_str(format!("{}", MFPTAnalysis::mfpt_export_form(width)).as_str());
        return string;
    }

    fn export<W: Write>(&self, prec: usize, brief_data : &mut W, export_dir: &String, filename: &String) -> Result<(), Error>{
        // Export mfpt datas
        brief_data.write(format!("{}\n", self.export_mean_stddev(prec)?).as_bytes()).map_err(Error::make_error_io)?;

        // Export linear histogram
        let linear_filename = format!("{}", format_args!("{}/linear_distribution/{}", export_dir, filename));
        let linear = File::create(linear_filename).map_err(Error::make_error_io)?;
        let mut linear = BufWriter::new(linear);
        self.export_distribution(prec, linear.get_mut())?;

        // Export logarithmic histogram
        let log_filename = format!("{}", format_args!("{}/linear_distribution/{}", export_dir, filename));
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
        let summary_dir : String = format!("{}", format_args!("{}/analysis_{}",
                                    data_dir, Utc::today().format("%Y%m%d").to_string()));
        let summary_file : String = format!("{}/analysis_fpt.dat", summary_dir);

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

            let (dataset, mut lines) : (H, Lines<BufReader<File>>) = match H::from_file(path.clone()){
                Ok(ds) => ds,
                Err(_err) => {continue;},
            };

            let analysis = match hashmap.get_mut(&dataset){
                Some(x) => x,
                None => {
                    let x = Self::from_bin_size(min_time, max_time, bin_size, lbin_size)?;
                    hashmap.insert(dataset, x);
                    hashmap.get_mut(&dataset).unwrap()
                },
            };
            lines.next();

            for line in lines{
                let line = line.map_err(Error::make_error_io)?;
                let fpt : f64 = line.trim().parse().unwrap();
                analysis.add_ensemble(fpt);
            }
        }

        for (dataset, analysis) in hashmap.iter_mut(){
            analysis.draw();

            summary.write_fmt(format_args!("{}{}\n", dataset.export_data(width)?, analysis.export_mean_stddev(width)?))
                    .map_err(Error::make_error_io)?;

            let hist_filename = dataset.export_file(prefix);

            let linear = File::create(format!("{}", format_args!("{}/linear_distribution/{}",
                                        summary_dir, hist_filename))).map_err(Error::make_error_io)?;
            let mut linear = BufWriter::new(linear);
            analysis.export_distribution(width, linear.get_mut())?;
            let log = File::create(format!("{}", format_args!("{}/logarithmic_distribution/{}",
                                        summary_dir, hist_filename))).map_err(Error::make_error_io)?;
            let mut log = BufWriter::new(log);
            analysis.export_log_scaled_distribution(width, log.get_mut())?;
        }
        return Ok(());
    }
}

impl MFPT for MFPTAnalysis{
    #[allow(dead_code)]
    fn from_bin_size(min_time : f64, max_time : f64, bin_size : f64, lbin_size : f64) -> Result<Self, Error>{
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

        Ok(Self{
            sum_fpt : 0f64,
            square_fpt : 0f64,
            ensemble : 0usize,
            min_time : time,
            max_time : max_time,
            bin_size : bin_size,
            lbin_size : lbin_size,
            num_bin : num_bin,
            num_lbin : num_lbin,
            hist : vec![0.0f64; num_bin],
            lhist : vec![0.0f64; num_lbin],
        })
    }

    #[allow(dead_code)]
    fn from_num_bin(min_time : f64, max_time : f64, num_bin : usize) -> Result<Self, Error>{
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

        Ok(Self{
            sum_fpt : 0f64,
            square_fpt : 0f64,
            ensemble : 0usize,
            min_time : time,
            max_time : max_time,
            bin_size : bin_size,
            lbin_size : lbin_size,
            num_bin : num_bin,
            num_lbin : num_bin,
            hist : vec![0.0f64; num_bin],
            lhist : vec![0.0f64; num_bin],
        })
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
    fn add_ensemble(&mut self, fpt : f64){
        self.ensemble += 1;
        self.sum_fpt += fpt;
        self.square_fpt += fpt * fpt;

        match self.bin_pos(fpt){
            Some(idx) => {self.hist[idx] += 1f64;},
            None => ()
        }
        match self.lbin_pos(fpt){
            Some(idx) => {self.lhist[idx] += 1f64;},
            None => ()
        }
    }

    #[allow(dead_code)]
    fn mfpt(&self) -> f64{
        self.sum_fpt / (self.ensemble as f64)
    }

    #[allow(dead_code)]
    fn stddev(&self) -> f64{
        (self.square_fpt / (self.ensemble as f64) - self.mfpt().powi(2)).sqrt()
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

    #[allow(dead_code)]
    fn draw(&mut self){
        let en : f64= self.ensemble as f64;
        let d_bin : f64 = en * (self.bin_size as f64);
        let lbin : f64 = self.lbin_size;
        let mut d_lbin : f64 = self.min_time * en * (lbin - 1f64);

        for x in &mut self.hist{
            *x /= d_bin;
        }

        for x in &mut self.lhist{
            *x /= d_lbin;
            d_lbin *= lbin;
        }
    }

    #[allow(dead_code)]
    fn export_mean_stddev(&self, prec : usize) -> Result<String, Error>{
        Ok(format!("{}", format_args!("{1:<0$e}\t{2:<0$e}\t{3:<0$}", prec, self.mfpt(), self.stddev(), self.ensemble)))
    }

    #[allow(dead_code)]
    fn export_distribution<W: Write>(&self, prec : usize, writer : &mut W) -> Result<(), Error>{
        let bin_size = self.bin_size;
        let mut time = self.min_time - bin_size / 2f64;
        for x in &self.hist{
            time += bin_size;
            if x.abs() < 1e-15{
                continue;
            }
            writer.write_fmt(format_args!("{1:0$e}\t{2:0$e}\n", prec, time, x)).map_err(Error::make_error_io)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn export_log_scaled_distribution<W: Write>(&self, prec : usize, writer: &mut W) -> Result<(), Error>{
        let r = self.lbin_size;
        let mut time = self.min_time / r.sqrt();

        for x in &self.lhist{
            time *= r;
            if x.abs() < 1e-15{
                continue;
            }
            writer.write_fmt(format_args!("{1:0$e}\t{2:0$e}\n", prec, time, x)).map_err(Error::make_error_io)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn clear_mfpt_data(&mut self){
        self.sum_fpt = 0f64;
        self.square_fpt = 0f64;
        self.ensemble = 0usize;
        self.hist = vec![0.0; self.num_bin];
        self.lhist = vec![0.0; self.num_lbin];
    }
}


impl Default for MFPTAnalysis{
    fn default() -> Self{
        MFPTAnalysis{
            sum_fpt : 0f64,
            square_fpt : 0f64,
            ensemble : 0usize,
            min_time : 1e-15f64,
            max_time : 1e13f64,
            bin_size : 1e10f64,
            lbin_size : 1.2f64,
            num_bin : 1001usize,
            num_lbin : 355,
            hist : vec![0.0f64; 1001],
            lhist : vec![0.0f64; 355],
        }
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

        let mut analysis : MFPTAnalysis = Default::default();
        analysis.add_ensemble(3f64);
        analysis.add_ensemble(5f64);
        assert!((analysis.sum_fpt - 8f64).abs() < thr);
        assert!((analysis.square_fpt - 34f64).abs() < thr);
        assert_eq!(analysis.ensemble, 2);

        analysis.clear();
        assert!((analysis.sum_fpt - 0f64).abs() < thr);
        assert!((analysis.square_fpt - 0f64).abs() < thr);
        assert_eq!(analysis.ensemble, 0);

        for _i in 0..n{
            let x = get_gaussian(&mut rng);
            analysis.add_ensemble(x);
        }

        let mfpt    : f64 = analysis.mfpt();
        let stddev  : f64 = analysis.stddev();
        let en      : usize = analysis.ensemble;

        assert!(mfpt.abs() < thr);
        assert!((stddev - 1f64).abs() < thr);
        assert_eq!(en, n);
    }

    #[test]
    fn test_histogram() -> Result<(), Error>{
        use crate::random_mod::{get_gaussian};
        let mut rng = rng_seed(100);
        let n : usize = 100000;

        let mut analysis = MFPTAnalysis::from_bin_size(0f64, 10f64, 0.05f64, 1.05f64)?;
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

        let bs : (f64, f64) = MFPTAnalysis::convert_num_bin_to_bin_size(min, max, num_bin)?;
        assert_eq!(bs, (1f64, 1.0472327459898225));

        Ok(())
    }
}
