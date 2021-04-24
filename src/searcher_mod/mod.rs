// Modules for Searcher configuration
//
// Searcher is defined with size, position, and its various characteristic of movement


use crate::prelude::*;

// =====================================================================================
// ===  Implement Searcher =============================================================
// =====================================================================================

pub trait SearcherCore<T>{    // Core functionality of searcher.
    fn pos(&self) -> &Position<T>;

    // Mutual displacement
    // Position<T> indicates the direction between particles (hence, normal vector)
    // T indicates distance between them
    fn mutual_displacement(&self, other : &Self) -> Result<(Position<T>, T), Error>;

    fn mutual_displacement_to_vec(&self, other : &Self, vec : &mut Position<T>) -> Result<T, Error>;

    fn mutual_distance(&self, other : &Self) -> Result<T, Error>;
}

pub trait Passive<T, F>{         // Functions for Passive ptls
    // Random movement
    fn random_move(&self, rng : &mut Pcg64, dt : F) -> Result<Position<T>, Error>;

    // add random movement to vector
    fn random_move_to_vec(&self, rng: &mut Pcg64, dt: F, vec: &mut Position<T>) -> Result<(), Error>;
}

pub trait Active<T>{          // Functions for Active ptls
    // Active motion
    fn active_move(&self) -> Position<T>;
}

pub trait Interaction<T, F>{

    // Interaction between particles
    fn potential(&self, radius : T) -> F;

    fn force(&self, radius : T) -> F;

    fn add_force(&mut self, vec : &Position<T>) -> Result<(), Error>;
}

pub trait Merge{
    // Merge two searchers.
    fn merge(&mut self, other : &Self) -> Result<(), Error>;

    // Give size info
    fn size(&self) -> usize;

    // Add size
    fn add_size(&mut self, size : usize) -> Result<(), Error>;
}

pub mod types;
pub mod cont_passive_indep;     // 연속 시스템에서 Passive하게 움직이는 독립된 searcher
pub mod cont_passive_merge;     // 연속 시스템에서 Passive하게 움직이는 서로 합쳐질 수 있는 searcher
pub mod cont_passive_exp;  //연속 시스템에서 Passive하게 움직이며, interaction을 주고 받는 Searcher




