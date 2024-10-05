#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION { eprint!("\x1b[31m"); print!("{}", format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s| acc+&s).unwrap()); eprint!("\x1b[0m"); }
        if $crate::SUBMISSION { /* do nothing */ }
    }
}

/// `nest!(void; 2; 3) ... vec[0..2][0..3]: [[vec![]; 3]; 2]`  
/// `nest!(init; 2; 3) ... vec[0..2][0..3]: [[init; 3]; 2]`
/// 
/// `!Clone` な要素を入れるときは `void` は出来ない
#[macro_export]
macro_rules! nest {
    (void; $n:expr) => { vec![vec![]; $n] };
    (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] };
    
    () => { vec![] };
    ($e:expr; $n:expr) => { vec![$e; $n] };
    ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] };
}

#[macro_export]
macro_rules! min {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x<y {x} else {y}).unwrap() }
}

#[macro_export]
macro_rules! max {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x>y {x} else {y}).unwrap() }
}

#[macro_export]
macro_rules! chmin {
    ($dst:expr; $($vl:expr),+) => { { let v = crate::min!($($vl),+); if v < $dst { $dst = v; true } else { false } } };
}

#[macro_export]
macro_rules! chmax {
    ($dst:expr; $($vl:expr),+) => { { let v = crate::max!($($vl),+); if $dst < v { $dst = v; true } else { false } } };
}



/// `elsedef!(cond; value)` の形で使う。
/// `cond == true` のとき `value` を返し、そうでないとき `Default::default()` を返す。
#[macro_export]
macro_rules! elsedef {
    ($cond:expr; $v:expr) => { if $cond {$v} else {Default::default()} }
}




/* macro_rules! impl_for {
    ($trait:ty; $($type:ty),+) => { $( impl $trait for $type {} )+ }
}
pub(crate) use impl_for; */
