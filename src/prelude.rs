//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    {define_num_args, define_pub_num_args, define_num_args_of_structure,
        define_structure, impl_structure, construct_structure,
        impl_fn_info, impl_fn_brief_info, impl_fn_print_configuration,
        impl_fn_read_args_from_vec, impl_fn_read_args_from_lines, impl_argument_trait,
        impl_fmt_for_type, impl_fmt_test, impl_fromstr_for_type, impl_fromstr_test,
        construct_dataset, derive_hash, export_form, pub_export_form, export_data, pub_export_data,
        export_simulation_item, define_total_num_args, read_arguments, export_simulation_info,
        setup_simulation, setup_simulation_fixed
    },
    error::{Error, ErrorCode},
    position::{Position, Numerics},
    argument::{Argument},
    analysis::{MFPT, Analysis, MFPTAnalysis, DataSet},
    random_mod::{rng_seed},
    system_mod::{SystemCore, SystemType, BoundaryCond},
    target_mod::{TargetCore, TargetType},
    searcher_mod::{SearcherCore, SearcherType, MoveType, InitType},
    time_mod::{TimeType, TimeIterator},
    setup::{Simulation, SimulationArguments},
    iterator::{Node, LinkedList},
    macros::TypeName,
};

pub use rand_pcg::Pcg64;
pub use std::{fmt::{self, Display, Formatter},
            str::FromStr,
            io::{prelude::*, self, Lines, BufReader, Write, BufWriter},
            fs::{self, File},
            convert::AsRef,
            path::Path,
            hash::{Hash, Hasher},
            collections::HashMap,
            default::Default,
            any::type_name,};
pub use streaming_iterator::{StreamingIterator,
        };


