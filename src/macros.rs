// Modules for macros except for arguments

#[macro_export]
#[allow(unused_macros)]
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
#[allow(unused_macros)]
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
#[allow(unused_macros)]
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
#[allow(unused_macros)]
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
