// Macros and Trait implementation for various argument handling
// Simulation을 여러개 만들고자 할 때
// 매번 어떤 변수를 받아야 하는지,
// 그리고 그 변수들을 어떻게 parsing할지,
// 시스템 정보를 어떻게 통일되게 출력할지,
// 그 출력된 시스템 정보를 다시 어떻게 통일되게 읽을 수 있을지
// 고민이 될 수 있다.
// 여기서는 그 통일성을 위해 하나의 macro로 해당 정보들을 정의할 수 있도록 하는 module이다.

#[allow(unused_imports)]
use crate::prelude::*;


// Argument Trait
pub trait Argument<T>{

    // 해당 structure를 정의하기 위해서
    // 어떤 값들이 주어져야 하는지, 그 값들의 type이 무엇인지, 값들의 의미는 무엇인지
    // 를 출력해주는 함수
    // width는 변수명이 담기는 영역의 길이를 말한다. align을 위한 장치
    fn info(width : usize) -> String;

    // info 보다 더 짧게, 변수명만 적은 info
    fn brief_info() -> String;

    // args 벡터로부터 structure_arguments를 읽어서 반환해주는 함수
    fn read_args_from_vec(args: Vec<String>) -> Result<T, Error>;

    // structure의 정보가 담긴 string을 반환해주는 함수
    // simulation result file을 만들 때 맨 위에 적히게 될 것이다.
    // width는 변수명이 담기는 영역의 길이를 말한다. align을 위한 장치
    fn print_configuration(&self, width : usize) -> String;

    // simulation result file을 읽을 때
    // print_configuration의 결과를 다시 variable들로 바꿔줄 필요가 있다.
    // file -> BufReader -> Lines<BufRead> 로 바꾼 후에 함수를 호출하면
    // 해당 변수에 필요한 정보들을 parsing해 structure_arguments를 반환해준다.
    fn read_args_from_lines(reader : &mut Lines<BufReader<File>>) -> Result<T, Error>;
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! define_num_args{
    ($var:expr) => {
        pub const NUM_ARGS : usize = $var;
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fn_info {
    ($($var:ident, $description:expr),*) => {
        #[allow(unused_variables)]
        fn info(width : usize) -> String{
            #[allow(unused_mut)]
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:1$}: {2:}\n", stringify!($var), width, $description)).as_str());
                )*
            string
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fn_brief_info {
    ($description:expr, $($var:ident),*) => {
        #[allow(unused_variables)]
        fn brief_info() -> String{
            #[allow(unused_mut)]
            let mut string = String::from(format!("{0:} arguments : ", $description));
            $(string.push_str(format!("({}) ", stringify!($var)).as_str());
                )*
            string
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fn_read_args_from_vec{
    ($name:ident $(, $type_name:ident, $type_default:expr)* ; $($var:ident), *) => {
        fn read_args_from_vec(args: Vec<String>) -> Result<$name, Error>{
            if args.len() != NUM_ARGS{
                return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments));
            }
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            let mut iter = args.iter();
            Ok($name{
                $($type_name : $type_default,
                    )*
                $($var : {
                        let word : &str = iter.next().unwrap().trim();
                        word.parse().expect("Failed to parse\n")
                    },
                    )*
            })
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fn_print_configuration{
    (,$($var:ident), *) => {
        fn print_configuration(&self, width : usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:1$}: {2:}\n", stringify!($var), width, self.$var)).as_str());
                )*
            string
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fn_read_args_from_lines{
    ($name:ident, $($var:ident), *) => {
        fn read_args_from_lines(reader : &mut Lines<BufReader<File>>) -> Result<$name, Error>{
            Ok($name{
                $($var : { let list : String = reader.next().unwrap().unwrap();
                            let split : Vec<String> = list.split(":")
                                                          .map(|s| s.to_string())
                                                          .collect();
                            if split.len() != 2{
                                return Err(Error::make_error_syntax(ErrorCode::InvalidFormat));
                            }
                            let val_string : &str = &split[1].trim();
                            val_string.parse().expect("Failed to parse\n")},
                    )*
            })
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_argument_trait{
    ($struct_name:ident, $struct_description:expr, $arg_name:ident, $num_args:expr
        $(,$type_name:ident, $type_type:ty, $type_default:expr)*;
        $($var:ident, $t:ty, $description:expr), *) => {
        define_num_args!($num_args);
        define_structure!($arg_name $(, $type_name, $type_type)* ; $($var, $t), *);
        impl_structure!($arg_name $(, $type_name, $type_default)* ; $($var, $t), *);

        impl Argument<$arg_name> for $struct_name{
            impl_fn_info!($($var, $description), *);

            impl_fn_brief_info!($struct_description $(, $var)*);

            impl_fn_read_args_from_vec!($arg_name $(,$type_name, $type_default)*; $($var), *);

            impl_fn_print_configuration!($(,$type_name)* $(,$var)*);

            impl_fn_read_args_from_lines!($arg_name $(, $type_name)* $(,$var)*);
        }

        impl Display for $struct_name{
            fn fmt(&self, f : &mut Formatter) -> fmt::Result{
                write!(f, "{}", self.print_configuration(10))
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    // #[test]
    // fn test_structure(){
    //     // macro를 정의하기 전에 macro로 짤 구조가 잘 작동하는지부터 확인하고 macro를 짜도록 하자.

    //     #[allow(dead_code)]
    //     struct TestStruct{
    //         var1 : f64,
    //         var2 : usize,
    //         var3 : String,
    //     }

    //     #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
    //     struct TestArguments{
    //         var1 : f64,
    //         var2 : usize,
    //     }

    //     pub const NUM_ARGS : usize = 2;

    //     impl Argument<TestArguments> for TestStruct{

    //         fn info(width : usize) -> String{
    //             let mut string = String::new();
    //             string.push_str(format!("{}", format_args!("{0:1$}: {2:}\n", stringify!(var1), width, "Test variable 1")).as_str());
    //             string.push_str(format!("{}", format_args!("{0:1$}: {2:}\n", stringify!(var2), width, "Test variable 2")).as_str());
    //             string
    //         }

    //         fn brief_info() -> String{
    //             "Test arguments : (var1) (var2) ".to_string()
    //         }

    //         fn read_args_from_vec(args: Vec<String>) -> Result<TestArguments, Error>{
    //             if args.len() != NUM_ARGS{
    //                 return Err(Error::make_error_syntax(ErrorCode::InvalidNumberOfArguments));
    //             }
    //             let mut iter = args.iter();
    //             Ok(TestArguments{
    //                 var1 : iter.next().unwrap().parse().expect("Failed to parse\n"),
    //                 var2 : iter.next().unwrap().parse().expect("Failed to parse\n"),
    //             })
    //         }

    //         fn print_configuration(&self, width : usize) -> String{
    //             let mut string = String::new();
    //             string.push_str(format!("{}", format_args!("{0:1$}: {2:.3$e}\n", stringify!(var1), width, self.var1, 5)).as_str());
    //             string.push_str(format!("{}", format_args!("{0:1$}: {2:3$}\n", stringify!(var2), width, self.var2, 0)).as_str());
    //             string
    //         }

    //         fn read_args_from_lines(reader : &Vec<String>) -> Result<TestArguments, Error>{
    //             let mut iter = reader.iter();
    //             Ok(TestArguments{
    //                 var1 : {let list : &String = iter.next().unwrap();
    //                         let split : Vec<String> = list.split(":")
    //                                                       .map(|s| s.to_string())
    //                                                       .collect();
    //                         if split.len() != 2{
    //                             return Err(Error::make_error_syntax(ErrorCode::InvalidFormat));
    //                         }
    //                         let val_string : &str = &split[1].trim();
    //                         val_string.parse().expect("Failed to parse\n")},
    //                 var2 : {let list : &String = iter.next().unwrap();
    //                         let split : Vec<String> = list.split(":")
    //                                                       .map(|s| s.to_string())
    //                                                       .collect();
    //                         if split.len() != 2{
    //                             return Err(Error::make_error_syntax(ErrorCode::InvalidFormat));
    //                         }
    //                         let val_string : &str = &split[1].trim();
    //                         val_string.parse().expect("Failed to parse\n")},
    //             })
    //         }
    //     }

    //     let test1 = TestStruct{
    //         var1 : 3.0f64,
    //         var2 : 15usize,
    //         var3 : String::from("Test string"),
    //     };
    //     let test_arg = TestArguments{
    //         var1 : 3.0f64,
    //         var2 : 15usize,
    //     };
    //     let test_args = vec!["3.0".to_string(), "15".to_string()];
    //     let test_iter : Vec<String> = String::from("var1      : 3.00000e0\nvar2      : 15\n")
    //                                         .split("\n")
    //                                         .map(|s| s.to_string())
    //                                         .collect();

    //     assert_eq!(TestStruct::info(10), String::from("var1      : Test variable 1\nvar2      : Test variable 2\n"));
    //     assert_eq!(TestStruct::read_args_from_vec(test_args), Ok(test_arg));
    //     assert_eq!(test1.print_configuration(10), String::from("var1      : 3.00000e0\nvar2      : 15\n"));
    //     assert_eq!(TestStruct::read_args_from_lines(&test_iter), Ok(test_arg));
    // }

    #[test]
    fn test_define_num_args(){
        define_num_args!(2);
        assert_eq!(NUM_ARGS, 2usize);
    }

    #[test]
    fn test_define_arguments(){
        define_structure!(TestArguments ;var1, f64, var2, usize);
        let _test = TestArguments{
            var1 : 3.0f64,
            var2 : 15usize,
        };
    }

    #[test]
    fn test_impl_fn_info(){
        impl_fn_info!(var1, "Test variable 1", var2, "Test variable 2");
        assert_eq!(info(10), String::from("var1      : Test variable 1\nvar2      : Test variable 2\n"));
    }

    #[test]
    fn test_impl_fn_breif_info(){
        impl_fn_brief_info!("Test", var1, var2);
        assert_eq!(brief_info(), String::from("Test arguments : (var1) (var2) "));
    }

    #[test]
    fn test_impl_read_args_from_vec(){
        define_num_args!(2);
        define_structure!(TestArguments ; var1, f64, var2, usize);
        impl_fn_read_args_from_vec!(TestArguments ;var1 ,var2);

        let test_arg = TestArguments{
            var1 : 3.0f64,
            var2 : 15usize,
        };
        let test_args = vec!["3.0".to_string(), "15".to_string()];
        assert_eq!(read_args_from_vec(test_args), Ok(test_arg));
    }

    #[test]
    fn test_impl_print_configuration(){
        #[allow(dead_code)]
        struct TestStruct{
            var1 : f64,
            var2 : usize,
            var3 : String,
        }

        impl TestStruct{
            impl_fn_print_configuration!(,var1, var2);
        }

        let test1 = TestStruct{
            var1 : 3.0f64,
            var2 : 15usize,
            var3 : String::from("Test string"),
        };
        assert_eq!(test1.print_configuration(10), String::from("var1      : 3\nvar2      : 15\n"));
    }


    #[test]
    fn test_impl_argument_trait(){
        use crate::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};

        let f = File::open("tests/images/RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_10_DIM_2_TARGET_SIZE_1_NUMBER_OF_SEARCHER_1_SET_1.dat").unwrap();
        let f = BufReader::new(f);
        let mut lines = f.lines();
        lines.next();


        let test1 = ContCircSystem::new(10f64, 2usize);
        let test2 = ContCircSystemArguments{
            sys_type : SystemType::ContinuousCircular,
            bctype : BoundaryCond::Reflection,
            sys_size : 10f64,
            dim : 2usize,
        };
        let test_args = vec!["10.0".to_string(), "2".to_string()];

        assert_eq!(ContCircSystem::info(10), String::from("sys_size  : Size of System\ndim       : Dimension of System\n"));
        assert_eq!(ContCircSystem::read_args_from_vec(test_args), Ok(test2.clone()));
        assert_eq!(test1.print_configuration(10), String::from("sys_type  : Continuous Circular system.\nbctype    : Reflective Boundary Condtion\nsys_size  : 10\ndim       : 2\n"));
        assert_eq!(ContCircSystem::read_args_from_lines(&mut lines), Ok(test2));
    }


    #[test]
    fn test_impl_read_args_from_lines2() -> Result<(), io::Error>{
        use crate::system_mod::cont_circ::ContCircSystem;

        let f = File::open("tests/images/RTS_N_PTL_INDEP_SEARCHER_SYS_SIZE_10_DIM_2_TARGET_SIZE_1_NUMBER_OF_SEARCHER_1_SET_1.dat")?;
        let f = BufReader::new(f);
        let mut lines = f.lines();
        lines.next();


        impl_fn_read_args_from_lines!(ContCircSystem,
            sys_type, bctype, sys_size, dim);

        let sys = read_args_from_lines(&mut lines).unwrap();
        let res = ContCircSystem::new(10f64, 2usize);

        assert_eq!(sys, res);
        Ok(())
    }
}




