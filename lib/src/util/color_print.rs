//! 便利なもの

use colored::Colorize;

pub fn color_print(s: impl Into<String>, color: &str) {
    std::print!("{}", s.into().color(color));
}

pub fn color_eprint(s: impl Into<String>, color: &str) {
    std::eprint!("{}", s.into().color(color));
}

#[macro_export]
macro_rules! pr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION {
            for s in format!($($args)*).split('\n') {
                // for output
                $crate::util::color_print::color_eprint(">> ", "green");
                $crate::util::color_print::color_print(s.to_string()+"\n", "green");
            }
        }
        
        if $crate::SUBMISSION {
            println!($($args)*);
        }
    }
}

#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION {
            $crate::util::color_print::color_eprint(
                format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s|acc+&s).unwrap(),
                "red"
            );
        }
        
        if $crate::SUBMISSION {
            // do nothing
        }
    }
}
