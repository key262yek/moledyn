use crate::prelude::*;
use std::default::Default;
use std::convert::From;


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
    SpecificPosition(Position<T>),      // Initiate searcher at specific position
    Uniform,               // Initiate searchers with uniform distribution
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
    Exponential(usize, f64),
    // Exponential form V(r) = C exp(- r / g), C depends on a dimension and typical length
    // dim : usize / dimension
    // gamma : f64 / typical length
    Coulomb2D,
    // Coulomb potential V(r) = log(r) / 2pi
    Coulomb3D,
    // Coulomb potential V(r) = 1 / 4 pi r
    LennardJones(f64),
    // Lennard Jones potential V(r) = 4[(a / r)^12 - (a / r)^6 ],
    // ptl_size : f64 / size of particle, should be O(1)
}



impl Display for InteractType{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result{
        match self{
            InteractType::Exponential(dim, gamma) =>
                write!(f, "Exponential form interaction in {} D with typical length {}", dim, gamma),
            InteractType::Coulomb2D =>
                write!(f, "Coulomb potential in 2 D"),
            InteractType::Coulomb3D =>
                write!(f, "Coulomb potential in 3 D"),
            InteractType::LennardJones(ptl_size) =>
                write!(f, "Lennard Jones potential with particle size {}", ptl_size),
        }
    }
}


impl FromStr for InteractType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split : Vec<&str> = s.split_whitespace().collect();
        if split.len() == 1{
            let split : Vec<&str> = split[0].split(&['(', ')'][..]).collect();
            let name = split[0];
            match name{
                "Exponential" => {
                    let args : Vec<&str> = split[1].split(',').collect();
                    if args.len() != 2{
                        return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                    }
                    let dim : usize = args[0].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    let gamma : f64 = args[1].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    return Ok(InteractType::Exponential(dim, gamma));
                },
                "Coulomb2D" => {
                    return Ok(InteractType::Coulomb2D);
                },
                "Coulomb3D" => {
                    return Ok(InteractType::Coulomb3D);
                },
                "LennardJones" => {
                    let ptl_size : f64 = split[1].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    return Ok(InteractType::LennardJones(ptl_size));
                }
                _ => {
                    return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                }
            }
        } else {
            match split[0]{
                "Exponential" => {
                    if split.len() != 10{
                        return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                    }
                    let dim : usize = split[4].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    let gamma : f64 = split[9].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    return Ok(InteractType::Exponential(dim, gamma));
                },
                "Coulomb" =>{
                    if split.len() != 5{
                        return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                    }
                    match split[3]{
                        "2" => {
                            return Ok(InteractType::Coulomb2D);
                        },
                        "3" =>{
                            return Ok(InteractType::Coulomb3D);
                        }
                        _ => {
                            return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                        }
                    }
                },
                "Lennard" => {
                    if split.len() != 7{
                        return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                    }
                    let ptl_size = split[6].parse().map_err(|_e| Error::make_error_syntax(ErrorCode::InvalidArgumentInput))?;
                    return Ok(InteractType::LennardJones(ptl_size));
                },
                _ => {
                    return Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
                }
            }
        }
    }
}

impl Default for InteractType{
    fn default() -> Self{
        InteractType::Exponential(2, 0.1)
    }
}

impl From<InteractType> for f64{
    fn from(item : InteractType) -> Self{
        match item {
            InteractType::Exponential(_dim, gamma) => gamma,
            InteractType::LennardJones(ptl_size) => ptl_size,
            _ => 0.0,
        }
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
    fn test_fmt_interact_type(){
        assert_eq!(format!("{}", InteractType::Exponential(2, 0.1)).as_str(),
            "Exponential form interaction in 2 D with typical length 0.1");
        assert_eq!(format!("{}", InteractType::Coulomb2D).as_str(),
            "Coulomb potential in 2 D");
        assert_eq!(format!("{}", InteractType::Coulomb3D).as_str(),
            "Coulomb potential in 3 D");
        assert_eq!(format!("{}", InteractType::LennardJones(1.0)).as_str(),
            "Lennard Jones potential with particle size 1");
    }


    #[test]
    fn test_fromstr_interact_type(){
        let test =  "Exponential form interaction in 2 D with typical length 0.1";
        let result = Ok(InteractType::Exponential(2, 0.1));
        assert_eq!(InteractType::from_str(test), result);

        let test = "Coulomb potential in 2 D";
        let result = Ok(InteractType::Coulomb2D);
        assert_eq!(InteractType::from_str(test), result);

        let test = "Coulomb potential in 3 D";
        let result = Ok(InteractType::Coulomb3D);
        assert_eq!(InteractType::from_str(test), result);

        let test = "Lennard Jones potential with particle size 1.0";
        let result = Ok(InteractType::LennardJones(1.0));
        assert_eq!(InteractType::from_str(test), result);

        let test = "Exponential(2,0.1)";
        let result = Ok(InteractType::Exponential(2, 0.1));
        assert_eq!(InteractType::from_str(test), result);

        let test = "Coulomb2D";
        let result = Ok(InteractType::Coulomb2D);
        assert_eq!(InteractType::from_str(test), result);

        let test = "Coulomb3D";
        let result = Ok(InteractType::Coulomb3D);
        assert_eq!(InteractType::from_str(test), result);

        let test = "LennardJones(1.0)";
        let result = Ok(InteractType::LennardJones(1.0));
        assert_eq!(InteractType::from_str(test), result);

        let test =  "Exponential form interaction in 2 D with typical length";
        let result = Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        assert_eq!(InteractType::from_str(test), result);

        let test =  "Test form interaction in 2 D with typical length 1.0";
        let result = Err(Error::make_error_syntax(ErrorCode::InvalidArgumentInput));
        assert_eq!(InteractType::from_str(test), result);
    }

    #[test]
    #[should_panic]
    fn test_fromstr_panic() -> (){
        let test7 = "Coulomb(a,1.0)";
        InteractType::from_str(test7).expect("Panic occurs");
    }

    #[test]
    fn test_from_into(){
        let test = InteractType::Exponential(2, 1.0);
        assert_eq!(f64::from(test), 1.0);

        let test = InteractType::Coulomb2D;
        assert_eq!(f64::from(test), 0.0);

        let test = InteractType::Coulomb3D;
        assert_eq!(f64::from(test), 0.0);

        let test = InteractType::LennardJones(2.0);
        assert_eq!(f64::from(test), 2.0);
    }
}

