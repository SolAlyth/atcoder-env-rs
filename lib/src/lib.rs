pub const SUBMISSION: bool = false;

#[doc(hidden)]
pub mod external {
    pub use {
        proconio::{input, marker::{Chars as chars, Usize1 as usize1, Isize1 as isize1}, source::{Source, line::LineSource, once::OnceSource}},
        std::io::{BufReader, BufRead, stdin},
        std::cmp::{min, max, Reverse as Rev},
        std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
        std::mem::swap,
        itertools::Itertools,
        superslice::Ext,
        num_integer::{gcd, lcm}
    };
}



pub mod data_struct {
    pub mod bitset;
    pub mod unionfind;
}

pub mod util {
    pub mod color_print;
    pub mod printer;
    pub mod macros;
}

pub mod math {
    pub mod modulo;
    pub mod prime;
}



pub use crate::{
    data_struct::{
        bitset::{BoolSet, BitSet},
        unionfind::UnionFind
    },
    math::modulo::{u998, i998, Mod},
    
    util::printer::{Printer, end}
};

#[doc(hidden)] pub use external::*;


// for debug
pub use rand::prelude::random;
