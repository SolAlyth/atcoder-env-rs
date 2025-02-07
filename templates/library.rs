#[cfg(not(debug_assertions))] #[allow(unused)]
mod mylib {
    #![allow(non_upper_case_globals)]
    
    pub const SUBMISSION: bool = true;
    
    pub use {
        util::{
            printer::{out, end, EndFlag}, traits::*, func::binary_search
        }
    };
    
    pub mod ds {
        pub mod compress {
            // source: ds/compress.rs
        }
        
        pub mod unionfind {
            // source: ds/unionfind.rs
        }
        
        pub mod foldable_deque {
            // source: ds/foldable_deque.rs
        }
        
        pub mod splay_tree {
            // source: ds/splay_tree.rs
        }
    }
    
    pub mod math {
        // FIXME: math の整備
    }
    
    pub mod util {
        pub mod printer {
            // source: util/printer.rs
        }
        
        pub mod traits {
            // source: util/traits.rs
        }
        
        pub mod macros {
            // source: util/macros.rs
        }
        
        pub mod func {
            // source: util/func.rs
        }
    }
}

mod external {
    pub use {
        proconio::{
            input, input_interactive,
            marker::{Bytes as bytes, Chars as chars, Usize1 as usize1, Isize1 as isize1}
        },
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet},
        std::mem::{swap, replace},
        itertools::{Itertools, iproduct, izip},
        superslice::Ext,
        num_integer::{gcd, lcm, Roots},
        ac_library,
        
        rand
    };
    
}
