#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        {
            let s = format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc, s| acc+&s).unwrap();
            { use colored::Colorize; std::print!("{}", s.green()); }
        }
    }
}

#[macro_export]
macro_rules! eprintln {
    ($($args:tt)*) => {
        {
            let s = format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc, s| acc+&s).unwrap();
            { use colored::Colorize; std::print!("{}", s.red()); }
        }
    }
}
