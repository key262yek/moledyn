// Functionality for analysis



#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_dataset {
    ( $( $struct_type:ty, $arg_name:ident, $arg_type:ty, [$($var:ident, $t:ty), *] );*) => {
        define_structure!(DataSet; $($($var, $t),*),*);

        impl DataSet{
            #[allow(dead_code)]
            pub fn new($($arg_name : &$arg_type), *) -> Self{
                DataSet{
                    $(
                        $($var : $arg_name.$var,
                            )*
                    )*
                }
            }

            #[allow(dead_code)]
            pub fn from_file<P>(path : P) -> Result<Self, Error>
                where P : AsRef<Path>{
                let f = File::open(path).unwrap();
                let f = BufReader::new(f);
                let mut lines = f.lines();
                lines.next();

                $(let $arg_name = <$struct_type>::read_args_from_lines(&mut lines)?;
                    )*

                Ok(DataSet::new($(&$arg_name),*))
            }

            #[allow(dead_code)]
            pub fn export_file(&self, prefix : &str) -> String{
                let mut string = String::from(prefix);
                $(
                    $(
                        string.push_str(format!("{}", format_args!("_{}_{}", stringify!($var), self.$var)).as_str());
                    )*
                )*
                string.push_str(".dat");
                return string;
            }
        }

        impl Eq for DataSet{
        }
    }
}

pub trait FptAnalysis{
    // Mean, STDDEV of FPT computation
    fn clear(&mut self);
    fn renew_fpt(&mut self, fpt : f64);
    fn mfpt(&self) -> f64;
    fn stddev(&self) -> f64;
}

pub trait Histogram{
    // Histogram
    fn initiate(&mut self, min_time : f64, max_time : f64, bin : f64, lbin : f64);
    fn renew_data(&mut self, fpt : f64);
    fn draw(&mut self);
}

pub trait NumSearcherAnalysis{
    // Number of Searcher Trajectory computation
    fn renew_num_searcher(&self, dt : f64, num_searcher_at_t : Vec<f64>);
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_mfpt_analysis {
    ($name:ident, $mfpt:ident, $stddev:ident, $ensemble:ident) => {
        impl FptAnalysis for $name{
            fn clear(&mut self){
                self.$mfpt = 0;
                self.$stddev = 0;
            }

            fn renew_fpt_sum(&mut self, fpt:f64){
                self.$mfpt += fpt;
                self.$stddev += fpt * fpt;
                self.$ensemble += 1;
            }

            fn mfpt(&self) -> f64{
                self.$mfpt / (self.$ensemble as f64)
            }

            fn stddev(&self) -> f64{
                self.$stddev / (self.$ensemble as f64) - self.mfpt().powi(2)
            }
        }
    }
}


#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_histogram{
    ($name:ident, $min_time:ident, $max_time:ident, $bin_size:ident, $lbin_size:ident,
        $n_bin:ident, $n_lbin:ident, $hist:ident, $lhist:ident, $ensemble:ident) =>{
        impl Histogram for $name{
            fn initiate(&mut self, min_time : f64, max_time : f64, bin : f64, lbin : f64)
                    -> Result<(), Error>{
                if min_time <0 || max_time < 0 || bin < 0 || lbin < 0{
                    return Error::make_error_syntax(ErrorCode::InvalidArgumentInput);
                }

                self.$min_time = min_time;
                self.$max_time = max_time;
                self.$bin_size = bin;
                self.$lbin_size = lbin.log2();
                self.$n_bin = ((max_time - min_time) / bin + 2).floor() as usize;
                self.$n_lbin = (((max_time / min_time).log2()) / lbin + 2).floor() as usize;

                self.$hist : Vec<f64> = vec![0.0f64; self.$n_bin];
                self.$lhist : Vec<f64> = vecl![0.0f64; self.$n_lbin];
            }

            fn renew_data(&mut self, fpt : f64){
                let pos : usize = 0;
                let lpos : usize = 0;

                if fpt > self.$max_time{
                    return;
                }
                else if fpt > $max_time{
                    pos = ((fpt - self.$min_time) / self.$bin + 1).floor() as usize;
                    lpos = ((fpt / self.$min_time).log2() / self.$lbin + 1).floor() as usize;
                }

                self.$hist[pos] += 1;
                self.$lhist[lpos] += 1;
            }

            fn draw(&mut self){
                let en : f64= self.$ensemble as f64;
                let d_bin : f64 = en * (self.$bin_size as f64);
                let lbin : f64 = self.$lbin_size;
                let mut d_lbin : f64 = self.$min_time / lbin.sqrt() * en * (lbin - 1);

                self.$hist[0] = self.$hist[0] / (self.$min_time * en);
                for i in 1..self.$n_bin{
                    self.$hist[i] = self.$hist[i] / (bin * en);
                }

                self.$lhist[0] = self.$lhist[0] / (self.$min_time * en);
                for i in 1..self.$n_lbin{
                    d_lbin = d_lbin * lbin;
                    self.$lhist[i] = self.$lhist[i] / d_lbin;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;
    use crate::prelude::*;
    use crate::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
    use crate::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};


    #[test]
    fn test_constuct_dataset(){

        define_structure!(DataSet;sys_size, f64, dim, usize, target_size, f64, searcher_type, SearcherType);

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
    fn test_new(){

        construct_dataset!( ContCircSystem, sys_arg, ContCircSystemArguments,
                            [sys_size, f64, dim, usize];
                            ContBulkTarget, target_arg, ContBulkTargetArguments,
                            [target_size, f64]);

        let sys_arg = ContCircSystemArguments::new(10f64, 2usize);
        let target_arg = ContBulkTargetArguments::new(Position::<f64>::new(vec![0f64,0f64]), 1f64);
        let test = DataSet::new(&sys_arg, &target_arg);

        let res = DataSet{
            sys_size : 10f64,
            dim : 2usize,
            target_size : 1f64,
        };

        assert_eq!(res, test);
    }

    #[test]
    fn test_from_file(){

        construct_dataset!( ContCircSystem, sys_arg, ContCircSystemArguments,
                            [sys_size, f64, dim, usize];
                            ContBulkTarget, target_arg, ContBulkTargetArguments,
                            [target_size, f64]);

        let filename = "tests/images/RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_10_DIM_2_TARGET_SIZE_1_NUMBER_OF_SEARCHER_1_SET_1.dat";
        let test = DataSet::from_file(filename);
        let res = DataSet{
            sys_size : 10f64,
            dim : 2usize,
            target_size : 1f64,
        };

        assert_eq!(Ok(res), test);
    }
}
