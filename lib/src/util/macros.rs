#[macro_export]
macro_rules! nest {
    (void; $n:expr) => { vec![vec![];$n] };
    (void; $n:expr $(;$m:expr)+) => { vec![nest![void$(;$m)+]; $n] };
    
    ($e:expr; $n:expr) => { vec![$e; $n] };
    ($e:expr; $n:expr $(;$m:expr)+) => { vec![nest![$e$(;$m)+]; $n] };
}
