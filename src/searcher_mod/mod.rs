// Modules for Searcher configuration
//
// Searcher is defined with size, position, and its various characteristic of movement


use std::fmt::{Display, Formatter, self};
use crate::error::Error;
use crate::position::Position;
use rand_pcg::Pcg64;

pub mod cont_passive_indep;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum SearcherType{
    ContinousPassiveIndependent,
    ContinousPassiveInteracting,
    ContinousActiveIndependent,
    ContinousActiveInteracting,
    LatticePassiveIndependent,
    LatticePassiveInteracting,
    LatticeActiveIndependent,
    LatticeActiveInteracting,
    NetworkPassiveIndependent,
    NetworkPassiveInteracting,
    NetworkActiveIndependent,
    NetworkActiveInteracting,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MoveType{
    Brownian(f64),      // Brownian motion with given diffusion coefficient
    Levy,
}

pub trait SearcherCore<T>{

}

pub trait Passive<T>{
    // Random movement
    fn random_move(&self, rng : &mut Pcg64, dt : T) -> Result<Position<T>, Error>;

    // add random movement to vector
    fn random_move_to_vec(&self, rng: &mut Pcg64, dt: f64, vec: &mut Position<f64>) -> Result<(), Error>;
}

pub trait Active<T>{
    // Active motion
    fn active_move(&self) -> Position<T>;
}

pub trait Interaction<T>{
    // Interaction between particles
    fn force(&self, other : &Self) -> Position<T>;
}

pub trait Merge{
    // Merge two searchers.
    fn merge(&self, other : &Self);
}

impl SearcherType{
    #[allow(dead_code)]
    fn in_lattice(&self) -> bool{
        match self{
            SearcherType::LatticePassiveIndependent
            | SearcherType::LatticePassiveInteracting
            | SearcherType::LatticeActiveIndependent
            | SearcherType::LatticeActiveInteracting => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn in_continous(&self) -> bool{
        match self{
            SearcherType::ContinousPassiveIndependent
            | SearcherType::ContinousPassiveInteracting
            | SearcherType::ContinousActiveIndependent
            | SearcherType::ContinousActiveInteracting => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn in_network(&self) -> bool{
        match self{
            SearcherType::NetworkPassiveIndependent
            | SearcherType::NetworkPassiveInteracting
            | SearcherType::NetworkActiveIndependent
            | SearcherType::NetworkActiveInteracting => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_passive(&self) -> bool{
        match self{
            SearcherType::ContinousPassiveIndependent
            | SearcherType::ContinousPassiveInteracting
            | SearcherType::LatticePassiveIndependent
            | SearcherType::LatticePassiveInteracting
            | SearcherType::NetworkPassiveIndependent
            | SearcherType::NetworkPassiveInteracting => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_active(&self) -> bool{
        match self{
            SearcherType::ContinousActiveIndependent
            | SearcherType::ContinousActiveInteracting
            | SearcherType::LatticeActiveIndependent
            | SearcherType::LatticeActiveInteracting
            | SearcherType::NetworkActiveIndependent
            | SearcherType::NetworkActiveInteracting => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_independent(&self) -> bool{
        match self{
            SearcherType::ContinousPassiveIndependent
            | SearcherType::ContinousActiveIndependent
            | SearcherType::LatticePassiveIndependent
            | SearcherType::LatticeActiveIndependent
            | SearcherType::NetworkPassiveIndependent
            | SearcherType::NetworkActiveIndependent => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn is_interacting(&self) -> bool{
        match self{
            SearcherType::ContinousPassiveInteracting
            | SearcherType::ContinousActiveInteracting
            | SearcherType::LatticePassiveInteracting
            | SearcherType::LatticeActiveInteracting
            | SearcherType::NetworkPassiveInteracting
            | SearcherType::NetworkActiveInteracting => true,
            _ => false,
        }
    }
}

impl Display for SearcherType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            SearcherType::ContinousPassiveIndependent =>
                write!(f, "Passive Independent Searcher in Continous system."),
            SearcherType::ContinousPassiveInteracting =>
                write!(f, "Passive Interacting Searcher in Continous system."),
            SearcherType::ContinousActiveIndependent =>
                write!(f, "Active Independent Searcher in Continous system."),
            SearcherType::ContinousActiveInteracting =>
                write!(f, "Active Interacting Searcher in Continous system."),
            SearcherType::LatticePassiveIndependent =>
                write!(f, "Passive Independent Searcher in Lattice system."),
            SearcherType::LatticePassiveInteracting =>
                write!(f, "Passive Interacting Searcher in Lattice system."),
            SearcherType::LatticeActiveIndependent =>
                write!(f, "Active Independent Searcher in Lattice system."),
            SearcherType::LatticeActiveInteracting =>
                write!(f, "Active Interacting Searcher in Lattice system."),
            SearcherType::NetworkPassiveIndependent =>
                write!(f, "Passive Independent Searcher in Network."),
            SearcherType::NetworkPassiveInteracting =>
                write!(f, "Passive Interacting Searcher in Network."),
            SearcherType::NetworkActiveIndependent =>
                write!(f, "Active Independent Searcher in Network."),
            SearcherType::NetworkActiveInteracting =>
                write!(f, "Active Interacting Searcher in Network."),
        }
    }
}

impl Display for MoveType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            MoveType::Brownian(coeff_diff) =>
                write!(f, "Brownian with diffusion coefficient {}", coeff_diff),
            MoveType::Levy =>
                write!(f, "Levy"),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_fmt(){
        assert_eq!(format!("{}", SearcherType::ContinousPassiveIndependent).as_str(),
            "Passive Independent Searcher in Continous system.");
        assert_eq!(format!("{}", SearcherType::ContinousPassiveInteracting).as_str(),
            "Passive Interacting Searcher in Continous system.");
        assert_eq!(format!("{}", SearcherType::ContinousActiveIndependent).as_str(),
            "Active Independent Searcher in Continous system.");
        assert_eq!(format!("{}", SearcherType::ContinousActiveInteracting).as_str(),
            "Active Interacting Searcher in Continous system.");
        assert_eq!(format!("{}", SearcherType::LatticePassiveIndependent).as_str(),
            "Passive Independent Searcher in Lattice system.");
        assert_eq!(format!("{}", SearcherType::LatticePassiveInteracting).as_str(),
            "Passive Interacting Searcher in Lattice system.");
        assert_eq!(format!("{}", SearcherType::LatticeActiveIndependent).as_str(),
            "Active Independent Searcher in Lattice system.");
        assert_eq!(format!("{}", SearcherType::LatticeActiveInteracting).as_str(),
            "Active Interacting Searcher in Lattice system.");
        assert_eq!(format!("{}", SearcherType::NetworkPassiveIndependent).as_str(),
            "Passive Independent Searcher in Network.");
        assert_eq!(format!("{}", SearcherType::NetworkPassiveInteracting).as_str(),
            "Passive Interacting Searcher in Network.");
        assert_eq!(format!("{}", SearcherType::NetworkActiveIndependent).as_str(),
            "Active Independent Searcher in Network.");
        assert_eq!(format!("{}", SearcherType::NetworkActiveInteracting).as_str(),
            "Active Interacting Searcher in Network.");

        assert_eq!(format!("{}", MoveType::Brownian(0.0)).as_str(),
            "Brownian with diffusion coefficient 0");
        assert_eq!(format!("{}", MoveType::Levy).as_str(),
            "Levy");
    }

    #[test]
    fn test_classify(){
        assert_eq!(SearcherType::ContinousPassiveIndependent.in_continous(), true);
        assert_eq!(SearcherType::ContinousPassiveIndependent.in_lattice(), false);
        assert_eq!(SearcherType::ContinousPassiveIndependent.in_network(), false);

        assert_eq!(SearcherType::ContinousPassiveIndependent.is_passive(), true);
        assert_eq!(SearcherType::ContinousPassiveIndependent.is_active(), false);

        assert_eq!(SearcherType::ContinousPassiveIndependent.is_independent(), true);
        assert_eq!(SearcherType::ContinousPassiveIndependent.is_interacting(), false);
    }
}
