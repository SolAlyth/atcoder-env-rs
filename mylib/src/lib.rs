#![allow(non_upper_case_globals)]

#[allow(unused_imports)]
pub(crate) use crate as mylib;

pub const SUBMISSION: bool = false;

pub const us998: usize = 998244353;
pub const i998: i128 = 998244353;
// pub const us107: usize = 1000000007;
// pub const i107: i128 = 1000000007;

pub type Mint = ac_library::ModInt998244353;

pub mod data_struct {
    pub mod bitset;
    pub mod unionfind;
    pub mod compress;
    // pub mod sqrtset;
    pub mod segtree;
    pub mod inversible;
    // pub mod rangeset;
    
    // pub mod multiset;
    // pub mod rolling_hash;
    
    // pub mod segtree_new;
}

pub mod util {
    pub mod printer;
    pub mod macros;
    pub mod traits;
    pub mod func;
}

pub mod math {
    pub mod prime;
    // pub mod modint;
    // pub mod modcalc;
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
        printer::{Printer, end, EndFlag},
        traits::*
    },
    
    proconio::{
        input,
        marker::{Bytes as bytes, Chars as chars, Usize1 as usize1, Isize1 as isize1},
        source::{Source, line::LineSource, once::OnceSource}
    },
    std::io::{BufReader, BufRead, stdin},
    std::collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet},
    std::mem::swap,
    itertools::{Itertools, iproduct, izip},
    superslice::Ext,
    num::integer::{gcd, lcm, Roots}
};


// for debug
// pub use rand::prelude::random;
