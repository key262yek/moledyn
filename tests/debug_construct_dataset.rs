


use rts::prelude::*;
use rts::system_mod::cont_circ::{ContCircSystem, ContCircSystemArguments};
use rts::target_mod::cont_bulk::{ContBulkTarget, ContBulkTargetArguments};



#[test]
fn test_construct_dataset2(){
        // Dataset
        construct_dataset!(ContCircSystem, _sys_arg, ContCircSystemArguments,
                        [];
                        ContBulkTarget, target_arg, ContBulkTargetArguments,
                        [target_size, f64]);
}
