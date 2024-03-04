#![allow(non_snake_case)]

use mylib::*;

const INTERACTIVE: bool = false;

#[allow(unused_must_use)]
fn solve<_T: BufRead>(#[allow(unused)] out: &Printer, mut stdin: impl Source<_T>) {
    macro_rules! input {($($t:tt)*)=>{mylib::input!(from &mut stdin, $($t)*);}}
    input! {
        /* $1 */
    }
    
    /* $0 */
}



fn _input() -> String { String::from("") }


fn main() {
    let out = Printer::new(if SUBMISSION && !INTERACTIVE {EndFlag::LineFeed} else {EndFlag::Print});
    
    if !SUBMISSION {
        let inp = _input();
        if !inp.is_empty() {
            solve(&out, OnceSource::from(inp.as_str()));
        } else {
            solve(&out, LineSource::new(BufReader::new(stdin())));
        }
    } else {
        if !INTERACTIVE {
            solve(&out, OnceSource::new(BufReader::new(stdin())));
        } else {
            solve(&out, LineSource::new(BufReader::new(stdin())));
        }
    }
    out.print();
}
