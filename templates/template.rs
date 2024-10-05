#![allow(unused_must_use, non_snake_case)]

#[allow(unused_imports)]
use {mylib::*, ac_library::ModInt998244353 as Mint};
const INTERACTIVE: bool = false;

fn solve() {
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() { out.init(if INTERACTIVE || !SUBMISSION { EndFlag::Print } else { EndFlag::LineFeed }); solve(); out.print() }

#[test] fn test() { out.init(EndFlag::Print); out.print(); }
