use crate::prelude::*;
use std::default::Default;


// =====================================================================================
// ===  Implement SearcherType =========================================================
// =====================================================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum SearcherType{                          // Type of searcher
    ContinuousPassiveIndependent,
    ContinuousPassiveInteracting,
    ContinuousActiveIndependent,
    ContinuousActiveInteracting,
    LatticePassiveIndependent,
    LatticePassiveInteracting,
    LatticeActiveIndependent,
    LatticeActiveInteracting,
    NetworkPassiveIndependent,
    NetworkPassiveInteracting,
    NetworkActiveIndependent,
    NetworkActiveInteracting,
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
            SearcherType::ContinuousPassiveIndependent
            | SearcherType::ContinuousPassiveInteracting
            | SearcherType::ContinuousActiveIndependent
            | SearcherType::ContinuousActiveInteracting => true,
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
            SearcherType::ContinuousPassiveIndependent
            | SearcherType::ContinuousPassiveInteracting
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
            SearcherType::ContinuousActiveIndependent
            | SearcherType::ContinuousActiveInteracting
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
            SearcherType::ContinuousPassiveIndependent
            | SearcherType::ContinuousActiveIndependent
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
            SearcherType::ContinuousPassiveInteracting
            | SearcherType::ContinuousActiveInteracting
            | SearcherType::LatticePassiveInteracting
            | SearcherType::LatticeActiveInteracting
            | SearcherType::NetworkPassiveInteracting
            | SearcherType::NetworkActiveInteracting => true,
            _ => false,
        }
    }
}

impl_fmt_for_type!(SearcherType,
    SearcherType::ContinuousPassiveIndependent => "Passive Independent Searcher in Continous system.",
    SearcherType::ContinuousPassiveInteracting => "Passive Interacting Searcher in Continous system.",
    SearcherType::ContinuousActiveIndependent => "Active Independent Searcher in Continous system.",
    SearcherType::ContinuousActiveInteracting => "Active Interacting Searcher in Continous system.",
    SearcherType::LatticePassiveIndependent => "Passive Independent Searcher in Lattice system.",
    SearcherType::LatticePassiveInteracting => "Passive Interacting Searcher in Lattice system.",
    SearcherType::LatticeActiveIndependent => "Active Independent Searcher in Lattice system.",
    SearcherType::LatticeActiveInteracting => "Active Interacting Searcher in Lattice system.",
    SearcherType::NetworkPassiveIndependent => "Passive Independent Searcher in Network.",
    SearcherType::NetworkPassiveInteracting => "Passive Interacting Searcher in Network.",
    SearcherType::NetworkActiveIndependent => "Active Independent Searcher in Network.",
    SearcherType::NetworkActiveInteracting => "Active Interacting Searcher in Network.");

impl_fromstr_for_type!(SearcherType,
    SearcherType::ContinuousPassiveIndependent => "Passive Independent Searcher in Continous system.",
    SearcherType::ContinuousPassiveInteracting => "Passive Interacting Searcher in Continous system.",
    SearcherType::ContinuousActiveIndependent => "Active Independent Searcher in Continous system.",
    SearcherType::ContinuousActiveInteracting => "Active Interacting Searcher in Continous system.",
    SearcherType::LatticePassiveIndependent => "Passive Independent Searcher in Lattice system.",
    SearcherType::LatticePassiveInteracting => "Passive Interacting Searcher in Lattice system.",
    SearcherType::LatticeActiveIndependent => "Active Independent Searcher in Lattice system.",
    SearcherType::LatticeActiveInteracting => "Active Interacting Searcher in Lattice system.",
    SearcherType::NetworkPassiveIndependent => "Passive Independent Searcher in Network.",
    SearcherType::NetworkPassiveInteracting => "Passive Interacting Searcher in Network.",
    SearcherType::NetworkActiveIndependent => "Active Independent Searcher in Network.",
    SearcherType::NetworkActiveInteracting => "Active Interacting Searcher in Network.");

impl Default for SearcherType{
    fn default() -> Self{
        SearcherType::ContinuousPassiveIndependent
    }
}

// =====================================================================================
// ===  Implement MoveType =============================================================
// =====================================================================================

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum MoveType{
    Brownian(f64),      // Brownian motion with given diffusion coefficient
    Levy,               // Levy walk. not developed yet.
}

impl Display for MoveType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            MoveType::Brownian(coeff_diff) =>
                write!(f, "Brownian with diffusion coefficient {}", coeff_diff),
            MoveType::Levy =>
                write!(f, "Levy walk"),
        }
    }
}


impl FromStr for MoveType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        match split[0]{
            "Brownian" => Ok(MoveType::Brownian(split[4].parse::<f64>().expect("Failed to parse"))),
            "Levy" => Ok(MoveType::Levy),
            _ => {
                if split.len() == 1{
                    split[0].parse::<f64>().map(|c| MoveType::Brownian(c))
                                   .map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
                else{
                    Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput))
                }
            },
        }

    }
}

impl Default for MoveType{
    fn default() -> Self{
        MoveType::Brownian(1f64)
    }
}


// =====================================================================================
// ===  Implement InitType =============================================================
// =====================================================================================

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum InitType<T>{
    SpecificPosition(Position<T>),      // Brownian motion with given diffusion coefficient
    Uniform,               // Levy walk. not developed yet.
}



impl<T : Display> Display for InitType<T>{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            InitType::SpecificPosition(pos) =>
                write!(f, "Initialize all searchers at {}", pos),
            InitType::Uniform =>
                write!(f, "Initialize searchers uniformly"),
        }
    }
}

impl<T : FromStr + Default + Clone> FromStr for InitType<T>{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let split : Vec<&str> = s.split_whitespace().collect();
        if split.len() == 1{
            match split[0]{
                "Uniform" => Ok(InitType::<T>::Uniform),
                _ => {
                    split[0].parse::<Position<T>>().map(|pos| InitType::<T>::SpecificPosition(pos))
                },
            }
        }
        else{
            match split[1]{
                "all" => Ok(InitType::<T>::SpecificPosition(split[4].parse::<Position<T>>().expect("Failed to parse"))),
                "searchers" => Ok(InitType::<T>::Uniform),
                _ => Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput)),
            }
        }
    }
}

impl<T> Default for InitType<T>{
    fn default() -> Self{
        InitType::Uniform
    }
}






// =====================================================================================
// ===  Implement InteractType =============================================================
// =====================================================================================


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum InteractType{
    Exponential(usize, f64, f64),      // Exponential form f(r) = s C exp(- r / g), first : dim, second : g, third : s, C : determined by g
    Coulomb(usize, f64),             // Coulomb potential f(r) = k / 2pi * log(r / r0) , first : dim, second : k
}

impl InteractType{
    #[allow(dead_code)]
    fn dim(&self) -> usize{
        match *self{
            InteractType::Exponential(dim, _g, _s) => dim,
            InteractType::Coulomb(dim, _s) => dim,
        }
    }

    #[allow(dead_code)]
    fn name(&self) -> &str{
        match *self{
            InteractType::Exponential(_d, _g, _s) => "Exponential",
            InteractType::Coulomb(_d, _s) => "Coulomb",
        }
    }
}

impl Display for InteractType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            InteractType::Exponential(dim, gamma, strength) =>
                write!(f, "Exponential form in {} D with gamma= {} strength= {}", dim, gamma, strength),
            InteractType::Coulomb(dim, strength) =>
                write!(f, "Coulomb potential in {} D with strength= {}", dim, strength),
        }
    }
}


impl FromStr for InteractType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        if split.len() == 1{
            let length = split[0].len();
            let split : Vec<&str> = split[0][0..length-1].split('(').collect();  // remove character ')' and split '('
            let parameters : Vec<&str> = split[1].split(',').collect();
            match split[0]{
                "Exponential" => {
                    return Ok(InteractType::Exponential(parameters[0].parse::<usize>().expect("Failed to parse"),
                                                          parameters[1].parse::<f64>().expect("Failed to parse"),
                                                          parameters[2].parse::<f64>().expect("Failed to parse")));
                },
                "Coulomb" => {
                    return Ok(InteractType::Coulomb(parameters[0].parse::<usize>().expect("Failed to parse"),
                                                  parameters[1].parse::<f64>().expect("Failed to parse")));
                },
                _ => {return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));},
            }
        }
        else{
            match split[0]{
                "Exponential" => Ok(InteractType::Exponential(split[3].parse::<usize>().expect("Failed to parse"),
                                                              split[7].parse::<f64>().expect("Failed to parse"),
                                                              split[9].parse::<f64>().expect("Failed to parse"))),
                "Coulomb" => Ok(InteractType::Coulomb(split[3].parse::<usize>().expect("Failed to parse"),
                                                      split[7].parse::<f64>().expect("Failed to parse"))),
                _ => Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput)),
            }
        }
    }
}

impl Default for InteractType{
    fn default() -> Self{
        InteractType::Exponential(2, 1.0, 0.0)
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use crate::{impl_fmt_test, impl_fromstr_test};

    impl_fmt_test!(test_fmt_searchertype,
        SearcherType::ContinuousPassiveIndependent => "Passive Independent Searcher in Continous system.",
        SearcherType::ContinuousPassiveInteracting => "Passive Interacting Searcher in Continous system.",
        SearcherType::ContinuousActiveIndependent => "Active Independent Searcher in Continous system.",
        SearcherType::ContinuousActiveInteracting => "Active Interacting Searcher in Continous system.",
        SearcherType::LatticePassiveIndependent => "Passive Independent Searcher in Lattice system.",
        SearcherType::LatticePassiveInteracting => "Passive Interacting Searcher in Lattice system.",
        SearcherType::LatticeActiveIndependent => "Active Independent Searcher in Lattice system.",
        SearcherType::LatticeActiveInteracting => "Active Interacting Searcher in Lattice system.",
        SearcherType::NetworkPassiveIndependent => "Passive Independent Searcher in Network.",
        SearcherType::NetworkPassiveInteracting => "Passive Interacting Searcher in Network.",
        SearcherType::NetworkActiveIndependent => "Active Independent Searcher in Network.",
        SearcherType::NetworkActiveInteracting => "Active Interacting Searcher in Network.");

    impl_fromstr_test!(test_fromstr_searchertype,
        SearcherType,
        SearcherType::ContinuousPassiveIndependent => "Passive Independent Searcher in Continous system.",
        SearcherType::ContinuousPassiveInteracting => "Passive Interacting Searcher in Continous system.",
        SearcherType::ContinuousActiveIndependent => "Active Independent Searcher in Continous system.",
        SearcherType::ContinuousActiveInteracting => "Active Interacting Searcher in Continous system.",
        SearcherType::LatticePassiveIndependent => "Passive Independent Searcher in Lattice system.",
        SearcherType::LatticePassiveInteracting => "Passive Interacting Searcher in Lattice system.",
        SearcherType::LatticeActiveIndependent => "Active Independent Searcher in Lattice system.",
        SearcherType::LatticeActiveInteracting => "Active Interacting Searcher in Lattice system.",
        SearcherType::NetworkPassiveIndependent => "Passive Independent Searcher in Network.",
        SearcherType::NetworkPassiveInteracting => "Passive Interacting Searcher in Network.",
        SearcherType::NetworkActiveIndependent => "Active Independent Searcher in Network.",
        SearcherType::NetworkActiveInteracting => "Active Interacting Searcher in Network.");

    #[test]
    fn test_fmt_move_type(){
        assert_eq!(format!("{}", MoveType::Brownian(0.0)).as_str(),
            "Brownian with diffusion coefficient 0");
        assert_eq!(format!("{}", MoveType::Levy).as_str(),
            "Levy walk");
    }

    #[test]
    fn test_fromstr_move_type(){
        let test1 = "Brownian with diffusion coefficient 0";
        let result1 = Ok(MoveType::Brownian(0.0));

        assert_eq!(MoveType::from_str(test1), result1);

        let test2 = "Levy walk";
        let result2 = Ok(MoveType::Levy);
        assert_eq!(MoveType::from_str(test2), result2);
    }

    #[test]
    fn test_fmt_init_type(){
        assert_eq!(format!("{}", InitType::<f64>::SpecificPosition(Position::<f64>::new(vec![0.0; 2]))).as_str(),
            "Initialize all searchers at 0,0");
        assert_eq!(format!("{}", InitType::<f64>::Uniform).as_str(),
            "Initialize searchers uniformly");
    }

    #[test]
    fn test_fromstr_init_type(){
        let test1 = "Initialize all searchers at 0,0";
        let result1 = Ok(InitType::<f64>::SpecificPosition(Position::<f64>::new(vec![0.0; 2])));
        assert_eq!(InitType::<f64>::from_str(test1), result1);

        let test2 = "Initialize searchers uniformly";
        let result2 = Ok(InitType::<f64>::Uniform);
        assert_eq!(InitType::<f64>::from_str(test2), result2);
    }


    #[test]
    fn test_classify(){
        assert_eq!(SearcherType::ContinuousPassiveIndependent.in_continous(), true);
        assert_eq!(SearcherType::ContinuousPassiveIndependent.in_lattice(), false);
        assert_eq!(SearcherType::ContinuousPassiveIndependent.in_network(), false);

        assert_eq!(SearcherType::ContinuousPassiveIndependent.is_passive(), true);
        assert_eq!(SearcherType::ContinuousPassiveIndependent.is_active(), false);

        assert_eq!(SearcherType::ContinuousPassiveIndependent.is_independent(), true);
        assert_eq!(SearcherType::ContinuousPassiveIndependent.is_interacting(), false);
    }

    #[test]
    fn test_fn_interact_type(){
        let test1 = InteractType::Exponential(2, 1f64, 3f64);
        assert_eq!(test1.dim(), 2);
        assert_eq!(test1.name(), "Exponential");

        let test2 = InteractType::Coulomb(3, 1f64);
        assert_eq!(test2.dim(), 3);
        assert_eq!(test2.name(), "Coulomb");
    }

    #[test]
    fn test_fmt_interact_type(){
        assert_eq!(format!("{}", InteractType::Exponential(2, 0.1, 0.0)).as_str(),
            "Exponential form in 2 D with gamma= 0.1 strength= 0");
        assert_eq!(format!("{}", InteractType::Coulomb(2, 1.0)).as_str(),
            "Coulomb potential in 2 D with strength= 1");
        assert_eq!(format!("{}", InteractType::Coulomb(3, 1.0)).as_str(),
            "Coulomb potential in 3 D with strength= 1");
    }

    #[test]
    fn test_fromstr_interact_type(){
        let test1 =  "Exponential form in 2 D with gamma= 0.1 strength= 0";
        let result1 = Ok(InteractType::Exponential(2, 0.1, 0.0));
        assert_eq!(InteractType::from_str(test1), result1);

        let test2 = "Coulomb potential in 2 D with strength= 1";
        let result2 = Ok(InteractType::Coulomb(2, 1.0));
        assert_eq!(InteractType::from_str(test2), result2);

        let test3 = "Coulomb potential in 3 D with strength= 1";
        let result3 = Ok(InteractType::Coulomb(3, 1.0));
        assert_eq!(InteractType::from_str(test3), result3);

        let test4 = "Exponential(2,1,0)";
        let result4 = Ok(InteractType::Exponential(2, 1f64, 0f64));
        assert_eq!(InteractType::from_str(test4), result4);

        let test5 = "Coulomb(2,1.0)";
        let result5 = Ok(InteractType::Coulomb(2, 1f64));
        assert_eq!(InteractType::from_str(test5), result5);

        let test6 = "Coulomb3D(2,1.0)";
        let result6 = Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        assert_eq!(InteractType::from_str(test6), result6);
    }

    #[test]
    #[should_panic]
    fn test_fromstr_panic() -> (){
        let test7 = "Coulomb(a,1.0)";
        InteractType::from_str(test7).expect("Panic occurs");
    }
}

