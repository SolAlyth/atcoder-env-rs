My environment for [AtCoder](https://atcoder.jp/) by [Rust](https://www.rust-lang.org/)

# lib で出来ること

## solver.rs

`Solver, end` を提供します。

```rust
fn main() {
    let s = Solver::new(false);
    solve(&out);
    out.print();
}

#[allow(unused_must_use)]
fn solve(out: &Solver) {
    // ...will print "1 2 3 string"
    out << 1 << 2 << 3 << "string" << end;
}
```

Release 時は一度に出力します。


## nest.rs

```rust
fn main() {
    // ... will vec[0, 0]
    let vec_1d: Vec<i32> = nest!(0; 2);
    
    // ... will vec[vec[0, 0, 0], vec[0, 0, 0]]
    let vec_2d: Vec<Vec<i32>> = nest!(0; 2; 3);
    
    // ... will vec[vec[], vec[]]
    let vec_void_1d: Vec<Vec<_>> = nest!(void; 2);
    
    let vec_void_2d: Vec<Vec<Vec<_>>> = nest!(void; 2; 3);
    
    
    // example: 有向グラフの入力
    
    input! { N: usize, Einp: [(Usize1, Usize1)] }
    
    let mut E = nest!(void; N);
    
    for (u, v) in Einp { E[u].push(v); }
}
```


## print.rs

`println, eprintln` に彩りを添えてくれます。
