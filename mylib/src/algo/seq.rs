//! 列関連のアルゴリズム
//! 
//! メモ: `Ord` が大きい `char` として `'|', '~'` がある。

use ac_library::string::{z_algorithm_arbitrary, suffix_array_arbitrary, lcp_array_arbitrary};

/// 最長共通接頭辞の長さの列を返す。`O(n)`
/// 
/// `res[i] = k` として `v[0..k] == v[i..i+k]` となる。
/// 
/// + 文字列 `S, T` の最長共通接頭辞の長さ `{ z = Z("S~T"); z[s+1] }`
pub fn z_algorithm<T: Ord>(v: &[T]) -> Vec<usize> { z_algorithm_arbitrary(v) }

/// Suffix Array, LCP Array を用いたパターン検索
/// 
/// # Suffix Array
/// 
/// `v[i..]` をソートしたら何番目か、の列。`O(n)`
/// 
/// # LCP(Longest Common Prefix) Array
/// 
/// 隣り合う Suffix 列の最長共通接頭辞の長さの列。`O(n)`  
/// 定式化するなら `lcp[i] = len of LCP(v[sa[i]..], v[sa[i+1]..])`
/// 
/// `len of LCP(v[sa[i]..], v[sa[j]..]) = min(lcp[i..j])` という性質を持つ。
pub struct SuffixLcp<T: Ord> {
    s: Vec<T>,
    suffix: Vec<usize>,
    lcp: Vec<usize>
}

impl<T: Ord> SuffixLcp<T> {
    pub fn new(v: Vec<T>) -> Self {
        let suffix = suffix_array_arbitrary(&v);
        let lcp = lcp_array_arbitrary(&v, &suffix);
        Self { s: v, suffix, lcp }
    }
    
    pub fn inner(&self) -> (&[usize], &[usize]) { (&self.suffix, &self.lcp) }
    
    pub fn lower_bound(&self, v: &[T]) -> usize {
        // todo: https://echizen-tm.hatenadiary.org/entry/20110728/1311871765
        let (mut ng, mut ok) = (usize::MAX, self.s.len()-1);
        while let Some(mid) = crate::mylib::binary_search(ng, ok) {
            *(if &self.s[mid..] < v { &mut ok } else { &mut ng }) = mid;
        }
        ok
    }
}
