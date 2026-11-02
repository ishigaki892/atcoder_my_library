use ac_library::{Max, Segtree};
use std::collections::HashMap;

// ------------------------------------------------------------
// その他関数系
// ------------------------------------------------------------

/// 回文判定
pub fn is_palindrome(s: &[char]) -> bool {
    s.iter().eq(s.iter().rev())
}

/// 座標圧縮
pub fn zaatu(v: &Vec<usize>) -> Vec<usize> {
    let mut vals = v.clone();
    vals.sort();
    vals.dedup();
    let compressed: Vec<usize> = v.iter().map(|&x| vals.binary_search(&x).unwrap()).collect();
    compressed
}

/// 汎用二分探索（めぐる式）
/// f(ok) == true, f(ng) == false となる境界を求める
/// ok, ng の初期値は「答えの取りうる範囲の外側」を含めて渡すこと
/// 例: 「x以上で条件を満たす最小値」を探すなら
///     binary_search(範囲の右端+1, 範囲の左端-1, |x| f(x)) のように呼ぶ
pub fn binary_search<F: Fn(isize) -> bool>(mut ok: isize, mut ng: isize, f: F) -> isize {
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

/// 累積和
pub fn prefix_sum(a: &[i64]) -> Vec<i64> {
    let mut s = vec![0; a.len() + 1];
    for i in 0..a.len() {
        s[i + 1] = s[i] + a[i];
    }
    s
}

/// 2次元累積和
pub fn grid_ruiseki(a: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let h = a.len();
    let w = a[0].len();
    let mut s = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] = s[i][j + 1] + s[i + 1][j] - s[i][j] + a[i][j];
        }
    }
    s
}
// クエリ (r1,c1)-(r2,c2)の和
// s[r2][c2] - s[r1][c2] - s[r2][c1] + s[r1][c1]

/// ランレングス圧縮
pub fn rle<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)> {
    if items.is_empty() {
        return vec![];
    }

    let mut res = Vec::new();
    let mut current_val = items[0].clone();
    let mut count = 0;

    for item in items {
        if *item == current_val {
            count += 1;
        } else {
            res.push((current_val, count));
            current_val = item.clone();
            count = 1;
        }
    }
    res.push((current_val, count));
    res
}

/// [l, r) のうち値が a 以上である最小のインデックス（無ければ None）
pub fn find_first_ge(seg: &Segtree<Max<i64>>, l: usize, r: usize, a: i64) -> Option<usize> {
    let v = seg.max_right(l, |&v| v < a);
    if v < r {
        Some(v)
    } else {
        None
    }
}

/// LIS（狭義単調増加）
/// ## 出力
/// `dp[k]` - 長さ k+1 の増加部分列の末尾として取りうる最小値。LISの長さは `dp.len()`
/// （dp自体はLISそのものではないので、復元したい場合は lis_restore を使う）
pub fn lis(sequence: &Vec<i64>) -> Vec<i64> {
    let mut dp = Vec::new();
    for &x in sequence {
        match dp.binary_search(&x) {
            Ok(_) => continue,
            Err(pos) => {
                if pos == dp.len() {
                    dp.push(x);
                } else {
                    dp[pos] = x;
                }
            }
        }
    }
    dp
}

/// LIS（実際の復元付き）O(n log n)
/// 狭義単調増加（strictly increasing）のLISを1つ復元して返す
/// 広義単調増加（non-decreasing）にしたい場合は upper_bound を使うよう変更すること
pub fn lis_restore(sequence: &[i64]) -> Vec<i64> {
    let n = sequence.len();
    if n == 0 {
        return vec![];
    }
    // dp[k] = 長さ k+1 のLISの末尾として達成できる最小値、その時のsequence上のインデックス
    let mut dp_val: Vec<i64> = Vec::new();
    let mut dp_idx: Vec<usize> = Vec::new();
    // prev[i] = sequence[i] をLISの末尾に採用したとき、その直前の要素のインデックス（無ければ usize::MAX）
    let mut prev = vec![usize::MAX; n];

    for i in 0..n {
        let x = sequence[i];
        let pos = dp_val.binary_search(&x).unwrap_or_else(|e| e);
        if pos > 0 {
            prev[i] = dp_idx[pos - 1];
        }
        if pos == dp_val.len() {
            dp_val.push(x);
            dp_idx.push(i);
        } else {
            dp_val[pos] = x;
            dp_idx[pos] = i;
        }
    }

    // 復元
    let mut res = Vec::with_capacity(dp_idx.len());
    let mut cur = *dp_idx.last().unwrap();
    loop {
        res.push(sequence[cur]);
        if prev[cur] == usize::MAX {
            break;
        }
        cur = prev[cur];
    }
    res.reverse();
    res
}

/// 等差数列となる部分列（連続でなくてよい）の長さの最大値を求める
/// ### 計算量 O(N^2)
pub fn get_tousa(a: &[i64]) -> usize {
    let n = a.len();
    if n <= 2 {
        return n;
    }

    let mut dp: Vec<HashMap<i64, usize>> = vec![HashMap::new(); n];
    let mut max_len = 2;

    for i in 0..n {
        for j in 0..i {
            let diff = a[i] - a[j];
            let len = *dp[j].get(&diff).unwrap_or(&1) + 1;
            dp[i].insert(diff, len);
            max_len = max_len.max(len);
        }
    }
    max_len
}
