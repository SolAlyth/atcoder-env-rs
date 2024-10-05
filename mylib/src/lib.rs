#![allow(non_upper_case_globals)]

#[allow(unused_imports)]
pub(crate) use crate as mylib;

pub const SUBMISSION: bool = false;

// pub const us998: usize = 998244353;
// pub const i998: i128 = 998244353;
// pub const us107: usize = 1000000007;
// pub const i107: i128 = 1000000007;

pub mod ds {
    pub mod bitset;
    pub mod unionfind;
    pub mod compress;
    // pub mod sqrtset;
    pub mod segtree;
    // pub mod rangeset;
    
    pub mod multiset;
    // pub mod rolling_hash;
    
    // pub mod segtree_new;
    
    pub mod foldable_deque;
    
    pub mod splay_tree;
}

pub mod algo {
    pub mod bellman_ford;
    pub mod warshall_floyd;
    
    pub mod seq;
}

pub mod util {
    pub mod printer;
    pub mod macros;
    pub mod traits;
    pub mod func;
    
    pub mod hyperint;
}

pub mod math {
    pub mod prime;
    pub mod barrett;
    pub mod matrix;
}

/* pub mod geometry {
    pub mod basis;
} */

pub mod traits {
    pub mod abstracts;
}


#[doc(hidden)]
pub use {
    util::{
        printer::{out, end, EndFlag}, traits::*, hyperint::h64, func::binary_search
    },
    
    proconio::{
        input, input_interactive,
        marker::{Bytes as bytes, Chars as chars, Usize1 as usize1, Isize1 as isize1}
    },
    std::io::{BufReader, BufRead, stdin},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet},
    std::mem::{swap, replace},
    itertools::{Itertools, iproduct, izip},
    superslice::Ext,
    num::integer::{gcd, lcm, Roots},
    ac_library,
    
    rand
};
