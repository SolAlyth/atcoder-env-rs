[ABC328-F Good Set Query](https://atcoder.jp/contests/abc328/tasks/abc328_f)

ポテンシャルを持って Union Find する。

```Rust
fn solve() {
    use ds::unionfind::*;
    #[derive(Clone)] struct Op;
    impl UnionFindOp for Op {
        type T = i64;
        fn e() -> Self::T { 0 }
        fn add(a: &Self::T, b: &Self::T) -> Self::T { a+b }
        fn inv(a: &Self::T) -> Self::T { -a }
    }
    
    input! { N: usize, Q: [(usize1, usize1, i64)] }
    let mut uf = UnionFind::<Op>::new(N);
    let mut ans = vec![];
    for (i, (a, b, d)) in Q.into_iter().enumerate() {
        if uf.merge(a, b, d).is_some() { ans.push(i+1); }
    }
    out << &ans[..];
}
```
