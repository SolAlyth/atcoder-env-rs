/// 提出コードをコンパイルするときに、ライブラリは事前コンパイルのみで使えるようにするため、このような変なフォルダ構成になっている。
///
/// `crate::lib` で全てにアクセスできる。

pub(crate) use lib as mylib;

#[allow(non_upper_case_globals)]
pub mod lib {
    pub const SUBMISSION: bool = false;

    pub const us998: usize = 998244353;
    pub const i998: i128 = 998244353;

    pub const us107: usize = 1000000007;
    pub const i107: i128 = 1000000007;

    const usmod: usize = us998;
    const imod: i128 = i998;

    pub mod data_struct {
        pub mod bitset;
        pub mod unionfind;
        pub mod compress;
        pub mod segtree;
        // pub mod rolling_hash;
        
        pub mod rational;
        
        pub mod traits;
    }

    pub mod util {
        pub mod printer;
        pub mod macros;
        
        pub mod traits;
        pub mod hash;
        pub mod iter;
    }

    pub mod math {
        pub mod modulo;
        pub mod modcalc;
        pub mod prime;
    }

    pub mod geometry {
        pub mod basis;
    }
}


#[doc(hidden)]
pub use {
    lib::*,
    
    lib::{
        SUBMISSION,
        math::modulo::Modulo,
        util::{
            printer::{Printer, end},
            traits::{AssignMinMax, CharFn}
        }
    },
    
    proconio::{
        input,
        marker::{Chars as chars, Usize1 as usize1, Isize1 as isize1},
        source::{Source, line::LineSource, once::OnceSource}
    },
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
