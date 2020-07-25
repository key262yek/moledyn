// Functionality for analysis

use crate::prelude::*;

pub struct<Sys, T, S> Analysis<Sys,T, S>
    where Sys : SystemCore + Argument,
          T   : TargetCore + Argument,
          S   : SearcherCore + Argument,
          Sim : Argument{
    sys : Sys,
    target : T,
    searcher : S,
    sim     : Sim,
}


