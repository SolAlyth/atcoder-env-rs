#![allow(unused_must_use, non_snake_case)]

use mylib::*;
const INTERACTIVE: bool = false;

fn solve() {
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn main() {
    out.init(if INTERACTIVE || !SUBMISSION { EndFlag::Print } else {EndFlag::LineFeed});
    solve();
    out.print();
}

// cargo test -- --nocapture
#[test]
fn test() {
    out.init(EndFlag::Print);
    out.print();
}
