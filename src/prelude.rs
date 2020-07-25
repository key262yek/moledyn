//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    {define_num_args, define_arguments, impl_fn_info, impl_fn_brief_info, impl_fn_read_args_from_vec,
        impl_fn_print_configuration, impl_fn_read_args_from_lines, impl_argument_trait,
        impl_fmt_for_type, impl_fmt_test, impl_fromstr_for_type, impl_fromstr_test},
    error::{Error, ErrorCode},
    position::{Position, Numerics},
    argument::{Argument},
    random_mod::{rng_seed},
    system_mod::{SystemCore, SystemType, BoundaryCond},
    target_mod::{TargetCore, TargetType},
    searcher_mod::{SearcherCore, SearcherType, MoveType, InitType}

};

pub use rand_pcg::Pcg64;
pub use std::fmt::{self, Display, Formatter};
pub use std::str::FromStr;
pub use std::io;

