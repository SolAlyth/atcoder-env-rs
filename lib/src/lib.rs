pub const SUBMISSION: bool = false;

pub mod data_struct {
    pub mod bitset; // BitSet is todo
    pub mod unionfind;
    pub mod compress;
    // pub mod rolling_hash;
}

pub mod util {
    pub mod color_print;
    pub mod printer;
    pub mod macros;
    
    pub mod traits;
    pub mod hash; // is todo
}

pub mod math {
    pub mod modulo;
    pub mod prime;
}



pub use crate::{
    data_struct::{
        bitset::{BoolSet, BitSet},
        unionfind::UnionFind,
        compress::{Numbering, Compress}
    },
    math::modulo::{i998, us998, Mod, ModCalc},
    
    util::{
        printer::{Printer, end},
        traits::{Update, CharFn},
        hash::hash
    }
};

#[doc(hidden)]
pub use {
    proconio::{input, marker::{Chars as chars, Usize1 as usize1, Isize1 as isize1}, source::{Source, line::LineSource, once::OnceSource}},
    std::io::{BufReader, BufRead, stdin},
    std::cmp::{min, max, Reverse as Rev},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    std::mem::swap,
    itertools::Itertools,
    superslice::Ext,
    num_integer::{gcd, lcm, Roots}
};


// for debug
// pub use rand::prelude::random;
