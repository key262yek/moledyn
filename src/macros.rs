// Modules for macros except for arguments

use crate::prelude::*;

#[macro_export]
// #[allow(unused_macros)]
macro_rules! impl_fmt_for_type{
    ($type_name:ident, $($variant:pat => $description:expr), *) =>{
        impl Display for $type_name{
            fn fmt(&self, f: &mut Formatter) -> fmt::Result{
                match *self{
                    $($variant => write!(f, $description),
                        )*
                }
            }
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! impl_fmt_test{
    ($test_name:ident, $($variant:expr => $description:expr), *) => {
        #[test]
        fn $test_name(){
            $(assert_eq!(format!("{}", $variant).as_str(), $description);
                )*
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! impl_fromstr_for_type{
    ($type_name:ident, $($variant:expr => $description:expr), *) => {
        impl FromStr for $type_name{
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s{
                    $($description => Ok($variant),
                        )*
                    _ => Err(Error::make_error_syntax(crate::error::ErrorCode::InvalidArgumentInput)),
                }
            }
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! impl_fromstr_test{
    ($test_name:ident, $type_name:ident, $($variant:expr => $description:expr), *) => {
        #[test]
        fn $test_name(){
            $(assert_eq!($type_name::from_str($description), Ok($variant));
              assert_eq!($description.parse::<$type_name>(), Ok($variant));
                )*
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! define_structure{
    ($name:ident $(, $type_name:ident, $type_type:ty)* ;$($var:ident, $t:ty,)*) =>{
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct $name{
            $(pub $type_name : $type_type,
                )*
            $(pub $var : $t,
                )*
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! define_structure_wo_eq{
    ($name:ident $(, $type_name:ident, $type_type:ty)* ;$($var:ident, $t:ty,)*) =>{
        #[derive(Clone, Debug, PartialOrd)]
        pub struct $name{
            $(pub $type_name : $type_type,
                )*
            $(pub $var : $t,
                )*
        }
    }
}


#[macro_export]
// #[allow(unused_macros)]
macro_rules! impl_structure{
    ($name:ident, $num_args:expr $(, $type_name:ident, $type_default:expr)* ;$($var:ident, $t:ty,) *) =>{
        #[allow(dead_code)]
        impl $name{
            define_pub_num_args!($num_args);

            pub fn new($($var : $t),*) -> Self{
                Self{
                    $($type_name : $type_default,
                        )*
                    $($var : $var,
                        )*
                }
            }

            pub fn clear(&mut self){
                $(
                    self.$type_name = $type_default;
                    )*
                $(
                    self.$var = Default::default();
                    )*
            }
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! construct_structure {
    ($name:ident, $num_args:expr $(, $type_name:ident, $type_type:ty, $type_default:expr)* ;$($var:ident, $t:ty), *) => {
        define_structure!($name $(, $type_name, $type_type)* ;$($var, $t,) *);

        impl_structure!($name, $num_args $(, $type_name, $type_default)* ;$($var, $t,) *);
    };
}



#[macro_export]
// #[allow(unused_macros)]
macro_rules! derive_hash{
    ($name:ident $(, $var:ident) *) => {
        impl Hash for $name{
            fn hash<H: Hasher>(&self, state: &mut H){
                $(
                    let mut c = self.$var as f64 * 1e10;
                    let mut b : f64 = 0.0;

                    if c < 0.0 {
                        c = - c;
                        b = 1.0;
                    }
                    ((c + b) as usize).hash(state);
                    )*
            }
        }
    }
}



#[macro_export]
// #[allow(unused_macros)]
macro_rules! export_form{
    ($name:ident $(,$var: ident) *) => {
        #[allow(dead_code)]
        fn $name(width: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}", stringify!($var), width)).as_str());
                )*
            return string;
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! pub_export_form{
    ($name:ident $(,$var:ident) *) => {
        #[allow(dead_code)]
        pub fn $name(width: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}", stringify!($var), width)).as_str());
                )*
            return string;
        }
    }
}

pub trait TypeName{
    fn type_of(&self) -> &'static str;
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! type_info{
    ($($type: ty), *) =>{
        $(impl TypeName for $type{
            fn type_of(&self) -> &'static str{
                type_name::<$type>()
            }
        }
        )*
    }
}

type_info!(usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, String, &str);


#[macro_export]
// #[allow(unused_macros)]
macro_rules! export_data{
    ($name:ident $(,$var:ident)*) => {

        #[allow(dead_code)]
        fn $name(&self, prec: usize) -> Result<String, Error>{
            let mut string = String::new();

            $(match self.$var.type_of(){
                "usize" | "u8" | "u16" | "u32" | "u64" | "u128" |
                "i8" | "i16" | "i32" | "i64" | "i128"
                => {
                   string.push_str(format!("{}", format_args!("{0:<1$}\t", self.$var, prec)).as_str());
                },
                "f32" | "f64"
                => {
                    string.push_str(format!("{}", format_args!("{0:<1$e}\t", self.$var, prec)).as_str());
                },
                _ => {
                    return Err(Error::make_error_syntax(ErrorCode::InvalidType));
                }
            }
            )*
            return Ok(string);
        }
    }
}

#[macro_export]
// #[allow(unused_macros)]
macro_rules! pub_export_data{
    ($name:ident $(,$var:ident)*) => {

        #[allow(dead_code)]
        pub fn $name(&self, prec: usize) -> Result<String, Error>{
            let mut string = String::new();

            $(match self.$var.type_of(){
                "usize" | "u8" | "u16" | "u32" | "u64" | "u128" |
                "i8" | "i16" | "i32" | "i64" | "i128"
                => {
                   string.push_str(format!("{}", format_args!("{0:<1$}\t", self.$var, prec)).as_str());
                },
                "f32" | "f64"
                => {
                    string.push_str(format!("{}", format_args!("{0:.1$e}\t", self.$var, prec)).as_str());
                },
                _ => {
                    return Err(Error::make_error_syntax(ErrorCode::InvalidType));
                }
            }
            )*
            return Ok(string);
        }
    }
}


#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_export_data() -> Result<(), Error>{
        #[allow(dead_code)]
        struct Test{
            num1 : usize,
            num2 : usize,
            float1 : f64,
        }
        impl Test{
            export_data!(test_export, num1, num2);
        }

        let test = Test{
            num1 : 10,
            num2 : 5,
            float1: 0f64,
        };

        assert_eq!(test.test_export(5)?, "10   \t5    \t".to_string());
        Ok(())
    }

    #[test]
    fn test_hash() -> Result<(), Error>{
        use crate::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
        use crate::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};
        use crate::searcher_mod::{cont_passive_merge::{ContPassiveMergeSearcher, ContPassiveMergeSearcherArguments}};
        use crate::time_mod::{ExponentialStep, ExponentialStepArguments};

        // Dataset
        construct_dataset!(SimulationData, ContCircSystem, sys_arg, ContCircSystemArguments,
                        [sys_size, f64, dim, usize ];
                        ContBulkTarget, target_arg, ContBulkTargetArguments,
                        [target_size, f64];
                        ContPassiveMergeSearcher, searcher_arg, ContPassiveMergeSearcherArguments,
                        [num_searcher, usize, radius, f64, alpha, f64];
                        ExponentialStep, time_arg, ExponentialStepArguments,
                        [dt_min, f64, dt_max, f64, length, usize];
                        {VariableSimulation, sim_arg, VariableSimulationArguments,
                        [idx_set, usize]});

        let dataset1 = SimulationData{
            sys_size : 10f64,
            dim : 2,
            target_size : 1f64,
            num_searcher : 10,
            radius : 0.1f64,
            alpha : 1.0f64,
            dt_min : 1e-10f64,
            dt_max : 1e-5f64,
            length : 10,
            idx_set : 1,
        };

        let dataset2 = SimulationData{
            sys_size : 10f64,
            dim : 2,
            target_size : 1f64,
            num_searcher : 10,
            radius : 0.1f64,
            alpha : 1.0f64,
            dt_min : 1e-10f64,
            dt_max : 1e-5f64,
            length : 10,
            idx_set : 2,
        };

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }

        println!("{} {}", calculate_hash(&dataset1), calculate_hash(&dataset2));

        let mut hashmap : HashMap<SimulationData, usize> = HashMap::new();
        hashmap.insert(dataset1, 0);
        match hashmap.get_mut(&dataset2){
            Some(x) => {*x = *x + 1},
            None => {panic!("Still different");},
        }

        Ok(())
    }
}
