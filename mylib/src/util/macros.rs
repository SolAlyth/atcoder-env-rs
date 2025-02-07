#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION { eprint!("\x1b[31m"); print!("{}", format!($($args)*).split('\n').map(|s| format!(">> {s}\n")).reduce(|acc,s| acc+&s).unwrap()); eprint!("\x1b[0m"); }
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



// Float は Ord が使えないので reduce している

#[macro_export]
macro_rules! min {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x <= y {x} else {y}).unwrap() }
}

#[macro_export]
macro_rules! max {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x >= y {x} else {y}).unwrap() }
}

/// `values < dst` であるとき `true` を返す。
#[macro_export]
macro_rules! chmin {
    ($dst:expr; $v:expr) => { { let v = $v; if v < $dst { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmin!($dst; crate::min!($($vl),+)) }
}

/// `dst < values` であるとき `true` を返す。
#[macro_export]
macro_rules! chmax {
    ($dst:expr; $v:expr) => { { let v = $v; if $dst < v { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmax!($dst; crate::max!($($vl),+)) }
}




/* macro_rules! impl_for {
    ($trait:ty; $($type:ty),+) => { $( impl $trait for $type {} )+ }
}
pub(crate) use impl_for; */
