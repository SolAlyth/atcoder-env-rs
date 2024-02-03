#[macro_export]
macro_rules! nest {
    (void; $n:expr) => { vec![vec![];$n] };
    (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] };
    
    ($e:expr; $n:expr) => { vec![$e; $n] };
    ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] };
}


/// 引数の最小値を返します。
///
/// # Requirements
/// 引数は `PartialOrd` を実装した、同じ型である必要があります。
#[macro_export]
macro_rules! min {
    ($l:expr, $r:expr) => { if $l.lt(&$r) { $l } else { $r } };
    ($v:expr, $($vl:expr),+) => { min!($v, min!($($vl),+)) };
}

/// 引数の最大値を返します。
///
/// 引数は `PartialOrd` を実装した、同じ型である必要があります。
#[macro_export]
macro_rules! max {
    ($l:expr, $r:expr) => { if $l.gt(&$r) { $l } else { $r } };
    ($v:expr, $($vl:expr),+) => { max!($v, max!($($vl),+)) };
}

#[cfg(test)]
mod test {
    #[allow(unused)] use super::*;
    
    #[test]
    fn minmax() {
        assert_eq!(min!(0.0, 1.0, 2.0, f64::INFINITY), 0.0);
        assert_eq!(max!(0.0, 1.0, 2.0, f64::INFINITY), f64::INFINITY);
    }
}
