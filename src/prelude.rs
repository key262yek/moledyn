//! Contains common types that can be glob-imported (`*`) for convenience.

pub use crate::{
    {define_num_args, define_pub_num_args, define_num_args_of_structure,
        define_structure, define_structure_wo_eq, impl_structure, construct_structure,
        impl_fn_info, impl_fn_brief_info, impl_fn_print_configuration,
        impl_fn_read_args_from_vec, impl_fn_read_args_from_lines, impl_argument_trait,
        impl_fmt_for_type, impl_fmt_test, impl_fromstr_for_type, impl_fromstr_test,
        construct_dataset, derive_hash, export_form, pub_export_form, export_data, pub_export_data,
        export_simulation_item, define_total_num_args, read_arguments, export_simulation_info,
        setup_simulation, setup_simulation_fixed, construct_trait_bin
    },
    error::{Error, ErrorCode},
    position::{Position, Numerics},
    argument::{Argument},
    analysis::{Bin, Var1, Analysis, TimeAnalysis, TimeVecAnalysis, ProcessAnalysis, DataSet},
    random_mod::{rng_seed},
    system_mod::{SystemCore, SystemType, BoundaryCond,
            cont_circ::{ContCircSystem, ContCircSystemArguments},
            cont_cubic::{ContCubicSystem, ContCubicSystemArguments},
            cont_cyl::{ContCylindricalSystem, ContCylindricalSystemArguments}
    },
    target_mod::{TargetCore, TargetType,
            // cont_boundary::{ContBoundaryTarget, ContBoundaryTargetArguments},
            cont_bulk::{ContBulkTarget, ContBulkTargetArguments},
    },
    agent_mod::{AgentCore, Passive, Active, Interaction, Merge,
            types::{AgentType, MoveType, InitType, InteractType},
            cont_passive_indep::{ContPassiveIndepAgent, ContPassiveIndepAgentArguments},
            cont_passive_merge::{ContPassiveMergeAgent, ContPassiveMergeAgentArguments},
            cont_passive_exp::{ContPassiveExpAgent, ContPassiveExpAgentArguments},
            cont_passive_lj::{ContPassiveLJAgent, ContPassiveLJAgentArguments},
    },
    time_mod::{TimeType, TimeIterator,
        ConstStep, ConstStepArguments,
        ExponentialStep, ExponentialStepArguments},
    setup::{VariableSimulation, VariableSimulationArguments, ParVariableSimulation, ParVariableSimulationArguments, ProcessSimulation, ProcessSimulationArguments},
    iterator::{Node, LinkedList},
    macros::TypeName,
};

pub use moledyn_proc::simulation;

pub use rand_pcg::Pcg64;
pub use std::{
            iter::Iterator,
            fmt::{self, Display, Formatter},
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
pub use std::f64::consts::PI;


