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

#[macro_export]
#[allow(unused_macros)]
macro_rules! define_structure{
    ($name:ident $(, $type_name:ident, $type_type:ty)* ;$($var:ident, $t:ty), *) =>{
        #[allow(dead_code)]
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
#[allow(unused_macros)]
macro_rules! impl_structure{
    ($name:ident $(, $type_name:ident, $type_default:expr)* ;$($var:ident, $t:ty), *) =>{
        #[allow(dead_code)]
        impl $name{
            pub fn new($($var : $t),*) -> Self{
                Self{
                    $($type_name : $type_default,
                        )*
                    $($var : $var,
                        )*
                }
            }
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_structure {
    ($name:ident $(, $type_name:ident, $type_type:ty, $type_default:expr)* ;$($var:ident, $t:ty), *) => {
        define_structure!($name, $(, $type_name, $type_type:)* ;$($var, $t), *);

        impl_structure!($name $(, $type_name, $type_default)* ;$($var, $t), *);
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! derive_hash{
    ($name:ident $(, $var:ident) *) => {
        impl Hash for $name{
            fn hash<H: Hasher>(&self, state: &mut H){
                $(self.$var.hash(state);
                    )*
            }
        }
    }
}
