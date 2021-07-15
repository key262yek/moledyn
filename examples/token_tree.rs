// #![feature(trace_macros)]
// trace_macros!(true);

#[allow(unused_imports)]
use moledyn_proc::*;
#[allow(unused_imports)]
use moledyn::prelude::*;

fn main() -> Result<(), Error>{
    simulation!("RTS_N_PTL_EXP_SEARCHER", TimeAnalysis,
        ContCircSystem, ContBulkTarget, ContPassiveLJAgent,
        ExponentialStep, VariableSimulation);

    Ok(())
}
