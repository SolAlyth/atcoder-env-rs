use colored::Colorize;

pub fn color_eprint(s: impl Into<String>, c: &str) {
    std::eprint!("{}", s.into().color(c));
}

pub fn color_print(s: impl Into<String>, c: &str) {
    std::print!("{}", s.into().color(c));
}

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        for s in format!($($args)*).split('\n') {
            $crate::print::color_eprint(">> ", "green");
            $crate::print::color_print(s.to_string()+"\n", "green");
        }
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($args:tt)*) => {
        {
            $crate::print::color_eprint(
                format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s|acc+&s).unwrap(),
                "red"
            );
        }
    }
}
