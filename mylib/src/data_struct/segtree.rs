//! 汎用セグメント木

pub use crate::mylib::traits::abstracts::{Map, Monoid};
use std::marker::PhantomData;

pub mod affine;



use std::ops::{Bound, RangeBounds};
use std::{fmt::Debug, mem::replace, ops::{Deref, DerefMut, Index, IndexMut}, slice::SliceIndex};

/// `len <= 2^n` を満たす最小の正整数 `n` を返す。
fn ceil_log(n: usize) -> u32 { (n.max(2)-1).ilog2() + 1 }

/// 汎用セグメント木
/// 
/// 内部は長さ `2*len` の `tree`, `map_tree` で構成されている。
/// 
/// 「`map_tree` には対応する場所にまだ作用していない写像が入り、その値を見たいときに作用・伝搬させる。」という方法を取っている。
/// 
/// [`Monoid`], [`Map`] も参照。
/// 
/// # Usage
/// 
/// ## 作成
/// 
/// 1. 載せたい演算の [`Monoid`], [`Map`] を作る。  
/// 2. [`Segtree::new`], [`Segtree::new_no_monoid`], [`Segtree::new_no_map`] から長さを指定して作成。
/// 
/// このとき、最下層の長さは、指定した値より大きい 2 冪の値となる。実際の長さは [`Segtree::len`] から取得できる。
/// 
/// ## 一点更新
/// 
/// [`Segtree::entry`] から [`Entry`] を取得できる。
/// 
/// + [`Entry::deref`], [`Entry::deref_mut`] から値を取得・変更できる。
/// + [`Entry::set`], [`Entry::modify`] で値を操作できる。
/// + [`Entry::map`] で載っている写像を一点作用できる。(これいる？)
/// 
/// Drop 時に木の更新が必要なら行う。
/// 
/// ## 全体更新
/// 
/// [`Segtree::entry_all`] から [`EntryAll`] を取得できる。
/// 
/// + [`EntryAll::index`], [`EntryAll::index_mut`] から取得・操作ができる。
/// 
/// Drop 時に木の更新が必要なら行う。
/// 
/// ## 範囲作用
/// 
/// [`Segtree::map`] から範囲作用ができる。
/// 
/// # Memo
/// 
/// `x >> i`: `x` の `i` 個上のセル。
/// 
/// `(x >> i) << i`: `x` の下 `i` bit を `0` にしたもの。`i` 個上のセルの左端と一致するなら、この値は `x` と等しい。
/// 
/// # Reference
/// 
/// [maspy さんの記事](https://maspypy.com/segment-tree-のお勉強2): 図付きでとても分かりやすい。
pub struct Segtree<Mn: Monoid, Mp: Map<Mn::Set>> {
    tree: Vec<Mn::Set>,
    /// 対応する場所にまだ作用していない写像。
    map_tree: Vec<Mp::F>,
    /// `log2(size)` であり、層の数と等しい。例えば `size = 8` のとき `tree = [1,2,4]`, `log = 3`
    log: u32,
}

/// 木全体のエントリー。`Map` は全て作用された状態であることが保証される。
/// 
/// `Drop` するとき、`Monoid != Nop && changed == true` ならば `tree` を再計算する。`O(N)`
pub struct EntryAll<'a, Mn: Monoid, Mp: Map<Mn::Set>> { seg: &'a mut Segtree<Mn, Mp>, changed: bool }

/// 特定のインデックスのエントリー。`tree[i]` に関わる `Map` は作用された状態であることが保証される。
/// 
/// `Drop` するとき、`Monoid != Nop && changed == true` ならば、`tree` を再計算する。`O(logN)`
pub struct Entry<'a, Mn: Monoid, Mp: Map<Mn::Set>> { seg: &'a mut Segtree<Mn, Mp>, i: usize, changed: bool }



impl Segtree<Nop<()>, Nop<()>> {
    /// `[Monoid::e]` で初期化されたセグメント木を作成する。
    pub fn new_no_map<Mn: Monoid>(len: usize) -> Segtree<Mn, Nop<Mn::Set>> {
        let log = ceil_log(len) + 1;
        let size = 1 << log;
        Segtree { tree: vec![Mn::e(); size], map_tree: vec![(); size], log }
    }
    
    /// `[init; len+α]` で初期化された双対セグメント木を作成する。
    pub fn new_no_monoid<T: Clone + Debug, Mp: Map<T>>(init: T, len: usize) -> Segtree<Nop<T>, Mp> {
        let log = ceil_log(len) + 1;
        let size = 1 << log;
        Segtree { tree: vec![init; size], map_tree: vec![Mp::id(); size], log }
    }
    
    /// `[Monoid::e; len+α]` で初期化された遅延セグメント木を構成する。
    pub fn new<Mn: Monoid, Mp: Map<Mn::Set>>(len: usize) -> Segtree<Mn, Mp> {
        let log = ceil_log(len) + 1;
        let size = 1 << log;
        Segtree { tree: vec![Mn::e(); size], map_tree: vec![Mp::id(); size], log }
    }
}



impl<Mn: Monoid, Mp: Map<Mn::Set>> Segtree<Mn, Mp> {
    /// `Monoid` の型のヒント
    pub fn hint_monoid(&self, _: Mn) {}
    /// `Map` の型のヒント
    pub fn hint_map(&self, _: Mp) {}
    
    /// [`RangeBounds`] をいい感じに `[l, r)` に解釈する。
    /// 
    /// # Panic
    /// 
    /// `range` が `[0, len)` に含まれないとき。
    fn interpret(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let l = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v+1,
            Bound::Unbounded => 0
        };
        let r = match range.end_bound() {
            Bound::Included(v) => v+1,
            Bound::Excluded(v) => *v,
            Bound::Unbounded => self.len()
        };
        
        assert!(l <= r && r <= self.len(), "specified: [{l}, {r}), valid: [0, {})", self.len());
        
        (l+self.len(), r+self.len())
    }
    
    /// セグメント木の最下層の長さを返す。
    pub fn len(&self) -> usize { self.tree.len()/2 }
    
    /// `Map != Nop` 限定。`map_tree[i]` を作用させ、子に伝搬させる。
    fn act(&mut self, i: usize) {
        let f = replace(&mut self.map_tree[i], Mp::id());
        let is_leaf = self.map_tree.len() <= 2*i;
        
        if !is_leaf {
            Mp::comp(&f, &mut self.map_tree[2*i]);
            Mp::comp(&f, &mut self.map_tree[2*i+1]);
        }
        
        if !Mn::NOP || (Mn::NOP && is_leaf) {
            Mp::map(&f, &mut self.tree[i]);
        }
    }
    
    /// `l, r` の真上のセルを上から順に [`Segtree::act`] する。
    /// 
    /// このとき、`[l, r)` 内を適切なセル列に分割したとき、それらを [`Segtree::act`] すれば実際の値が得られる。
    fn act_range(&mut self, l: usize, r: usize) {
        for i in (1..self.log).rev() {
            // (x >> i) << i で、x の真上のセルを取得できる。
            if (l >> i) << i != l { self.act(l >> i); }
            if (r >> i) << i != r { self.act(r >> i); }
        }
    }
    
    /// `Monoid != Nop` 限定。`tree[2i], tree[2i+1]` から `tree[i]` を再計算する。
    fn calc(&mut self, i: usize) {
        self.tree[i] = Mn::op(&self.tree[2*i], &self.tree[2*i+1]);
    }
    
    /// `Monoid != Nop` 限定。`A[l..r]` を計算する。
    /// 
    /// # Panic
    /// 
    /// `Monoid == Nop` のとき。
    /// 
    /// `range` が `[0, len)` に含まれないとき。
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> Mn::Set {
        assert!(!Mn::NOP);
        
        let (mut l, mut r) = self.interpret(range);
        
        if !Mp::NOP { self.act_range(l, r); }
        
        let (mut outl, mut outr) = (Mn::e(), Mn::e());
        
        // 計算する必要のあるセルを、act してから op する。
        while l < r {
            if l&1 == 1 {
                if !Mp::NOP { self.act(l); }
                outl = Mn::op(&outl, &self.tree[l]); l += 1;
            }
            if r&1 == 1 {
                if !Mp::NOP { self.act(r-1); }
                outr = Mn::op(&self.tree[r-1], &outr); r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        
        Mn::op(&outl, &outr)
    }
    
    /// `Map != Nop` 限定。`map` を `A[range]` に作用させる。`O(logN)`
    /// 
    /// # Panic
    /// 
    /// `Map == Nop` のとき。
    /// 
    /// `range` が `[0, len)` に含まれないとき。
    pub fn map(&mut self, range: impl RangeBounds<usize>, map: Mp::F) {
        assert!(!Mp::NOP);
        
        let (l, r) = self.interpret(range);
        
        self.act_range(l, r);
        
        {
            let (mut l, mut r) = (l, r);
            // 一個外側のセルも act する必要があるため、初めてガタついたら外側も act する。
            let (mut lf, mut rf) = (true, true);
            // 作用する範囲に合成していく。
            while l < r {
                if l&1 != 0 {
                    Mp::comp(&map, &mut self.map_tree[l]); self.act(l);
                    if lf { self.act(l-1); lf = false; }
                    l += 1;
                }
                if r&1 != 0 {
                    Mp::comp(&map, &mut self.map_tree[r-1]); self.act(r-1);
                    if rf { self.act(r); rf = false; }
                    r -= 1;
                }
                l >>= 1; r >>= 1;
            }
        }
        
        if !Mn::NOP {
            for i in 1..self.log {
                if (l >> i) << i != l { self.calc(l >> i); }
                if (r >> i) << i != r { self.calc(r >> i); }
            }
        }
    }
    
    /// `pred(A[l..r]) == true` かつ `r == len || pred(A[l..r+1]) == false` となる `r` を一つ返す。
    /// 
    /// # Requirements
    /// 
    /// `pred(Monoid::e) == true` が成り立つ。
    /// 
    /// # Panic
    /// 
    /// `l <= len` でない。
    pub fn max_right(&mut self, mut l: usize, pred: impl Fn(&Mn::Set) -> bool) -> usize {
        assert!(l <= self.len());
        assert!(pred(&Mn::e()));
        
        if l == self.len() { return self.len(); }
        
        l += self.len();
        if !Mp::NOP { for i in (1..self.log).rev() { self.act(l >> i); } }
        
        let mut res = Mn::e();
        
        loop {
            while l&1 == 0 { l >>= 1; }
            if !Mp::NOP { self.act(l); }
            
            let tmp = Mn::op(&res, &self.tree[l]);
            if !pred(&tmp) {
                while l < self.len() {
                    l <<= 1;
                    if !Mp::NOP { self.act(l); }
                    let tmp = Mn::op(&res, &self.tree[l]);
                    if pred(&tmp) { res = tmp; l += 1; }
                }
                
                break l - self.len();
            }
            
            res = tmp;
            l += 1;
            
            let l = l as isize;
            if (l & -l) != l { break self.len(); }
        }
    }
    
    /// テストしてない！
    /// 
    /// `pred(A[l..r]) == true` かつ `r == len || pred(A[l..r+1]) == false` となる `r` を一つ返す。
    /// 
    /// # Requirements
    /// 
    /// `pred(Monoid::e) == true` が成り立つ。
    /// 
    /// # Panic
    /// 
    /// `r <= len` でない。
    pub fn max_left(&mut self, mut r: usize, pred: impl Fn(&Mn::Set) -> bool) -> usize {
        assert!(r <= self.len());
        assert!(pred(&Mn::e()));
        
        if r == 0 { return 0; }
        
        r += self.len();
        if !Mp::NOP { for i in (1..self.log).rev() { self.act((r-1) >> i); } }
        
        let mut res = Mn::e();
        
        loop {
            r -= 1;
            while 1 < r && r&1 == 0 { r >>= 1; }
            if !Mp::NOP { self.act(r); }
            let tmp = Mn::op(&self.tree[r], &res);
            if !pred(&tmp) {
                while r < self.len() {
                    r = 2*r + 1;
                    if !Mp::NOP { self.act(r); }
                    let tmp = Mn::op(&self.tree[r], &res);
                    if pred(&tmp) { res = tmp; r -= 1; }
                }
                
                return r + 1 - self.len();
            }
            
            res = tmp;
            
            let r = r as isize;
            if (r & -r) != r { return 0; }
        }
    }
    
    
    
    /// `A[idx]` の値を計算したのち、[`Entry`] を返す。Map: `O(logN)` / No Map: `O(1)` if no change
    pub fn entry<'a>(&'a mut self, idx: usize) -> Entry<'a, Mn, Mp> {
        assert!(idx < self.len());
        
        let i = idx + self.len();
        if !Mp::NOP { for j in (0..self.log).rev() { self.act(i >> j); } }
        Entry { seg: self, i, changed: false }
    }
    
    /// `A[..]` の値を計算したのち、[`EntryAll`] を返す。Map: `O(N)` / No Map: `O(1)` if no change
    pub fn entry_all<'a>(&'a mut self) -> EntryAll<'a, Mn, Mp> {
        if !Mp::NOP { for i in 1..self.map_tree.len() { self.act(i); } }
        EntryAll { seg: self, changed: false }
    }
}

impl<Mn: Monoid, Mp: Map<Mn::Set>> Debug for Segtree<Mn, Mp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = vec![String::new(); self.tree.len()];
        for i in (if !Mn::NOP {0} else {self.len()})..self.tree.len() {
            a[i] += &format!("[{}]", Mn::debug(&self.tree[i]));
        }
        if !Mp::NOP {
            for i in 0..self.map_tree.len() {
                if !a[i].is_empty() { a[i] += " "; }
                a[i] += &format!("({})", &Mp::debug(&self.map_tree[i]));
            }
        }
        
        let max_len = a.iter().map(|s| s.len()).max().unwrap();
        let mut out = String::new();
        for i in 0..self.log {
            out += "\n";
            for j in (1 << i)..(1 << i+1) {
                out += &format!("{: <width$}", a[j], width = max_len*(1 << self.log-i-1));
            }
        }
        
        write!(f, "{}", &out[1..])
    }
}



impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Drop for EntryAll<'a, Mn, Mp> {
    fn drop(&mut self) {
        if self.changed && !Mn::NOP { for i in (1..self.seg.tree.len()/2).rev() { self.seg.calc(i); } }
    }
}

impl<'a, Mn: Monoid, Mp: Map<Mn::Set>, I: SliceIndex<[Mn::Set]>> Index<I> for EntryAll<'a, Mn, Mp> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output { Index::index(&self.seg.tree[self.seg.len()..], index) }
}

impl<'a, Mn: Monoid, Mp: Map<Mn::Set>, I: SliceIndex<[Mn::Set]>> IndexMut<I> for EntryAll<'a, Mn, Mp> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output { self.changed = true; let len = self.seg.len(); IndexMut::index_mut(&mut self.seg.tree[len..], index) }
}



impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Entry<'a, Mn, Mp> {
    pub fn map(&mut self, map: &Mp::F) { self.changed = true; Mp::map(map, &mut self.seg.tree[self.i]); }
    pub fn set(&mut self, value: Mn::Set) -> Mn::Set { self.changed = true; replace(&mut self.seg.tree[self.i], value) }
    pub fn modify<T>(&mut self, f: impl FnOnce(&mut Mn::Set) -> T) -> T { self.changed = true; f(&mut self.seg.tree[self.i]) }
}

impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Drop for Entry<'a, Mn, Mp> {
    fn drop(&mut self) { if self.changed && !Mn::NOP { for j in 1..self.seg.log { self.seg.calc(self.i >> j); } } }
}

impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> Deref for Entry<'a, Mn, Mp> {
    type Target = Mn::Set;
    fn deref(&self) -> &Self::Target { &self.seg.tree[self.i] }
}

impl<'a, Mn: Monoid, Mp: Map<Mn::Set>> DerefMut for Entry<'a, Mn, Mp> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed = true; &mut self.seg.tree[self.i]
    }
}



/// No-op な `Monoid` & `Map`
/// 
/// # Panic
/// 
/// 不必要な実装はだいたい panic している。
/// 
/// **[`Segtree::new`] で [`Monoid`] として使うと [`Nop::e`] の関係で panic する。**  
/// 直接指定せず、代わりに [`Segtree::new_no_monoid`] や [`Segtree::new_no_map`] を使用することを推奨。
pub struct Nop<T: Clone> (PhantomData<T>);

impl<T: Clone + Debug> Monoid for Nop<T> {
    const NOP: bool = true;
    type Set = T;
    fn e() -> T { panic!() }
    fn op(_: &T, _: &T) -> T { panic!() }
    fn debug(v: &Self::Set) -> String { format!("{v:?}") }
}


impl<T: Clone> Map<T> for Nop<T> {
    const NOP: bool = true;
    type F = ();
    fn id() -> () { panic!() }
    fn comp(_: &(), _: &mut ()) { panic!() }
    fn map(_: &(), _: &mut T) { panic!() }
    fn debug(_: &Self::F) -> String { "".into() }
}
