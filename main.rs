// ============================================================
// AtCoder Rust Template
// version5
// made by ishigaki122 & Claude AI
// この提出コードは自作IDEを用いています
// templates: https://github.com/ishigaki892/atcoder_my_template
// IDE: https://github.com/ishigaki892/acd_studio
// ============================================================

#![allow(unused_imports, unused_macros, dead_code, non_upper_case_globals)]
use ac_library::modint::{ModInt1000000007, ModInt998244353};
use ac_library::{scc, Dsu, FenwickTree};
use ac_library::{Additive, LazySegtree, MapMonoid, Max, Min, Monoid, Segtree};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::vec;
use superslice::Ext;

// ------------------------------------------------------------
// 定数
// ------------------------------------------------------------

const MOD: u64 = 998_244_353;
const MOD7: u64 = 1_000_000_007;
const INF: i64 = 1 << 60;
const EPS: f64 = 1e-9;
const DX: [isize; 8] = [-1, 0, 1, 0, 1, 1, -1, -1];
const DY: [isize; 8] = [0, 1, 0, -1, 1, -1, 1, -1];
const ABC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER_ABC: &str = "abcdefghijklmnopqrstuvwxyz";

// ------------------------------------------------------------
// 便利マクロ
// ------------------------------------------------------------

/// chmin!(a, b) : a = min(a, b) を行い、更新されたら true を返す
macro_rules! chmin {
    ($a:expr, $b:expr) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// chmax!(a, b) : a = max(a, b) を行い、更新されたら true を返す
macro_rules! chmax {
    ($a:expr, $b:expr) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// デバッグ出力
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

/// Yes or No
macro_rules! yn {
    ($cond:expr) => {
        println!("{}", if $cond { "Yes" } else { "No" });
    };
}

/// YES or NO
macro_rules! YN {
    ($cond:expr) => {
        println!("{}", if $cond { "YES" } else { "NO" });
    };
}

// ------------------------------------------------------------
// メイン処理
// ------------------------------------------------------------

// fastout は「付けた関数の中の println!」だけをバッファ化するので、出力する solve 側に付ける
#[proconio::fastout]
fn solve() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!("{:?}", a);
}

fn main() {
    input! {}
    solve();
    // 深い再帰DFSでスタックが足りない場合は上の行の代わりにこちらを使う
    // std::thread::Builder::new().stack_size(1 << 28).spawn(solve).unwrap().join().unwrap();
}

// ------------------------------------------------------------
// ModInt 使用例（ac_library）
// ------------------------------------------------------------
// type Mint = ModInt998244353; // or ModInt1000000007
//
// let mut x = Mint::new(3);
// let y = Mint::new(4);
// let z = x + y;             // 通常の四則演算子がそのままMOD付きで使える
// let p = x.pow(10);         // 累乗
// let inv = x.inv();         // 逆元
// println!("{}", z);         // Displayで自動的に0..MOD範囲の値を表示

// ------------------------------------------------------------
// 数論系
// ------------------------------------------------------------

/// 最大公約数
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// 最小公倍数
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a / gcd(a, b)) * b
}

/// 組み合わせ（O(r)、1回きりのクエリ向け。大量にnCrを問われる問題ではComb構造体を使うこと）
fn combination(n: u64, r: u64, md: u64) -> u64 {
    if r > n {
        return 0;
    }
    let mut num = 1u64;
    let mut den = 1u64;
    for i in 0..r {
        num = num * ((n - i) % md) % md;
        den = den * ((i + 1) % md) % md;
    }
    num * power(den, md - 2, md) % md
}

/// 階乗・逆元階乗を前計算し、O(1)でnCr, nPrを求める構造体
/// 1問の中で何度もnCrを問われる数え上げ系問題向け
/// let comb = Comb::new(200000, MOD);
/// comb.ncr(n, r)
struct Comb {
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
    modulo: u64,
}

impl Comb {
    fn new(max_n: usize, modulo: u64) -> Self {
        let mut fact = vec![1u64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i as u64 % modulo;
        }
        let mut inv_fact = vec![1u64; max_n + 1];
        inv_fact[max_n] = power(fact[max_n], modulo - 2, modulo);
        for i in (0..max_n).rev() {
            inv_fact[i] = inv_fact[i + 1] * (i as u64 + 1) % modulo;
        }
        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    fn ncr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n - r] % self.modulo
    }

    fn npr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        self.fact[n] * self.inv_fact[n - r] % self.modulo
    }
}

/// 素因数分解
fn factorize(mut n: u64) -> std::collections::HashMap<u64, u64> {
    let mut map = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        map.insert(n, 1);
    }
    map
}

/// 10進法をn進法に変換
fn to_n_base(mut n: u64, base: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut res = Vec::new();
    while n > 0 {
        res.push(n % base);
        n /= base;
    }
    res.reverse();
    res
}

/// n進法を10新法に変換
fn from_n_base(digits: &[u64], base: u64) -> u64 {
    let mut res = 0;
    for &d in digits {
        res = res * base + d;
    }
    res
}

/// 桁数カウント（0 は 1 桁）
fn cnt_keta(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut s = n;
    let mut ans = 0;
    while s > 0 {
        s /= 10;
        ans += 1;
    }
    ans
}

/// 桁和カウント
fn sum_keta(n: usize) -> usize {
    let mut s = n;
    let mut ans = 0;
    while s > 0 {
        ans += s % 10;
        s /= 10;
    }
    ans
}

/// 整数平方根 floor(sqrt(n))（usize全域で正しい）
fn isqrt(n: usize) -> usize {
    // f64 で概算してから u128 で誤差を補正する
    let mut x = (n as f64).sqrt() as u128;
    let n = n as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x as usize
}

/// 整数三乗根 floor(cbrt(n))（usize全域で正しい）
fn cbrt(n: usize) -> usize {
    let mut x = (n as f64).cbrt() as u128;
    let n = n as u128;
    while x * x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x as usize
}

/// 整数の床除算（a / b の床）
/// - 任意の符号の a, b に対して floor を返す（b != 0 を仮定）
fn div_floor(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q - 1
    } else {
        q
    }
}

/// 整数の天井除算（a / b の天井）
/// - 任意の符号の a, b に対して ceil を返す（b != 0 を仮定）
fn div_ceil(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) == (b > 0)) {
        q + 1
    } else {
        q
    }
}

/// usize 用の安全な天井除算（b > 0 を仮定）
fn ceil_div_usize(a: usize, b: usize) -> usize {
    assert!(b > 0);
    if a == 0 {
        0
    } else {
        (a - 1) / b + 1
    }
}

/// 約数の個数をカウント
fn count_divisors(mut n: u64) -> u64 {
    let mut res = 1;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                count += 1;
                n /= i;
            }
            res *= count + 1;
        }
        i += 1;
    }
    if n > 1 {
        res *= 2;
    }
    res
}

/// エラトステネスの篩
fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=n {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    is_prime
}

/// ユークリッドの互除法
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = ext_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (g, x, y)
    }
}

/// 逆元
fn mod_inv(a: u64, m: u64) -> u64 {
    let (g, x, _) = ext_gcd(a as i64, m as i64);
    if g != 1 {
        0
    } else {
        ((x % m as i64 + m as i64) % m as i64) as u64
    }
}

/// N!(Nの階乗)
/// usizeのまま計算するのでn>=21程度でオーバーフローする。MOD付きが必要ならComb構造体を使うこと
fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// 約数を全列挙
fn divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }
    divs.sort();
    divs
}

/// Miller-Rabin素数判定 (u64全域で決定的に正しい)
fn miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    let mul_mod = |a: u64, b: u64, m: u64| -> u64 { ((a as u128 * b as u128) % m as u128) as u64 };
    let pow_mod = |mut a: u64, mut e: u64, m: u64| -> u64 {
        let mut res = 1u64;
        a %= m;
        while e > 0 {
            if e & 1 == 1 {
                res = mul_mod(res, a, m);
            }
            a = mul_mod(a, a, m);
            e >>= 1;
        }
        res
    };

    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut ok = false;
        for _ in 0..r - 1 {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return false;
        }
    }
    true
}

/// Pollard's Rho による素因数分解 O(n^{1/4} polylog)
fn pollard_rho(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }
    if miller_rabin(n) {
        return vec![n];
    }
    let mul_mod = |a: u64, b: u64, m: u64| -> u64 { ((a as u128 * b as u128) % m as u128) as u64 };

    let mut c = 1u64;
    loop {
        let f = |x: u64| (mul_mod(x, x, n) + c) % n;
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(if x > y { x - y } else { y - x }, n);
        }
        if d != n {
            let mut res = pollard_rho(d);
            res.extend(pollard_rho(n / d));
            return res;
        }
        c += 1;
    }
}

/// floor_sum: sum_{i=0}^{n-1} floor((a*i+b)/m) をO(log(min(a,m)))で求める
/// AtCoder Library の floor_sum と同一のアルゴリズム
/// 制約: n >= 0, m >= 1 (a, b は負でもよい)
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    let mut ans: i64 = 0;
    let (mut n, mut m, mut a, mut b) = (n, m, a, b);
    if a < 0 {
        let a2 = a.rem_euclid(m);
        ans -= n * (n - 1) / 2 * ((a2 - a) / m);
        a = a2;
    }
    if b < 0 {
        let b2 = b.rem_euclid(m);
        ans -= n * ((b2 - b) / m);
        b = b2;
    }
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}

/// 中国剰余定理 (CRT)
/// x ≡ r1 (mod m1), x ≡ r2 (mod m2) を満たす x を法 lcm(m1, m2) の範囲で求める
/// ## 出力
/// `Some((x, lcm))` - 解が存在する場合。x は 0 <= x < lcm を満たす
/// `None` - 矛盾していて解が存在しない場合
fn crt(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
    let (g, p, _q) = ext_gcd(m1, m2);
    if (r2 - r1) % g != 0 {
        return None;
    }
    let lcm = m1 / g * m2;
    // x = r1 + m1 * ((r2 - r1) / g * p mod (m2 / g))
    let tmp = (r2 - r1) / g % (m2 / g) * p % (m2 / g);
    let mut x = r1 + m1 * tmp;
    x = x.rem_euclid(lcm);
    Some((x, lcm))
}

/// 複数の合同式 x ≡ rs[i] (mod ms[i]) をまとめて解く CRT
/// ## 出力
/// `Some((x, lcm))` - 0 <= x < lcm を満たす解
/// `None` - 矛盾がある場合
fn crt_multi(rs: &[i64], ms: &[i64]) -> Option<(i64, i64)> {
    let mut r = 0i64;
    let mut m = 1i64;
    for (&ri, &mi) in rs.iter().zip(ms.iter()) {
        match crt(r, m, ri, mi) {
            Some((nr, nm)) => {
                r = nr;
                m = nm;
            }
            None => return None,
        }
    }
    Some((r, m))
}

// ------------------------------------------------------------
// グリッド系
// ------------------------------------------------------------

/// グリッドの範囲内判定
fn in_grid(ny: isize, nx: isize, h: usize, w: usize) -> bool {
    ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize
}

/// グリッドを時計回りに90度回転する
/// 入力: N行M列 → 出力: M行N列
/// 変換式: new[j][N-1-i] = old[i][j]
fn rotate_cw<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![grid[0][0].clone(); n]; m];
    for i in 0..n {
        for j in 0..m {
            new_grid[j][n - 1 - i] = grid[i][j].clone();
        }
    }
    new_grid
}

/// グリッドを反時計回りに90度回転する
/// 変換式: new[M-1-j][i] = old[i][j]
fn rotate_ccw<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![grid[0][0].clone(); n]; m];
    for i in 0..n {
        for j in 0..m {
            new_grid[m - 1 - j][i] = grid[i][j].clone();
        }
    }
    new_grid
}

/// グリッドを180度回転する（上下・左右反転）
/// 変換式: new[N-1-i][M-1-j] = old[i][j]
fn rotate_180<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![grid[0][0].clone(); m]; n];
    for i in 0..n {
        for j in 0..m {
            new_grid[n - 1 - i][m - 1 - j] = grid[i][j].clone();
        }
    }
    new_grid
}

// ------------------------------------------------------------
// その他関数系
// ------------------------------------------------------------

/// 回文判定
fn is_palindrome(s: &[char]) -> bool {
    s.iter().eq(s.iter().rev())
}

/// 座標圧縮
fn zaatu(v: &Vec<usize>) -> Vec<usize> {
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
fn binary_search<F: Fn(isize) -> bool>(mut ok: isize, mut ng: isize, f: F) -> isize {
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
fn prefix_sum(a: &[i64]) -> Vec<i64> {
    let mut s = vec![0; a.len() + 1];
    for i in 0..a.len() {
        s[i + 1] = s[i] + a[i];
    }
    s
}

/// 2次元累積和
fn grid_ruiseki(a: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
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
fn rle<T: PartialEq + Clone>(items: &[T]) -> Vec<(T, usize)> {
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
fn find_first_ge(seg: &Segtree<Max<i64>>, l: usize, r: usize, a: i64) -> Option<usize> {
    let v = seg.max_right(l, |&v| v < a);
    if v < r {
        Some(v)
    } else {
        None
    }
}

/// Z algorithm: z[i] = s と s[i..] の最長共通接頭辞長 (z[0]=s.len())
fn z_algorithm(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    z[0] = n;
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

// ------------------------------------------------------------
// 文字列系
// ------------------------------------------------------------

/// KMP法（Knuth-Morris-Pratt）
/// pattern の失敗関数（部分一致テーブル）を前計算し、text 内の全一致開始位置を O(|text|+|pattern|) で返す
/// fail[i] = pattern[0..i] の最長の「真の接頭辞かつ接尾辞」の長さ
struct Kmp {
    pattern: Vec<char>,
    fail: Vec<usize>,
}

impl Kmp {
    fn new(pattern: &[char]) -> Self {
        let m = pattern.len();
        let mut fail = vec![0usize; m];
        let mut k = 0usize;
        for i in 1..m {
            while k > 0 && pattern[i] != pattern[k] {
                k = fail[k - 1];
            }
            if pattern[i] == pattern[k] {
                k += 1;
            }
            fail[i] = k;
        }
        Self {
            pattern: pattern.to_vec(),
            fail,
        }
    }

    /// text 内で pattern が一致する開始インデックスを全て返す
    fn search(&self, text: &[char]) -> Vec<usize> {
        let mut res = vec![];
        if self.pattern.is_empty() {
            return res;
        }
        let mut k = 0usize;
        for i in 0..text.len() {
            while k > 0 && text[i] != self.pattern[k] {
                k = self.fail[k - 1];
            }
            if text[i] == self.pattern[k] {
                k += 1;
            }
            if k == self.pattern.len() {
                res.push(i + 1 - k);
                k = self.fail[k - 1];
            }
        }
        res
    }
}

/// Manacher法
/// 文字と文字の間に番兵(区切り)を挟んだ長さ `2n-1` の列 t を考え、
/// t の各中心 i について「t上で何歩まで左右が一致するか」r[i] を O(n) で求める。
/// t[2i] = s[i] （元の文字）、t[2i+1] = 区切り、に対応する。
/// 区切り同士は常に一致するため r[i] はそのまま s 上の長さにはならない。
/// 元の文字列上での回文の長さは palindrome_len(&r, i) で求めること。
/// * s[i] を中心とする最長奇数長回文の長さ = palindrome_len(&r, 2*i)
/// * s[i], s[i+1] の間を中心とする最長偶数長回文の長さ = palindrome_len(&r, 2*i+1)
/// * 回文の開始位置 = (i + 1 - len) / 2 （s上のインデックス）
fn palindrome_len(r: &[usize], i: usize) -> usize {
    if (i + r[i]) % 2 == 0 {
        r[i] + 1
    } else {
        r[i]
    }
}

fn manacher<T: PartialEq + Clone>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    let m = 2 * n - 1;
    let t: Vec<Option<T>> = (0..m)
        .map(|i| {
            if i % 2 == 0 {
                Some(s[i / 2].clone())
            } else {
                None // 区切り文字（区切り同士は常に一致する）
            }
        })
        .collect();

    let mut r = vec![0usize; m];
    let mut c = 0usize; // 直近でrgtを更新した中心
    let mut rgt = 0usize; // 現在分かっている範囲の右端(排他的でない、届く最大index)
    let mut i = 0usize;
    while i < m {
        if i < rgt {
            r[i] = r[2 * c - i].min(rgt - i);
        }
        while i >= r[i] + 1 && i + r[i] + 1 < m && t[i - r[i] - 1] == t[i + r[i] + 1] {
            r[i] += 1;
        }
        if i + r[i] > rgt {
            c = i;
            rgt = i + r[i];
        }
        i += 1;
    }
    r
}

/// トライ木（文字は 'a'..'z' を仮定。他の文字集合の場合は ALPHABET_SIZE / base を変更する）
struct Trie {
    children: Vec<[i32; 26]>,
    is_end: Vec<bool>,
    count: Vec<usize>, // このノードを通過した単語数（prefixカウント用）
}

impl Trie {
    fn new() -> Self {
        Self {
            children: vec![[-1; 26]],
            is_end: vec![false],
            count: vec![0],
        }
    }

    fn insert(&mut self, s: &[u8]) {
        let mut cur = 0usize;
        self.count[cur] += 1;
        for &c in s {
            let idx = (c - b'a') as usize;
            if self.children[cur][idx] == -1 {
                self.children.push([-1; 26]);
                self.is_end.push(false);
                self.count.push(0);
                self.children[cur][idx] = (self.children.len() - 1) as i32;
            }
            cur = self.children[cur][idx] as usize;
            self.count[cur] += 1;
        }
        self.is_end[cur] = true;
    }

    fn contains(&self, s: &[u8]) -> bool {
        let mut cur = 0usize;
        for &c in s {
            let idx = (c - b'a') as usize;
            if self.children[cur][idx] == -1 {
                return false;
            }
            cur = self.children[cur][idx] as usize;
        }
        self.is_end[cur]
    }

    /// s を接頭辞に持つ単語の個数
    fn count_prefix(&self, s: &[u8]) -> usize {
        let mut cur = 0usize;
        for &c in s {
            let idx = (c - b'a') as usize;
            if self.children[cur][idx] == -1 {
                return 0;
            }
            cur = self.children[cur][idx] as usize;
        }
        self.count[cur]
    }
}

/// Aho-Corasick法
/// 複数パターンを text 中から一括検索する
/// ## 使い方
/// let mut ac = AhoCorasick::new();
/// ac.add_pattern(b"he");
/// ac.add_pattern(b"she");
/// ac.build();
/// let hits = ac.search(b"ushers"); // (終了位置, パターンindex) のリスト
/// let cnt = ac.count_matches(b"ushers"); // 一致の総数
struct AhoCorasick {
    children: Vec<[i32; 26]>,
    fail: Vec<usize>,
    // このノードで終了するパターンのindex一覧（build後はfail先の分も含む）
    outputs: Vec<Vec<usize>>,
    pattern_cnt: usize,
}

impl AhoCorasick {
    fn new() -> Self {
        Self {
            children: vec![[-1; 26]],
            fail: vec![0],
            outputs: vec![vec![]],
            pattern_cnt: 0,
        }
    }

    /// パターンを追加し、そのパターンindex（追加した順に 0, 1, 2, ...）を返す
    fn add_pattern(&mut self, s: &[u8]) -> usize {
        let mut cur = 0usize;
        for &c in s {
            let idx = (c - b'a') as usize;
            if self.children[cur][idx] == -1 {
                self.children.push([-1; 26]);
                self.fail.push(0);
                self.outputs.push(vec![]);
                self.children[cur][idx] = (self.children.len() - 1) as i32;
            }
            cur = self.children[cur][idx] as usize;
        }
        let pat_id = self.pattern_cnt;
        self.pattern_cnt += 1;
        self.outputs[cur].push(pat_id);
        pat_id
    }

    /// BFSでfailリンクを構築する。add_pattern を全て呼んだ後に一度だけ呼ぶ
    fn build(&mut self) {
        let mut que = VecDeque::new();
        for c in 0..26 {
            if self.children[0][c] != -1 {
                self.fail[self.children[0][c] as usize] = 0;
                que.push_back(self.children[0][c] as usize);
            } else {
                self.children[0][c] = 0;
            }
        }
        while let Some(v) = que.pop_front() {
            let fv = self.fail[v];
            let out_fv = self.outputs[fv].clone();
            self.outputs[v].extend(out_fv);
            for c in 0..26 {
                let nv = self.children[v][c];
                if nv == -1 {
                    self.children[v][c] = self.children[fv][c];
                } else {
                    self.fail[nv as usize] = self.children[fv][c] as usize;
                    que.push_back(nv as usize);
                }
            }
        }
    }

    /// text 中の全一致を (終了位置(含む), パターンindex) のリストで返す
    /// パターンの開始位置は 終了位置 + 1 - パターン長
    fn search(&self, text: &[u8]) -> Vec<(usize, usize)> {
        let mut cur = 0usize;
        let mut res = vec![];
        for (i, &c) in text.iter().enumerate() {
            let idx = (c - b'a') as usize;
            cur = self.children[cur][idx] as usize;
            for &p in &self.outputs[cur] {
                res.push((i, p));
            }
        }
        res
    }

    /// text 中でパターンが一致した回数の総数（重複・包含も全て数える）
    fn count_matches(&self, text: &[u8]) -> i64 {
        let mut cur = 0usize;
        let mut total = 0i64;
        for &c in text {
            let idx = (c - b'a') as usize;
            cur = self.children[cur][idx] as usize;
            total += self.outputs[cur].len() as i64;
        }
        total
    }
}

/// Suffix Array（倍加法 O(n log^2 n)）
/// s のSuffix Array（各接尾辞を辞書順に並べたときの開始インデックス列）を構築する
fn suffix_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i64> = s.iter().map(|&c| c as i64).collect();
    let mut tmp = vec![0i64; n];
    let mut k = 1usize;

    let compare = |rank: &Vec<i64>, k: usize, a: usize, b: usize| -> std::cmp::Ordering {
        if rank[a] != rank[b] {
            return rank[a].cmp(&rank[b]);
        }
        let ra = if a + k < n { rank[a + k] } else { -1 };
        let rb = if b + k < n { rank[b + k] } else { -1 };
        ra.cmp(&rb)
    };

    while k < n {
        sa.sort_by(|&a, &b| compare(&rank, k, a, b));
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if compare(&rank, k, sa[i - 1], sa[i]) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        rank.copy_from_slice(&tmp);
        k *= 2;
        if rank[sa[n - 1]] as usize == n - 1 {
            break;
        }
    }
    sa
}

/// Kasaiのアルゴリズムで LCP配列（高さ配列）を構築する
/// ## 出力
/// `lcp[i]` - sa[i] と sa[i+1] の接尾辞の最長共通接頭辞長（長さ n-1）
fn lcp_array(s: &[u8], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0usize; n];
    for i in 0..n {
        rank[sa[i]] = i;
    }
    let mut lcp = vec![0usize; n.saturating_sub(1)];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] == 0 {
            h = 0;
            continue;
        }
        let j = sa[rank[i] - 1];
        if h > 0 {
            h -= 1;
        }
        while i + h < n && j + h < n && s[i + h] == s[j + h] {
            h += 1;
        }
        lcp[rank[i] - 1] = h;
    }
    lcp
}

/// Double Rolling Hash（衝突確率を大幅に下げた二重ハッシュ）
/// (h1, h2) のペアで一致判定することでハッシュ衝突をほぼ無視できる
/// 基数は実行ごとにランダム。2つの文字列を比較するときは with_base で同じ基数を使うこと
struct DoubleRollingHash {
    hash1: Vec<u64>,
    hash2: Vec<u64>,
    pow1: Vec<u64>,
    pow2: Vec<u64>,
    bases: (u64, u64),
}

impl DoubleRollingHash {
    const MOD1: u64 = 1_000_000_007;
    const MOD2: u64 = 998_244_353;

    fn new(s: &[u8]) -> Self {
        let r = random_u64();
        let bases = (
            (r & 0xFFFF_FFFF) % (Self::MOD1 - 1000) + 500,
            (r >> 32) % (Self::MOD2 - 1000) + 500,
        );
        Self::with_base(s, bases)
    }

    fn with_base(s: &[u8], bases: (u64, u64)) -> Self {
        let (b1, b2) = bases;
        let n = s.len();
        let mut hash1 = vec![0u64; n + 1];
        let mut hash2 = vec![0u64; n + 1];
        let mut pow1 = vec![1u64; n + 1];
        let mut pow2 = vec![1u64; n + 1];
        for i in 0..n {
            let c = s[i] as u64 + 1;
            hash1[i + 1] = (hash1[i] * b1 + c) % Self::MOD1;
            hash2[i + 1] = (hash2[i] * b2 + c) % Self::MOD2;
            pow1[i + 1] = pow1[i] * b1 % Self::MOD1;
            pow2[i + 1] = pow2[i] * b2 % Self::MOD2;
        }
        Self {
            hash1,
            hash2,
            pow1,
            pow2,
            bases,
        }
    }

    /// s[l..r) のハッシュ値 (h1, h2) を返す
    fn get(&self, l: usize, r: usize) -> (u64, u64) {
        let h1 = (self.hash1[r] + Self::MOD1 - self.hash1[l] * self.pow1[r - l] % Self::MOD1)
            % Self::MOD1;
        let h2 = (self.hash2[r] + Self::MOD2 - self.hash2[l] * self.pow2[r - l] % Self::MOD2)
            % Self::MOD2;
        (h1, h2)
    }
}

// ------------------------------------------------------------
// 幾何系
// ------------------------------------------------------------

/// 2次元座標を表す点。f64 だけでなく i64 でも使えるよう型引数を持たせる想定なら
/// 別途ジェネリック化してもよいが、ここでは f64 前提のシンプル版を用意する。
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// 外積 (cross product) : a × b
/// > 0 なら b は a から見て反時計回り、< 0 なら時計回り、0 なら平行
fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

/// 内積 (dot product)
fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

/// ccw (counter-clockwise) : 点 p0, p1, p2 の位置関係を判定する
/// 1  : 反時計回り (COUNTER_CLOCKWISE)
/// -1 : 時計回り (CLOCKWISE)
/// 2  : p2, p0, p1 の順で一直線上 (p0 が p1-p2 の外側、逆方向)  (ONLINE_BACK)
/// -2 : p0, p1, p2 の順で一直線上 (p2 が p0-p1 の先) (ONLINE_FRONT)
/// 0  : p0, p1 の線分上に p2 が乗っている (ON_SEGMENT)
fn ccw(p0: Point, p1: Point, p2: Point) -> i32 {
    let a = p1 - p0;
    let b = p2 - p0;
    if cross(a, b) > EPS {
        return 1;
    }
    if cross(a, b) < -EPS {
        return -1;
    }
    if dot(a, b) < -EPS {
        return 2;
    }
    if a.x * a.x + a.y * a.y < b.x * b.x + b.y * b.y - EPS {
        return -2;
    }
    0
}

/// 凸包（Andrewのモノトーンチェイン法）O(n log n)
/// ## 出力
/// 凸包を構成する点を反時計回りに並べたもの（重複・共線点は含まない）
fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
    });
    pts.dedup_by(|a, b| (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS);
    let n = pts.len();
    if n <= 2 {
        return pts;
    }

    let mut hull: Vec<Point> = Vec::with_capacity(2 * n);
    // 下側
    for &p in &pts {
        while hull.len() >= 2
            && cross(
                hull[hull.len() - 1] - hull[hull.len() - 2],
                p - hull[hull.len() - 2],
            ) <= EPS
        {
            hull.pop();
        }
        hull.push(p);
    }
    // 上側
    let lower_size = hull.len() + 1;
    for &p in pts.iter().rev().skip(1) {
        while hull.len() >= lower_size
            && cross(
                hull[hull.len() - 1] - hull[hull.len() - 2],
                p - hull[hull.len() - 2],
            ) <= EPS
        {
            hull.pop();
        }
        hull.push(p);
    }
    hull.pop(); // 最後は始点と重複するので除く
    hull
}

// ------------------------------------------------------------
// MOD演算用
// ------------------------------------------------------------

fn power(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1u64;
    base %= modulo;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result
}

/// NTT (数論変換, mod = 998244353 決め打ち)
/// 畳み込みが必要な場合は ac_library::convolution::convolution / convolution_i64 を
/// 使うのが基本（内部でNTTが使われている）。以下は自前実装の参考用。
/// let c = ntt_convolution(&a, &b); // a, b は係数列 (下位桁が先頭)
fn ntt_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    type Mint = ModInt998244353;
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut fa: Vec<Mint> = a.iter().map(|&x| Mint::new(x)).collect();
    let mut fb: Vec<Mint> = b.iter().map(|&x| Mint::new(x)).collect();

    let n = fa.len() + fb.len() - 1;
    let mut sz = 1usize;
    while sz < n {
        sz *= 2;
    }
    fa.resize(sz, Mint::new(0));
    fb.resize(sz, Mint::new(0));

    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..sz {
        fa[i] *= fb[i];
    }
    ntt(&mut fa, true);

    let inv_sz = Mint::new(sz as i64).inv();
    fa.iter()
        .take(n)
        .map(|&x| (x * inv_sz).val() as i64)
        .collect()
}

fn ntt(f: &mut Vec<ModInt998244353>, invert: bool) {
    type Mint = ModInt998244353;
    let n = f.len();
    if n == 1 {
        return;
    }
    // ビット反転
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            f.swap(i, j);
        }
    }

    let g = Mint::new(3); // 998244353 の原始根
    let mut len = 2usize;
    while len <= n {
        let mut w = g.pow((MOD - 1) / len as u64);
        if invert {
            w = w.inv();
        }
        let mut i = 0usize;
        while i < n {
            let mut wn = Mint::new(1);
            for k in 0..len / 2 {
                let u = f[i + k];
                let v = f[i + k + len / 2] * wn;
                f[i + k] = u + v;
                f[i + k + len / 2] = u - v;
                wn *= w;
            }
            i += len;
        }
        len <<= 1;
    }

    if invert {
        let inv_n = Mint::new(n as i64).inv();
        for x in f.iter_mut() {
            *x *= inv_n;
        }
    }
}

// ------------------------------------------------------------
// セグメントツリー
// ------------------------------------------------------------

//let mut seg = Segtree::<Sum>::from(vec![(0i64, 1i64); n]); // 各要素 (値, 1) で初期化
//一点更新+区間和（要素数も持つので遅延セグ木の RangeAddRangeSum 等と共用できる）

struct Sum;
impl Monoid for Sum {
    type S = (i64, i64); // (合計, 要素数)
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

//let mut seg = Segtree::<InvCount>::from(vec![(0u64, 0u64, 0u64); n]); // 各要素(1,0,0)or(0,1,0)で初期化
// 0/1列の転倒数

struct InvCount;
impl Monoid for InvCount {
    type S = (u64, u64, u64); // (#0の数, #1の数, 転倒数)
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2 + a.1 * b.0)
    }
}
//let mut seg = Segtree::<MinMax>::from(vec![(0i64, 0i64); n]);
// 区間最小値と最大値

struct MinMax;
impl Monoid for MinMax {
    type S = (i64, i64); // (最小値, 最大値)
    fn identity() -> Self::S {
        (i64::MAX, i64::MIN)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0.min(b.0), a.1.max(b.1))
    }
}

//let mut seg = Segtree::<Xor>::from(vec![0i64; n]);
// 区間XOR

struct Xor;
impl Monoid for Xor {
    type S = i64;
    fn identity() -> Self::S {
        0
    } // 0 XOR x = x
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

//let mut seg = Segtree::<Gcd>::from(vec![0i64; n]);
// 区間最大公約数

struct Gcd;
impl Monoid for Gcd {
    type S = i64;
    fn identity() -> Self::S {
        0
    } // gcd(x, 0) = x
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        gcd(*a, *b)
    }
}
//let mut seg = Segtree::<MinCount>::from(vec![(1i64, 1i64); n]); // 各要素(値, 1)で初期化
// 区間最小値とその個数

struct MinCount;
impl Monoid for MinCount {
    type S = (i64, i64); // (最小値, その個数)
    fn identity() -> Self::S {
        (i64::MAX, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 < b.0 {
            *a
        } else if a.0 > b.0 {
            *b
        } else {
            (a.0, a.1 + b.1)
        }
    }
}

// ------------------------------------------------------------
// 遅延セグメントツリー
// ------------------------------------------------------------

//let mut seg = LazySegtree::<RangeAddRangeSum>::from(vec![(0, 1); n]);
//区間加算+区間和

struct RangeAddRangeSum;
impl MapMonoid for RangeAddRangeSum {
    type M = Sum;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x.0 + f * x.1, x.1)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<RangeAddRangeMin>::from(vec![0i64; n]);
//区間加算 + 区間最小値

struct RangeAddRangeMin;
impl MapMonoid for RangeAddRangeMin {
    type M = Min<i64>;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<Flip>::from(vec![(0u64, 0u64, 0u64); n]); // 各要素(1,0,0)or(0,1,0)で初期化
// 区間反転 + 0/1列の転倒数

struct Flip;
impl MapMonoid for Flip {
    type M = InvCount;
    type F = bool;
    fn identity_map() -> Self::F {
        false
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f {
            (x.1, x.0, x.0 * x.1 - x.2)
        } else {
            *x
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
    }
}

//let mut seg = LazySegtree::<RangeUpdateRangeMax>::from(vec![i64::MIN; n]);
// 区間更新(代入) + 区間最大値

struct RangeUpdateRangeMax;
impl MapMonoid for RangeUpdateRangeMax {
    type M = Max<i64>;
    type F = Option<i64>;
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.unwrap_or(*x)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

//let mut seg = LazySegtree::<RangeAddRangeMax>::from(vec![0i64; n]);
// 区間加算 + 区間最大値

struct RangeAddRangeMax;
impl MapMonoid for RangeAddRangeMax {
    type M = Max<i64>;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<RangeXorRangeXor>::from(vec![0i64; n]);
// 区間XOR + 区間一括XOR

struct RangeXorRangeXor;
impl MapMonoid for RangeXorRangeXor {
    type M = Xor;
    type F = i64; // 一括でXORする値
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f ^ x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
    }
}

//let init: Vec<(i64, i64)> = vec![(0, 1); n];
//let mut seg = LazySegtree::<RangeUpdateRangeSum>::from(init);
// 区間更新(代入) + 区間和

struct RangeUpdateRangeSum;
impl MapMonoid for RangeUpdateRangeSum {
    type M = Sum;
    type F = Option<i64>;
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        match f {
            Some(v) => (v * x.1, x.1),
            None => *x,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

/// Sparse Table
/// 静的配列（更新なし）に対する区間min/max/gcdなどの冪等モノイドクエリを O(1) で処理する
/// 構築 O(n log n)、クエリ O(1)
/// ## 使い方
/// let st = SparseTable::new(&a, |x, y| *x.min(y)); // 区間min
/// st.query(l, r) // [l, r) の最小値
struct SparseTable<T, F> {
    table: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(&T, &T) -> T> SparseTable<T, F> {
    fn new(a: &[T], op: F) -> Self {
        let n = a.len();
        let mut log = 1usize;
        while (1 << log) <= n {
            log += 1;
        }
        let mut table = vec![a.to_vec()];
        for k in 1..log {
            let prev = &table[k - 1];
            let half = 1usize << (k - 1);
            let mut row = Vec::with_capacity(n - (1 << k) + 1);
            for i in 0..=(n - (1 << k)) {
                row.push(op(&prev[i], &prev[i + half]));
            }
            table.push(row);
        }
        Self { table, op }
    }

    /// [l, r) の集約値（冪等演算であること: min, max, gcd, and, or など）
    fn query(&self, l: usize, r: usize) -> T {
        let len = r - l;
        let k = (usize::BITS - 1 - len.leading_zeros()) as usize; // floor(log2(len))
        (self.op)(&self.table[k][l], &self.table[k][r - (1 << k)])
    }
}

// ------------------------------------------------------------
// FenwickTree（BIT）使用例（ac_library）
// ------------------------------------------------------------
// let mut bit = FenwickTree::<i64>::new(n, 0);
// bit.add(i, x);                  // a[i] += x
// let s = bit.sum(l, r);          // a[l] + ... + a[r-1]
// 一点更新・区間和だけなら上のSegtree<Sum>より軽量で高速

/// lower_bound付きBIT（値は非負のみ対応。1-indexedで内部管理）
/// 「先頭からの累積和が w 以上になる最小のインデックス」を O(log n) で求められる
/// 通常の FenwickTree (ac_library) は add / sum のみで lower_bound 相当が無いため自作する
struct BitLowerBound {
    n: usize,
    data: Vec<i64>, // 1-indexed
    log: usize,
}

impl BitLowerBound {
    fn new(n: usize) -> Self {
        let mut log = 0usize;
        while (1usize << log) <= n {
            log += 1;
        }
        Self {
            n,
            data: vec![0; n + 1],
            log,
        }
    }

    /// a[i] += x (0-indexed, x >= 0 を想定)
    fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    /// a[0] + ... + a[i-1] (0-indexed prefix sum, [0, i))
    fn sum(&self, i: usize) -> i64 {
        let mut i = i;
        let mut s = 0i64;
        while i > 0 {
            s += self.data[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    /// 累積和が w 以上になる最小のインデックス pos（0-indexed）を返す
    /// つまり sum(0..=pos) >= w となる最小の pos 。存在しなければ n を返す
    /// 各要素は非負であることが前提
    fn lower_bound(&self, w: i64) -> usize {
        if w <= 0 {
            return 0;
        }
        let mut w = w;
        let mut x = 0usize; // 1-indexed 上の現在位置
        let mut k = if self.log == 0 {
            0
        } else {
            1usize << (self.log - 1)
        };
        while k > 0 {
            if x + k <= self.n && self.data[x + k] < w {
                x += k;
                w -= self.data[x];
            }
            k >>= 1;
        }
        x // 0-indexedでの「境界の直前」= 答えのインデックス
    }
}

// ------------------------------------------------------------
// 2次元BIT (2D Fenwick Tree)
// ------------------------------------------------------------

/// 2次元BIT（点更新・矩形和取得）
/// let mut bit2 = Bit2D::new(h, w);
/// bit2.add(i, j, x);
/// bit2.sum(r1, c1, r2, c2); // [r1,r2) x [c1,c2) の和
struct Bit2D {
    h: usize,
    w: usize,
    data: Vec<Vec<i64>>, // 1-indexed
}

impl Bit2D {
    fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![vec![0; w + 1]; h + 1],
        }
    }

    /// a[i][j] += x (0-indexed)
    fn add(&mut self, i: usize, j: usize, x: i64) {
        let mut ii = i + 1;
        while ii <= self.h {
            let mut jj = j + 1;
            while jj <= self.w {
                self.data[ii][jj] += x;
                jj += jj & jj.wrapping_neg();
            }
            ii += ii & ii.wrapping_neg();
        }
    }

    /// [0, i) x [0, j) の和 (0-indexed, prefix)
    fn prefix_sum(&self, i: usize, j: usize) -> i64 {
        let mut s = 0i64;
        let mut ii = i;
        while ii > 0 {
            let mut jj = j;
            while jj > 0 {
                s += self.data[ii][jj];
                jj -= jj & jj.wrapping_neg();
            }
            ii -= ii & ii.wrapping_neg();
        }
        s
    }

    /// [r1, r2) x [c1, c2) の和 (0-indexed半開区間)
    fn sum(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
        self.prefix_sum(r2, c2) - self.prefix_sum(r1, c2) - self.prefix_sum(r2, c1)
            + self.prefix_sum(r1, c1)
    }
}

// ------------------------------------------------------------
// その他データ構造系
// ------------------------------------------------------------

/// 多重集合
struct Multiset<T> {
    map: BTreeMap<T, usize>,
    len: usize,
}

impl<T: Ord> Multiset<T> {
    fn new() -> Self {
        Multiset {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    fn insert(&mut self, value: T) {
        *self.map.entry(value).or_insert(0) += 1;
        self.len += 1;
    }

    fn remove(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.get_mut(value) {
            if *count > 0 {
                *count -= 1;
                self.len -= 1;
                if *count == 0 {
                    self.map.remove(value);
                }
                return true;
            }
        }
        false
    }

    fn count(&self, value: &T) -> usize {
        *self.map.get(value).unwrap_or(&0)
    }

    /// 集合全体の要素数（種類数ではなく、多重度を含めた総数）
    fn len(&self) -> usize {
        self.len
    }

    fn min_element(&self) -> Option<&T> {
        self.map.keys().next()
    }

    fn max_element(&self) -> Option<&T> {
        self.map.keys().next_back()
    }

    fn pop_min(&mut self) -> Option<T>
    where
        T: Clone,
    {
        let min_key = self.min_element()?.clone();

        self.remove(&min_key);

        Some(min_key)
    }
}

/// 実行ごとに変わる乱数を1つ返す（ハッシュの基数などに使う）
fn random_u64() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u64(0x9E37_79B9_7F4A_7C15);
    h.finish()
}

/// Rolling Hash（mod 2^61-1, 基数は実行ごとにランダム）
/// 単一mod 1e9 程度だと 10^5 個の比較で普通に衝突するため、2^61-1 を使う
/// let rh = RollingHash::new(s.as_bytes());
/// rh.get_hash(l, r) // s[l..r) のハッシュ
/// 2つの文字列を比較するときは RollingHash::with_base(t, rh.base) で同じ基数を使うこと
struct RollingHash {
    hash: Vec<u64>,
    power: Vec<u64>,
    base: u64,
}

impl RollingHash {
    const MOD: u64 = (1u64 << 61) - 1;

    fn mul(a: u64, b: u64) -> u64 {
        let t = a as u128 * b as u128;
        let t = (t >> 61) as u64 + (t as u64 & Self::MOD);
        if t >= Self::MOD {
            t - Self::MOD
        } else {
            t
        }
    }

    fn new(s: &[u8]) -> Self {
        let base = random_u64() % (Self::MOD - 1000) + 500;
        Self::with_base(s, base)
    }

    fn with_base(s: &[u8], base: u64) -> Self {
        let n = s.len();
        let mut hash = vec![0u64; n + 1];
        let mut power = vec![1u64; n + 1];
        for i in 0..n {
            hash[i + 1] = (Self::mul(hash[i], base) + s[i] as u64 + 1) % Self::MOD;
            power[i + 1] = Self::mul(power[i], base);
        }
        Self { hash, power, base }
    }

    /// s[l..r) のハッシュ値
    fn get_hash(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + Self::MOD - Self::mul(self.hash[l], self.power[r - l])) % Self::MOD
    }
}

// ------------------------------------------------------------
// IntervalSet（半開区間 [l, r) の集合）
// ------------------------------------------------------------
// let mut s = IntervalSet::new();
// s.insert(l, r);        // [l, r) を追加。新たに埋まった長さを返す
// s.remove(l, r);        // [l, r) を削除。実際に消えた長さを返す
// s.get(x);              // x を含む区間 Some((l, r))、なければ None
// s.contains(x);         // x が含まれるか
// s.same(a, b);          // a と b が同じ区間に含まれるか
// s.mex(x);              // x 以上で、どの区間にも含まれない最小の整数
// s.covered();           // 全区間の長さの合計（O(1)）
// s.len();               // 区間の個数
// s.iter();              // (l, r) を昇順に列挙
// ・隣り合う区間 ([1,3) と [3,5)) は [1,5) に結合される
// ・閉区間 [l, r] を扱うなら insert(l, r + 1) のように渡す
// ・複数持つなら vec![IntervalSet::new(); n] で OK（Clone 実装済み）

#[derive(Clone, Debug, Default)]
struct IntervalSet {
    map: BTreeMap<i64, i64>, // 開始 -> 終端（半開）
    total: i64,              // カバーしている長さの合計
}

impl IntervalSet {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            total: 0,
        }
    }

    /// [l, r) を追加する。新たにカバーされた長さを返す
    fn insert(&mut self, mut l: i64, mut r: i64) -> i64 {
        if l >= r {
            return 0;
        }
        let mut removed_len = 0; // 結合して消した既存区間の長さ

        // 左: l 以下で始まる最後の区間が l に届く（隣接含む）なら結合
        let left = self.map.range(..=l).next_back().map(|(&a, &b)| (a, b));
        if let Some((pl, pr)) = left {
            if pr >= l {
                removed_len += pr - pl;
                self.map.remove(&pl);
                l = pl;
                r = max(r, pr);
            }
        }

        // 右: 開始が r 以下の区間（隣接含む）を全部取り込む
        loop {
            let right = self.map.range(l..).next().map(|(&a, &b)| (a, b));
            match right {
                Some((nl, nr)) if nl <= r => {
                    removed_len += nr - nl;
                    self.map.remove(&nl);
                    r = max(r, nr);
                }
                _ => break,
            }
        }

        self.map.insert(l, r);
        let added = (r - l) - removed_len;
        self.total += added;
        added
    }

    /// [l, r) を削除する。実際に消えた長さを返す
    fn remove(&mut self, l: i64, r: i64) -> i64 {
        if l >= r {
            return 0;
        }
        let mut removed = 0;

        // l 以上 r 未満で始まる区間
        let mids: Vec<(i64, i64)> = self.map.range(l..r).map(|(&a, &b)| (a, b)).collect();
        for (a, b) in mids {
            self.map.remove(&a);
            removed += min(b, r) - a;
            if b > r {
                self.map.insert(r, b); // 右にはみ出た部分を残す
            }
        }

        // l より前で始まり、l をまたぐ区間
        let left = self.map.range(..l).next_back().map(|(&a, &b)| (a, b));
        if let Some((pl, pr)) = left {
            if pr > l {
                removed += min(pr, r) - l;
                self.map.insert(pl, l); // 左側を残す
                if pr > r {
                    self.map.insert(r, pr); // 右側も残す
                }
            }
        }

        self.total -= removed;
        removed
    }

    /// x を含む区間 [l, r) を返す
    fn get(&self, x: i64) -> Option<(i64, i64)> {
        self.map
            .range(..=x)
            .next_back()
            .filter(|&(_, &r)| x < r)
            .map(|(&l, &r)| (l, r))
    }

    fn contains(&self, x: i64) -> bool {
        self.get(x).is_some()
    }

    /// a と b が同じ区間に含まれるか
    fn same(&self, a: i64, b: i64) -> bool {
        match (self.get(a), self.get(b)) {
            (Some(p), Some(q)) => p == q,
            _ => false,
        }
    }

    /// x 以上で、どの区間にも含まれない最小の整数 (mex)
    /// 隣接区間は結合済みなので、区間の終端 r は必ず未カバー
    fn mex(&self, x: i64) -> i64 {
        match self.get(x) {
            Some((_, r)) => r,
            None => x,
        }
    }

    /// カバーしている長さの合計
    fn covered(&self) -> i64 {
        self.total
    }

    /// 区間の個数
    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// (l, r) を昇順に列挙
    fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.map.iter().map(|(&l, &r)| (l, r))
    }
}

/// Index Monotonic Queue
struct IndexMonotonicQueue<T> {
    deque: VecDeque<(usize, T)>,
}

impl<T: Ord> IndexMonotonicQueue<T> {
    fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    fn push(&mut self, index: usize, value: T) {
        while let Some(&(_, ref back)) = self.deque.back() {
            if *back < value {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, value));
    }

    fn pop(&mut self, index: usize) {
        if let Some(&(front_index, _)) = self.deque.front() {
            if front_index == index {
                self.deque.pop_front();
            }
        }
    }

    fn max(&self) -> Option<&T> {
        self.deque.front().map(|&(_, ref value)| value)
    }
}

/// LIS（狭義単調増加）
/// ## 出力
/// `dp[k]` - 長さ k+1 の増加部分列の末尾として取りうる最小値。LISの長さは `dp.len()`
/// （dp自体はLISそのものではないので、復元したい場合は lis_restore を使う）
fn lis(sequence: &Vec<i64>) -> Vec<i64> {
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
fn lis_restore(sequence: &[i64]) -> Vec<i64> {
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

// ------------------------------------------------------------
// グラフ系
// ------------------------------------------------------------

/// 再帰DFS(グラフ)
fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[v] = true;

    for &next_v in &g[v] {
        if seen[next_v] {
            continue;
        }

        dfs(next_v, g, seen);
    }
}

/// 再帰DFS(迷路)
fn dfs2(y: usize, x: usize, h: usize, w: usize, maze: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) {
    seen[y][x] = true;

    let dy = [1, 0, -1, 0];
    let dx = [0, 1, 0, -1];

    for i in 0..4 {
        let ny = y as isize + dy[i];
        let nx = x as isize + dx[i];

        if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
            let (ny, nx) = (ny as usize, nx as usize);
            if maze[ny][nx] != '#' && !seen[ny][nx] {
                dfs2(ny, nx, h, w, maze, seen);
            }
        }
    }
}

/// 無向グラフのサイクル検出
/// 多重辺があるとバグります（辺の番号ではなく頂点番号でparentを判定しているため）
fn has_cycle(v: usize, parent: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> bool {
    seen[v] = true;
    for &next_v in &g[v] {
        if next_v == parent {
            continue;
        }
        if seen[next_v] || has_cycle(next_v, v, g, seen) {
            return true;
        }
    }
    false
}

/// 有向グラフのサイクル検出
fn has_cycle2(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<u8>) -> bool {
    seen[v] = 1;
    for &next_v in &g[v] {
        if seen[next_v] == 1 || (seen[next_v] == 0 && has_cycle2(next_v, g, seen)) {
            return true;
        }
    }
    seen[v] = 2;
    false
}

/// 頂点sからのオイラーツアー
/// ## 出力
/// 頂点sからのオイラーツアー（頂点の訪問順列）
/// ### 計算量 O(N+M)
fn euler_tour(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<usize> {
    let mut tour = Vec::new();
    let mut visited = vec![false; graph.len()];

    let mut stack = vec![(s, true)];

    while let Some((v, is_pre)) = stack.pop() {
        if is_pre {
            if visited[v] {
                continue;
            }
            visited[v] = true;

            tour.push(v);

            stack.push((v, false));

            for &(next_v, _) in graph[v].iter().rev() {
                if !visited[next_v] {
                    stack.push((next_v, true));
                }
            }
        } else {
            tour.push(v);
        }
    }
    tour
}

/// BFSによる頂点sから各頂点への最短ステップ数
/// ## 出力
/// `vec[i]` - 頂点sから頂点iへの最短ステップ数（到達不可能なら`-1`）
/// ### 計算量 O(N+M)
fn bfs(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![-1i64; graph.len()];
    let mut que = std::collections::VecDeque::new();
    dist[s] = 0;
    que.push_back(s);
    while let Some(v) = que.pop_front() {
        for &(next_v, _) in &graph[v] {
            if dist[next_v] == -1 {
                dist[next_v] = dist[v] + 1;
                que.push_back(next_v);
            }
        }
    }
    dist
}

/// 0-1 BFS
/// 辺のコストが 0 または 1 のみのグラフで、頂点sから各頂点への最短距離を
/// ダイクストラ法より軽量な O(N+M) で求める（両端キューを使う）
/// ## 入力
/// graph[v] = (行き先, コスト) のリスト。コストは 0 か 1 のみを想定
fn bfs01(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let n = graph.len();
    let mut dist = vec![i64::MAX; n];
    let mut deque: VecDeque<usize> = VecDeque::new();
    dist[s] = 0;
    deque.push_back(s);
    while let Some(v) = deque.pop_front() {
        // 古い情報はスキップ（本来は dist の再チェックで十分だが、明示的に距離を確認）
        for &(next_v, cost) in &graph[v] {
            let nd = dist[v] + cost;
            if nd < dist[next_v] {
                dist[next_v] = nd;
                if cost == 0 {
                    deque.push_front(next_v);
                } else {
                    deque.push_back(next_v);
                }
            }
        }
    }
    dist
}

/// ダイクストラ法による頂点sから各頂点への最小コスト（非負コストのみ）
/// ## 出力
/// `vec[i]` - 頂点sから頂点iへの最小コスト（到達不可能なら`i64::MAX`）
/// ### 計算量 O((N+M) log N)
fn djk(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![i64::MAX; graph.len()];
    let mut pq = std::collections::BinaryHeap::new();
    dist[s] = 0;
    pq.push(std::cmp::Reverse((0i64, s)));
    while let Some(std::cmp::Reverse((d, v))) = pq.pop() {
        if d > dist[v] {
            continue;
        }
        for &(next_v, cost) in &graph[v] {
            if dist[v] + cost < dist[next_v] {
                dist[next_v] = dist[v] + cost;
                pq.push(std::cmp::Reverse((dist[next_v], next_v)));
            }
        }
    }
    dist
}

/// ワーシャルフロイド法による全頂点間最短コスト
/// ## 出力
/// `vec[i][j]` - 頂点iから頂点jへの最短コスト（到達不可能なら`i64::MAX/2`）
/// ### 計算量 O(N^3)
fn warshall(graph: &[Vec<(usize, i64)>]) -> Vec<Vec<i64>> {
    let n = graph.len();
    let inf = i64::MAX / 2;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for u in 0..n {
        for &(v, w) in &graph[u] {
            dist[u][v] = dist[u][v].min(w);
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] < inf && dist[k][j] < inf {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }
    dist
}

/// ベルマンフォード法による頂点sから各頂点への最小コスト（負のコストも可）
/// ## 出力
/// * `vec[i]` - 頂点sから頂点iへの最小コスト（到達不可能なら`i64::MAX`）
/// * `bool` - 負閉路が存在するなら`true`
/// ### 計算量 O(NM)
fn bellman(graph: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, bool) {
    let n = graph.len();
    let mut dist = vec![i64::MAX; n];
    dist[s] = 0;
    for _ in 0..n - 1 {
        for v in 0..n {
            if dist[v] == i64::MAX {
                continue;
            }
            for &(next_v, cost) in &graph[v] {
                if dist[v] + cost < dist[next_v] {
                    dist[next_v] = dist[v] + cost;
                }
            }
        }
    }
    let mut has_negative_cycle = false;
    for v in 0..n {
        if dist[v] == i64::MAX {
            continue;
        }
        for &(next_v, cost) in &graph[v] {
            if dist[v] + cost < dist[next_v] {
                has_negative_cycle = true;
            }
        }
    }
    (dist, has_negative_cycle)
}

/// クラスカル法による最小全域木
/// ## 出力
/// * `Some((合計コスト, 辺のリスト(頂点, 頂点, コスト)))` - 存在する場合
/// * `None` - 非連結で存在しない場合
/// ### 計算量 O(M log M)
fn kruskal(graph: &[Vec<(usize, i64)>]) -> Option<(i64, Vec<(usize, usize, i64)>)> {
    let n = graph.len();
    if n == 0 {
        return Some((0, vec![]));
    }

    let mut edges: Vec<(i64, usize, usize)> = vec![];
    for u in 0..n {
        for &(v, w) in &graph[u] {
            if u < v {
                edges.push((w, u, v));
            }
        }
    }
    edges.sort();

    use ac_library::Dsu;
    let mut uf = Dsu::new(n);
    let mut total_cost = 0i64;
    let mut mst_edges = vec![];
    for (w, u, v) in edges {
        if !uf.same(u, v) {
            uf.merge(u, v);
            total_cost += w;
            mst_edges.push((u, v, w));
        }
        if mst_edges.len() == n - 1 {
            break;
        }
    }
    if mst_edges.len() < n - 1 {
        None
    } else {
        Some((total_cost, mst_edges))
    }
}

/// 二部グラフ判定（無向グラフ用）
/// ## 出力
/// 二部グラフなら`true`
/// ### 計算量 O(N+M)
fn is_bipartite(graph: &[Vec<(usize, i64)>]) -> bool {
    let n = graph.len();
    let mut color = vec![-1i64; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        let mut que = std::collections::VecDeque::new();
        color[start] = 0;
        que.push_back(start);
        while let Some(v) = que.pop_front() {
            for &(next_v, _) in &graph[v] {
                if color[next_v] == -1 {
                    color[next_v] = 1 - color[v];
                    que.push_back(next_v);
                } else if color[next_v] == color[v] {
                    return false;
                }
            }
        }
    }
    true
}

/// 連結成分分解（無向グラフ用）
/// ## 出力
/// `vec[i]` - 連結成分iに含まれる頂点リスト
/// ### 計算量 O(N+M)
fn conn_comp(graph: &[Vec<(usize, i64)>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut seen = vec![false; n];
    let mut components = vec![];
    for start in 0..n {
        if seen[start] {
            continue;
        }
        let mut component = vec![];
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(v) = stack.pop() {
            component.push(v);
            for &(next_v, _) in &graph[v] {
                if !seen[next_v] {
                    seen[next_v] = true;
                    stack.push(next_v);
                }
            }
        }
        components.push(component);
    }
    components
}

/// トポロジカルソート（有向グラフ用）
/// ## 出力
/// * `Some(Vec<usize>)` - サイクルがない場合のソート結果
/// * `None` - サイクルが存在する場合
/// ### 計算量 O(N+M)
fn topo(graph: &[Vec<(usize, i64)>]) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for &(v, _) in &graph[u] {
            in_degree[v] += 1;
        }
    }
    let mut que = std::collections::VecDeque::new();
    for i in 0..n {
        if in_degree[i] == 0 {
            que.push_back(i);
        }
    }
    let mut result = vec![];
    while let Some(v) = que.pop_front() {
        result.push(v);
        for &(next_v, _) in &graph[v] {
            in_degree[next_v] -= 1;
            if in_degree[next_v] == 0 {
                que.push_back(next_v);
            }
        }
    }
    if result.len() == n {
        Some(result)
    } else {
        None
    }
}

/// 木の直径（最も遠い2頂点間のステップ数）
/// 辺の重みは無視する（重み付きの直径が欲しい場合は bfs の代わりに djk を使う）
/// ### 計算量 O(N+M)
fn tree_diameter(graph: &[Vec<(usize, i64)>]) -> i64 {
    if graph.is_empty() {
        return 0;
    }

    let farthest = |s: usize| -> usize {
        bfs(graph, s)
            .iter()
            .enumerate()
            .max_by_key(|&(_, d)| d)
            .unwrap()
            .0
    };
    let u = farthest(0);
    let v = farthest(u);
    bfs(graph, u)[v]
}

/// LCA（ダブリングによる最小共通祖先）
/// ## 使い方
/// let lca = Lca::new(&graph, root);
/// lca.query(u, v)     -> uとvのLCA
/// lca.dist(u, v)      -> uとv間の辺数（非重み距離）
/// ### 構築 O(N log N)、クエリ O(log N)
struct Lca {
    depth: Vec<i64>,
    parent: Vec<Vec<usize>>, // parent[k][v] = vから2^k個祖先を遡った頂点
    log: usize,
}

impl Lca {
    fn new(graph: &[Vec<(usize, i64)>], root: usize) -> Self {
        let n = graph.len();
        let mut log = 1;
        while (1 << log) < n {
            log += 1;
        }
        log += 1;

        let mut depth = vec![0i64; n];
        let mut parent = vec![vec![usize::MAX; n]; log];
        let mut seen = vec![false; n];
        let mut stack = vec![(root, usize::MAX, 0i64)];
        seen[root] = true;

        while let Some((v, p, d)) = stack.pop() {
            depth[v] = d;
            parent[0][v] = p;
            for &(next_v, _) in &graph[v] {
                if !seen[next_v] {
                    seen[next_v] = true;
                    stack.push((next_v, v, d + 1));
                }
            }
        }

        for k in 0..log - 1 {
            for v in 0..n {
                parent[k + 1][v] = if parent[k][v] == usize::MAX {
                    usize::MAX
                } else {
                    parent[k][parent[k][v]]
                };
            }
        }

        Self { depth, parent, log }
    }

    fn query(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            swap(&mut u, &mut v);
        }
        let diff = self.depth[u] - self.depth[v];
        for k in 0..self.log {
            if (diff >> k) & 1 == 1 {
                u = self.parent[k][u];
            }
        }
        if u == v {
            return u;
        }
        for k in (0..self.log).rev() {
            if self.parent[k][u] != self.parent[k][v] {
                u = self.parent[k][u];
                v = self.parent[k][v];
            }
        }
        self.parent[0][u]
    }

    fn dist(&self, u: usize, v: usize) -> i64 {
        let l = self.query(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[l]
    }
}

/// HLD（Heavy-Light Decomposition, 重軽分解）
/// 木上のパスクエリ・部分木クエリを Segtree/FenwickTree に載せて O(log^2 N) で処理できるようにする
/// ## 使い方
/// let hld = Hld::new(&graph, root);
/// // hld.id[v] : 頂点vに対応する「オイラーツアー順」のインデックス（この順で1列に並べてSegtreeを構築する）
/// // hld.for_each_vertex_path(u, v, |l, r| { ... }) : パス u-v 上の頂点集合をカバーする区間 [l, r) を列挙
/// // hld.for_each_edge_path(u, v, |l, r| { ... })   : パス u-v 上の辺集合をカバーする区間 [l, r) を列挙（辺は子側の頂点idに対応させて管理する）
/// // hld.lca(u, v) : LCA
/// // hld.subtree_range(v) : 頂点vの部分木に対応する区間 [l, r)
struct Hld {
    par: Vec<usize>,
    depth: Vec<usize>,
    size: Vec<usize>,
    heavy: Vec<i64>, // heavy child (無ければ -1)
    head: Vec<usize>,
    pub id: Vec<usize>, // id[v] = オイラーツアー順でのインデックス
    rev_id: Vec<usize>, // rev_id[i] = idがiとなる頂点
}

impl Hld {
    fn new(graph: &[Vec<usize>], root: usize) -> Self {
        let n = graph.len();
        let mut par = vec![usize::MAX; n];
        let mut depth = vec![0usize; n];
        let mut size = vec![1usize; n];
        let mut heavy = vec![-1i64; n];

        // 1回目のDFS(帰りがけ順)でsize, heavy子を決定
        let mut order = vec![];
        let mut stack = vec![(root, usize::MAX, false)];
        while let Some((v, p, processed)) = stack.pop() {
            if processed {
                order.push(v);
                continue;
            }
            par[v] = p;
            stack.push((v, p, true));
            for &next_v in &graph[v] {
                if next_v != p {
                    depth[next_v] = depth[v] + 1;
                    stack.push((next_v, v, false));
                }
            }
        }
        for &v in &order {
            for &next_v in &graph[v] {
                if next_v != par[v] {
                    size[v] += size[next_v];
                    if heavy[v] == -1 || size[next_v] > size[heavy[v] as usize] {
                        heavy[v] = next_v as i64;
                    }
                }
            }
        }

        // 2回目のDFSでheadとidを決定（heavy pathを優先して先に潜る）
        let mut head = vec![0usize; n];
        let mut id = vec![0usize; n];
        let mut rev_id = vec![0usize; n];
        let mut cnt = 0usize;
        let mut stack2 = vec![(root, root)]; // (v, head)
        while let Some((v, h)) = stack2.pop() {
            head[v] = h;
            id[v] = cnt;
            rev_id[cnt] = v;
            cnt += 1;
            // heavy child以外を先にstackに積み、heavy childを最後に積む(popで先に処理させる)
            let mut lights = vec![];
            for &next_v in &graph[v] {
                if next_v != par[v] && heavy[v] != next_v as i64 {
                    lights.push(next_v);
                }
            }
            for &lv in &lights {
                stack2.push((lv, lv));
            }
            if heavy[v] != -1 {
                stack2.push((heavy[v] as usize, h));
            }
        }

        Self {
            par,
            depth,
            size,
            heavy,
            head,
            id,
            rev_id,
        }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.id[u] > self.id[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.par[self.head[v]];
        }
    }

    /// u-v パス上の頂点をカバーする [l, r) 区間（idベース、半開区間）を順に f に渡す
    fn for_each_vertex_path<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F) {
        loop {
            if self.id[u] > self.id[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                f(self.id[u], self.id[v] + 1);
                break;
            }
            f(self.id[self.head[v]], self.id[v] + 1);
            v = self.par[self.head[v]];
        }
    }

    /// u-v パス上の辺をカバーする [l, r) 区間を f に渡す（頂点区間から根側1つを除いたもの）
    fn for_each_edge_path<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F) {
        loop {
            if self.id[u] > self.id[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                if u != v {
                    f(self.id[u] + 1, self.id[v] + 1);
                }
                break;
            }
            f(self.id[self.head[v]], self.id[v] + 1);
            v = self.par[self.head[v]];
        }
    }

    /// 頂点vの部分木に対応する区間 [l, r)
    fn subtree_range(&self, v: usize) -> (usize, usize) {
        (self.id[v], self.id[v] + self.size[v])
    }
}

/// 全方位木DP（Rerooting DP, 木DPの根付け替え）
/// 全ての頂点を根としたときの木DPの値を まとめて O(N) で求める
/// ## 型パラメータ
/// * `M::S` - 部分木の集約値（モノイド）
/// * `merge(a, b)` - 2つの子部分木の集約値を結合する（結合的・可換であること）
/// * `add_root(x, v)` - 頂点vの寄与を集約値xに追加する（子の集約値 → 頂点vを含めた値）
/// 辺の重みは扱わない（必要なら add_root 内で頂点に重みを持たせるなどで対応する）
/// ## 使い方
/// let rerooting = Rerooting::new(n, &edges, identity, |a, b| ..., |acc, v| ...);
/// let ans: Vec<S> = rerooting.solve(); // ans[v] = vを根としたときの木DP値
struct Rerooting<S, F1, F2>
where
    S: Clone,
    F1: Fn(&S, &S) -> S,    // merge: 2つの子部分木の集約値を結合するモノイド演算
    F2: Fn(&S, usize) -> S, // add_root: 集約値に「自分自身」を足す変換 (頂点番号を受け取る)
{
    n: usize,
    graph: Vec<Vec<(usize, usize)>>, // (行き先, 辺index)
    identity: S,
    merge: F1,
    add_root: F2,
}

impl<S, F1, F2> Rerooting<S, F1, F2>
where
    S: Clone,
    F1: Fn(&S, &S) -> S,
    F2: Fn(&S, usize) -> S,
{
    /// edges: (u, v) のリスト（無向木）
    fn new(n: usize, edges: &[(usize, usize)], identity: S, merge: F1, add_root: F2) -> Self {
        let mut graph = vec![vec![]; n];
        for (i, &(u, v)) in edges.iter().enumerate() {
            graph[u].push((v, i));
            graph[v].push((u, i));
        }
        Self {
            n,
            graph,
            identity,
            merge,
            add_root,
        }
    }

    /// 全頂点を根としたときのDP値を返す
    fn solve(&self) -> Vec<S> {
        let n = self.n;
        if n == 0 {
            return vec![];
        }
        // 1. 頂点0を根として通常木DP（帰りがけ順を記録）
        let mut order = vec![];
        let mut par = vec![usize::MAX; n];
        let mut par_edge = vec![usize::MAX; n];
        let mut stack = vec![0usize];
        let mut visited = vec![false; n];
        visited[0] = true;
        while let Some(v) = stack.pop() {
            order.push(v);
            for &(next_v, ei) in &self.graph[v] {
                if !visited[next_v] {
                    visited[next_v] = true;
                    par[next_v] = v;
                    par_edge[next_v] = ei;
                    stack.push(next_v);
                }
            }
        }

        // dp_down[v] = 頂点vを根とした部分木(親方向を除く)のDP値
        let mut dp_down = vec![self.identity.clone(); n];
        for &v in order.iter().rev() {
            let mut acc = self.identity.clone();
            for &(next_v, _) in &self.graph[v] {
                if next_v != par[v] {
                    acc = (self.merge)(&acc, &dp_down[next_v]);
                }
            }
            dp_down[v] = (self.add_root)(&acc, v);
        }

        // dp_up[v] = 頂点vから見て「親方向（vを除いた残り全体）」のDP値
        let mut dp_up = vec![self.identity.clone(); n];
        let mut ans = vec![self.identity.clone(); n];
        for &v in order.iter() {
            let children: Vec<usize> = self.graph[v]
                .iter()
                .filter(|&&(nv, _)| nv != par[v])
                .map(|&(nv, _)| nv)
                .collect();
            let m = children.len();
            // 各子について「自分以外の子 + 親方向」の集約値を前後からの累積で求める(O(deg))
            let mut left = vec![self.identity.clone(); m + 1];
            let mut right = vec![self.identity.clone(); m + 1];
            for i in 0..m {
                left[i + 1] = (self.merge)(&left[i], &dp_down[children[i]]);
            }
            for i in (0..m).rev() {
                right[i] = (self.merge)(&right[i + 1], &dp_down[children[i]]);
            }
            // 頂点v自身を根としたときの答え = 全子 + 親方向 を集約して add_root
            let all_children = (self.merge)(&left[m], &dp_up[v]);
            ans[v] = (self.add_root)(&all_children, v);

            for i in 0..m {
                let without_i = (self.merge)(&left[i], &right[i + 1]);
                let up_for_child = (self.merge)(&without_i, &dp_up[v]);
                dp_up[children[i]] = (self.add_root)(&up_for_child, v);
            }
        }

        ans
    }
}

/// 重み付きUnion-Find
/// weight[x] = 「xの根からの相対値」を管理する
/// merge(x, y, w) は value[y] - value[x] = w となるようにマージする
/// 経路圧縮 + union by size を正しく実装（以前のバージョンにあった swap のバグを修正）
struct WeightedDsu {
    parent: Vec<i64>, // 負なら根(その場合は-サイズ), 非負なら親のインデックス
    weight: Vec<i64>, // 親との差分 (value[v] - value[parent[v]])
}

impl WeightedDsu {
    fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
            weight: vec![0; n],
        }
    }

    /// 根を求めつつ経路圧縮し、weight[x] を「根からの相対値」に更新する
    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let p = self.parent[x] as usize;
        let r = self.root(p);
        self.weight[x] += self.weight[p];
        self.parent[x] = r as i64;
        r
    }

    /// value[y] - value[x] （x, yが同じ木に属することが前提）
    fn diff(&mut self, x: usize, y: usize) -> i64 {
        self.root(x);
        self.root(y);
        self.weight[y] - self.weight[x]
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// value[y] - value[x] = w となるように merge する
    /// 矛盾があれば false を返す（マージはしない）
    fn merge(&mut self, x: usize, y: usize, w: i64) -> bool {
        let mut w = w;
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        if rx == ry {
            return self.weight[y] - self.weight[x] == w;
        }
        // value[y] - value[x] = w
        // value[ry] - value[rx] = w + weight[x] - weight[y]
        w += self.weight[x] - self.weight[y];
        // union by size: サイズが大きい方(絶対値が大きい方)にぶら下げる
        if self.parent[rx] > self.parent[ry] {
            // rx側のサイズが小さいので rx, ry を入れ替える (value[rx] - value[ry] = -w)
            swap(&mut rx, &mut ry);
            w = -w;
        }
        self.parent[rx] += self.parent[ry]; // サイズを加算
        self.parent[ry] = rx as i64;
        self.weight[ry] = w; // value[ry] - value[rx] = w
        true
    }
}

/// 2-SAT
/// 変数 x_0, ..., x_{n-1} (真偽値) に対する「または」節を追加していき、
/// 全ての節を満たす割り当てが存在するかどうかと、存在するならその割り当てを求める
/// 内部では 2*i を x_i が偽、2*i+1 を x_i が真 に対応させて SCC で処理する（ac_library::scc を使用）
/// ## 使い方
/// let mut sat = TwoSat::new(n);
/// sat.add_clause(i, true, j, false); // 「x_i is true」 or 「x_j is false」
/// if let Some(ans) = sat.solve() { ... } // ans[i] = x_i の真偽値
struct TwoSat {
    n: usize,
    scc: ac_library::SccGraph,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        Self {
            n,
            scc: ac_library::SccGraph::new(2 * n),
        }
    }

    /// 「x_i == f」 または 「x_j == g」を追加する (f, g は true/false)
    fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        // not(x_i == f) => (x_j == g) の対偶を2本の有向辺として追加
        self.scc
            .add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.scc
            .add_edge(2 * j + if g { 0 } else { 1 }, 2 * i + if f { 1 } else { 0 });
    }

    /// 充足可能なら各変数の真偽値の割り当てを返す。不可能なら None
    fn solve(self) -> Option<Vec<bool>> {
        let groups = self.scc.scc();
        let mut comp_id = vec![0usize; 2 * self.n];
        for (gi, g) in groups.iter().enumerate() {
            for &v in g {
                comp_id[v] = gi;
            }
        }
        let mut ans = vec![false; self.n];
        for i in 0..self.n {
            if comp_id[2 * i] == comp_id[2 * i + 1] {
                return None;
            }
            // ac_library::SccGraph::scc() は「u→vという辺があれば comp[u] <= comp[v]」となる
            // トポロジカル順でグループを返す。ノード 2*i+1 が「x_i = true」に対応し、
            // トポロジカル順で後ろにある方を真にすればよいので、comp[2*i+1] > comp[2*i] なら x_i = true。
            ans[i] = comp_id[2 * i + 1] > comp_id[2 * i];
        }
        Some(ans)
    }
}

// ------------------------------------------------------------
// Scc（強連結成分分解）使用例（ac_library）
// ------------------------------------------------------------
// let mut scc = ac_library::SccGraph::new(n);
// scc.add_edge(u, v);
// let groups: Vec<Vec<usize>> = scc.scc(); // トポロジカル順にグループ化された頂点集合

// ------------------------------------------------------------
// フロー（MfGraph / MinCostFlowGraph）使用例（ac_library）
// ------------------------------------------------------------
// use ac_library::MfGraph;
// let mut graph = MfGraph::new(n);
// graph.add_edge(from, to, capacity);
// let flow = graph.flow(s, t); // 最大流
//
// use ac_library::MinCostFlowGraph;
// let mut mcf = MinCostFlowGraph::new(n);
// mcf.add_edge(from, to, capacity, cost);
// let (flow, cost) = mcf.flow(s, t, flow_limit); // 最小費用流

// ------------------------------------------------------------
// Li Chao Tree（直線・線分の集合に対する最小値/最大値クエリ）
// ------------------------------------------------------------

/// Li Chao Tree（最小値クエリ版）
/// 直線 y = a*x + b を追加していき、任意のxに対する最小値を O(log W) で求める
/// 最大値が欲しい場合は a, b の符号を反転させて追加し、結果も反転させればよい
/// x の範囲は [x_min, x_max) を事前に決めて構築する（座標圧縮したxのindexで管理してもよい）
struct LiChaoTree {
    n: usize,
    xs: Vec<i64>,          // 各leafに対応するx座標（昇順）
    line: Vec<(i64, i64)>, // 各ノードに乗っている直線 (a, b) : y = a*x + b
    inf: i64,
}

impl LiChaoTree {
    fn new(xs: Vec<i64>) -> Self {
        let mut xs = xs;
        xs.sort();
        xs.dedup();
        let n = xs.len().max(1);
        Self {
            n,
            xs,
            line: vec![(0, i64::MAX / 2); 4 * n],
            inf: i64::MAX / 2,
        }
    }

    fn get_x(&self, i: usize) -> i64 {
        if i < self.xs.len() {
            self.xs[i]
        } else {
            self.xs[self.xs.len() - 1]
        }
    }

    fn eval(a: i64, b: i64, x: i64) -> i64 {
        a * x + b
    }

    fn add_line_rec(&mut self, node: usize, l: usize, r: usize, mut a: i64, mut b: i64) {
        let mid = (l + r) / 2;
        let xl = self.get_x(l);
        let xm = self.get_x(mid);

        let (mut cur_a, mut cur_b) = self.line[node];
        let mut left_better = Self::eval(a, b, xl) < Self::eval(cur_a, cur_b, xl);
        let mid_better = Self::eval(a, b, xm) < Self::eval(cur_a, cur_b, xm);

        if mid_better {
            std::mem::swap(&mut a, &mut cur_a);
            std::mem::swap(&mut b, &mut cur_b);
            self.line[node] = (cur_a, cur_b);
            left_better = Self::eval(a, b, xl) < Self::eval(cur_a, cur_b, xl);
        } else {
            self.line[node] = (cur_a, cur_b);
        }

        if l + 1 >= r {
            return;
        }
        if left_better {
            self.add_line_rec(node * 2 + 1, l, mid, a, b);
        } else {
            self.add_line_rec(node * 2 + 2, mid, r, a, b);
        }
    }

    /// 直線 y = a*x + b を追加する
    fn add_line(&mut self, a: i64, b: i64) {
        self.add_line_rec(0, 0, self.n, a, b);
    }

    /// x座標のインデックス（xsの何番目か）を指定してクエリする
    fn query_by_index(&self, i: usize) -> i64 {
        let x = self.get_x(i);
        let mut node = 0usize;
        let mut l = 0usize;
        let mut r = self.n;
        let mut res = self.inf;
        loop {
            let (a, b) = self.line[node];
            res = res.min(Self::eval(a, b, x));
            if l + 1 >= r {
                break;
            }
            let mid = (l + r) / 2;
            if i < mid {
                node = node * 2 + 1;
                r = mid;
            } else {
                node = node * 2 + 2;
                l = mid;
            }
        }
        res
    }

    /// 実際のx座標を指定してクエリする（構築時に渡した xs の中の値であること。無ければ panic）
    fn query(&self, x: i64) -> i64 {
        let i = self
            .xs
            .binary_search(&x)
            .expect("LiChaoTree::query: x が構築時の xs に含まれていません");
        self.query_by_index(i)
    }
}

// ------------------------------------------------------------
// Wavelet Matrix
// ------------------------------------------------------------

/// 簡易ビットベクトル（rank/selectをO(1) or O(log n)で提供）
struct BitVector {
    bits: Vec<u64>,
    rank_table: Vec<u32>, // rank_table[i] = bits[0..i*64) 内の1の個数
    n: usize,
}

impl BitVector {
    fn new(n: usize) -> Self {
        let words = n / 64 + 1;
        Self {
            bits: vec![0u64; words],
            rank_table: vec![0u32; words + 1],
            n,
        }
    }

    fn set(&mut self, i: usize) {
        self.bits[i / 64] |= 1u64 << (i % 64);
    }

    fn build(&mut self) {
        for i in 0..self.bits.len() {
            self.rank_table[i + 1] = self.rank_table[i] + self.bits[i].count_ones();
        }
    }

    fn access(&self, i: usize) -> bool {
        (self.bits[i / 64] >> (i % 64)) & 1 == 1
    }

    /// [0, i) 内の1の個数
    fn rank1(&self, i: usize) -> usize {
        let word = i / 64;
        let bit = i % 64;
        let mask = if bit == 0 { 0 } else { (1u64 << bit) - 1 };
        self.rank_table[word] as usize + (self.bits[word] & mask).count_ones() as usize
    }

    /// [0, i) 内の0の個数
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

/// Wavelet Matrix
/// 静的な非負整数列に対して以下を O(log(最大値)) で処理する:
///   - access(i)      : i番目の値
///   - rank(x, i)      : [0, i) の中に x が何個あるか
///   - kth_smallest(l, r, k) : [l, r) の中で k番目(0-indexed)に小さい値
///   - range_freq(l, r, lower, upper) : [l, r) の中で [lower, upper) に入る値の個数
/// 構築 O(n log(最大値))
struct WaveletMatrix {
    bit_len: usize,
    matrix: Vec<BitVector>,
    mid: Vec<usize>, // 各段で0が何個あったか
    len: usize,
}

impl WaveletMatrix {
    fn new(a: &[u64], bit_len: usize) -> Self {
        let n = a.len();
        let mut cur = a.to_vec();
        let mut matrix = Vec::with_capacity(bit_len);
        let mut mid = vec![0usize; bit_len];

        for level in (0..bit_len).rev() {
            let mut bv = BitVector::new(n);
            for i in 0..n {
                if (cur[i] >> level) & 1 == 1 {
                    bv.set(i);
                }
            }
            bv.build();
            // 安定に 0側, 1側 で分ける (stable partition)
            let mut zeros = Vec::with_capacity(n);
            let mut ones = Vec::with_capacity(n);
            for i in 0..n {
                if bv.access(i) {
                    ones.push(cur[i]);
                } else {
                    zeros.push(cur[i]);
                }
            }
            mid[bit_len - 1 - level] = zeros.len();
            zeros.extend(ones);
            cur = zeros;
            matrix.push(bv);
        }

        Self {
            bit_len,
            matrix,
            mid,
            len: n,
        }
    }

    /// i番目(0-indexed)の値
    fn access(&self, mut i: usize) -> u64 {
        let mut res = 0u64;
        for level in 0..self.bit_len {
            let bv = &self.matrix[level];
            let bit = bv.access(i);
            res = (res << 1) | (bit as u64);
            if bit {
                i = self.mid[level] + bv.rank1(i);
            } else {
                i = bv.rank0(i);
            }
        }
        res
    }

    /// [0, r) の中に値 x がいくつあるか
    fn rank(&self, x: u64, r: usize) -> usize {
        let (l, r) = self.rank_range(x, 0, r);
        r - l
    }

    // 内部用: [l, r) の中で値がxと一致する要素の、各段を降りた後の区間 [l', r') を返す
    fn rank_range(&self, x: u64, mut l: usize, mut r: usize) -> (usize, usize) {
        for level in 0..self.bit_len {
            let bv = &self.matrix[level];
            let bit = (x >> (self.bit_len - 1 - level)) & 1 == 1;
            if bit {
                l = self.mid[level] + bv.rank1(l);
                r = self.mid[level] + bv.rank1(r);
            } else {
                l = bv.rank0(l);
                r = bv.rank0(r);
            }
        }
        (l, r)
    }

    /// [l, r) の中で k番目 (0-indexed) に小さい値
    fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
        let mut res = 0u64;
        for level in 0..self.bit_len {
            let bv = &self.matrix[level];
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);
            let zero_cnt = r0 - l0;
            if k < zero_cnt {
                l = l0;
                r = r0;
                res <<= 1;
            } else {
                k -= zero_cnt;
                l = self.mid[level] + bv.rank1(l);
                r = self.mid[level] + bv.rank1(r);
                res = (res << 1) | 1;
            }
        }
        res
    }

    /// [l, r) の中で値が [lower, upper) に入る要素数
    fn range_freq(&self, l: usize, r: usize, lower: u64, upper: u64) -> usize {
        self.range_freq_rec(0, l, r, lower, upper, 0, 1u64 << self.bit_len)
    }

    fn range_freq_rec(
        &self,
        level: usize,
        l: usize,
        r: usize,
        lower: u64,
        upper: u64,
        cur_l: u64,
        cur_r: u64,
    ) -> usize {
        if r <= l || upper <= cur_l || cur_r <= lower {
            return 0;
        }
        if lower <= cur_l && cur_r <= upper {
            return r - l;
        }
        if level == self.bit_len {
            return 0;
        }
        let bv = &self.matrix[level];
        let l0 = bv.rank0(l);
        let r0 = bv.rank0(r);
        let l1 = self.mid[level] + bv.rank1(l);
        let r1 = self.mid[level] + bv.rank1(r);
        let mid_val = (cur_l + cur_r) / 2;
        self.range_freq_rec(level + 1, l0, r0, lower, upper, cur_l, mid_val)
            + self.range_freq_rec(level + 1, l1, r1, lower, upper, mid_val, cur_r)
    }
}

// ------------------------------------------------------------
// 使いますか？この構造体たち
// ------------------------------------------------------------

/// 遅延伝播・反転可能・乱択平衡二分探索木 (Implicit Treap / RBST)
///
/// 配列を「木構造」として保持し、以下をすべて O(log N) (期待値) で行える:
///   - 任意位置への要素の挿入 / 削除            ← セグ木にはできない
///   - 区間の一括更新 (作用素の適用, LazySegtreeのapply_rangeと同じ)
///   - 区間の反転 (reverse)                     ← セグ木にはできない
///   - 区間の集約値取得 (fold, LazySegtreeのprodと同じ)
///   - 木の分割 (split) / 結合 (merge) を基本操作として持つ
///
/// ac_library::MapMonoid と同じ考え方で「値のモノイド」と「作用」を1つの
/// トレイトにまとめている。追加で reverse() を要求する点だけが違う。
/// 値の集約(モノイド)と、区間への作用(マッピング)を定義するトレイト。
pub mod treap {
    use std::mem::swap;

    pub trait TMapMonoid {
        /// 集約される値の型 (区間の集約結果)
        type S: Clone;
        /// 区間に適用する作用の型
        type F: Clone;

        /// Sの単位元
        fn identity() -> Self::S;
        /// Sの二項演算 (結合律を満たすこと)
        fn op(a: &Self::S, b: &Self::S) -> Self::S;
        /// 区間反転時に集約値Sをどう変換するか。
        /// 和・min・max・xorなど「可換」な演算はデフォルト実装(そのまま返す)でよい。
        /// 文字列連結・行列積など「非可換」な演算では左右を入れ替える処理を書く。
        fn reverse(x: &Self::S) -> Self::S {
            x.clone()
        }

        /// Fの単位元 (何もしない作用)
        fn identity_map() -> Self::F;
        /// Sに作用Fを適用する
        fn mapping(f: &Self::F, x: &Self::S) -> Self::S;
        /// 作用の合成: 「先にgを適用、後にfを適用」した結果と同じ効果になるようにする
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }

    // --- 優先度生成用の乱数 (xorshift64) ---
    struct Xorshift(u64);
    impl Xorshift {
        fn new() -> Self {
            Xorshift(0x2545_F491_4F6C_DD1D)
        }
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
    }

    struct Node<M: TMapMonoid> {
        val: M::S,  // このノード単体の値
        acc: M::S,  // 部分木全体の集約値
        lazy: M::F, // 未反映の作用
        rev: bool,  // 反転フラグ
        priority: u64,
        size: usize,
        left: Option<Box<Node<M>>>,
        right: Option<Box<Node<M>>>,
    }

    impl<M: TMapMonoid> Node<M> {
        fn new(val: M::S, priority: u64) -> Self {
            Node {
                acc: val.clone(),
                val,
                lazy: M::identity_map(),
                rev: false,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    fn size<M: TMapMonoid>(node: &Option<Box<Node<M>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn acc<M: TMapMonoid>(node: &Option<Box<Node<M>>>) -> M::S {
        node.as_ref().map_or(M::identity(), |n| n.acc.clone())
    }

    /// 子を更新した後、部分木サイズ・集約値を再計算する
    fn update<M: TMapMonoid>(node: &mut Box<Node<M>>) {
        node.size = 1 + size::<M>(&node.left) + size::<M>(&node.right);
        node.acc = M::op(
            &M::op(&acc::<M>(&node.left), &node.val),
            &acc::<M>(&node.right),
        );
    }

    /// 未反映の反転フラグ・作用を子に伝播する
    fn push_down<M: TMapMonoid>(node: &mut Box<Node<M>>) {
        if node.rev {
            swap(&mut node.left, &mut node.right);
            if let Some(l) = node.left.as_mut() {
                l.rev ^= true;
                l.val = M::reverse(&l.val);
                l.acc = M::reverse(&l.acc);
            }
            if let Some(r) = node.right.as_mut() {
                r.rev ^= true;
                r.val = M::reverse(&r.val);
                r.acc = M::reverse(&r.acc);
            }
            node.rev = false;
        }
        let f = node.lazy.clone();
        node.lazy = M::identity_map();
        if let Some(l) = node.left.as_mut() {
            apply_node::<M>(l, &f);
        }
        if let Some(r) = node.right.as_mut() {
            apply_node::<M>(r, &f);
        }
    }

    fn apply_node<M: TMapMonoid>(node: &mut Box<Node<M>>, f: &M::F) {
        node.val = M::mapping(f, &node.val);
        node.acc = M::mapping(f, &node.acc);
        node.lazy = M::composition(f, &node.lazy);
    }

    /// 木を先頭k個 [0,k) と残り [k,size) に分割する
    fn split<M: TMapMonoid>(
        node: Option<Box<Node<M>>>,
        k: usize,
    ) -> (Option<Box<Node<M>>>, Option<Box<Node<M>>>) {
        match node {
            None => (None, None),
            Some(mut n) => {
                push_down::<M>(&mut n);
                let ls = size::<M>(&n.left);
                if k <= ls {
                    let (l, r) = split::<M>(n.left.take(), k);
                    n.left = r;
                    update::<M>(&mut n);
                    (l, Some(n))
                } else {
                    let (l, r) = split::<M>(n.right.take(), k - ls - 1);
                    n.right = l;
                    update::<M>(&mut n);
                    (Some(n), r)
                }
            }
        }
    }

    /// 2つの木を結合する (左木の全要素が右木より前という前提)
    fn merge<M: TMapMonoid>(
        l: Option<Box<Node<M>>>,
        r: Option<Box<Node<M>>>,
    ) -> Option<Box<Node<M>>> {
        match (l, r) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.priority > r.priority {
                    push_down::<M>(&mut l);
                    l.right = merge::<M>(l.right.take(), Some(r));
                    update::<M>(&mut l);
                    Some(l)
                } else {
                    push_down::<M>(&mut r);
                    r.left = merge::<M>(Some(l), r.left.take());
                    update::<M>(&mut r);
                    Some(r)
                }
            }
        }
    }

    pub struct RBST<M: TMapMonoid> {
        root: Option<Box<Node<M>>>,
        rng: Xorshift,
    }

    impl<M: TMapMonoid> RBST<M> {
        pub fn new() -> Self {
            RBST {
                root: None,
                rng: Xorshift::new(),
            }
        }

        /// 配列から構築する O(N log N)
        pub fn build(v: Vec<M::S>) -> Self {
            let mut t = Self::new();
            for x in v {
                let n = t.len();
                t.insert(n, x);
            }
            t
        }

        pub fn len(&self) -> usize {
            size::<M>(&self.root)
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// 位置pに値xを挿入する (0 <= p <= len())
        pub fn insert(&mut self, p: usize, x: M::S) {
            let new_node = Some(Box::new(Node::new(x, self.rng.next())));
            let root = self.root.take();
            let (l, r) = split::<M>(root, p);
            self.root = merge::<M>(merge::<M>(l, new_node), r);
        }

        /// 位置pの要素を削除する
        pub fn erase(&mut self, p: usize) {
            let root = self.root.take();
            let (l, r) = split::<M>(root, p);
            let (_, r) = split::<M>(r, 1);
            self.root = merge::<M>(l, r);
        }

        /// 位置pの値を取得する
        pub fn get(&mut self, p: usize) -> M::S {
            let root = self.root.take();
            let (l, mid_r) = split::<M>(root, p);
            let (mid, r) = split::<M>(mid_r, 1);
            let val = mid.as_ref().unwrap().val.clone();
            self.root = merge::<M>(merge::<M>(l, mid), r);
            val
        }

        /// 位置pの値をxに設定する
        pub fn set(&mut self, p: usize, x: M::S) {
            let root = self.root.take();
            let (l, mid_r) = split::<M>(root, p);
            let (_mid, r) = split::<M>(mid_r, 1);
            let new_mid = Some(Box::new(Node::new(x, self.rng.next())));
            self.root = merge::<M>(merge::<M>(l, new_mid), r);
        }

        /// 区間[l, r)の集約値を取得する
        pub fn fold(&mut self, l: usize, r: usize) -> M::S {
            let root = self.root.take();
            let (a, bc) = split::<M>(root, l);
            let (b, c) = split::<M>(bc, r - l);
            let result = acc::<M>(&b);
            self.root = merge::<M>(merge::<M>(a, b), c);
            result
        }

        /// 区間[l, r)に作用fを適用する
        pub fn apply(&mut self, l: usize, r: usize, f: M::F) {
            let root = self.root.take();
            let (a, bc) = split::<M>(root, l);
            let (mut b, c) = split::<M>(bc, r - l);
            if let Some(node) = b.as_mut() {
                apply_node::<M>(node, &f);
            }
            self.root = merge::<M>(merge::<M>(a, b), c);
        }

        /// 区間[l, r)を反転する
        pub fn reverse(&mut self, l: usize, r: usize) {
            let root = self.root.take();
            let (a, bc) = split::<M>(root, l);
            let (mut b, c) = split::<M>(bc, r - l);
            if let Some(node) = b.as_mut() {
                node.rev ^= true;
                node.val = M::reverse(&node.val);
                node.acc = M::reverse(&node.acc);
            }
            self.root = merge::<M>(merge::<M>(a, b), c);
        }

        /// 配列として復元する (デバッグ・出力用)
        pub fn to_vec(&mut self) -> Vec<M::S> {
            let mut result = Vec::with_capacity(self.len());
            let root = self.root.take();
            self.root = Self::collect(root, &mut result);
            result
        }

        fn collect(node: Option<Box<Node<M>>>, out: &mut Vec<M::S>) -> Option<Box<Node<M>>> {
            match node {
                None => None,
                Some(mut n) => {
                    push_down::<M>(&mut n);
                    n.left = Self::collect(n.left.take(), out);
                    out.push(n.val.clone());
                    n.right = Self::collect(n.right.take(), out);
                    update::<M>(&mut n);
                    Some(n)
                }
            }
        }
    }
}
use treap::{TMapMonoid, RBST};

// ========================= 使用例 =========================

// 例1: 区間加算 + 区間和 (可換なのでreverseはデフォルト実装のままでよい)
// ※ 遅延セグ木用の RangeAddRangeSum と名前が被るので、使うときは名前を変えること
/*
struct RangeAddRangeSum;
impl TMapMonoid for RangeAddRangeSum {
    type S = (i64, i64); // (合計, 要素数)
    type F = i64;

    fn identity() -> Self::S {
        (0, 0)
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
    // fn reverse(x: &Self::S) -> Self::S { x.clone() } // デフォルトのままでOK(和は可換)

    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &Self::S) -> Self::S {
        (x.0 + f * x.1, x.1)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    let init: Vec<(i64, i64)> = vec![1, 2, 3, 4, 5].into_iter().map(|v| (v, 1)).collect();
    let mut t = RBST::<RangeAddRangeSum>::build(init);
    // 現在の配列: [1, 2, 3, 4, 5]

    t.insert(2, (100, 1)); // 位置2に100を挿入
    println!("{:?}", t.to_vec()); // [(1,1),(2,1),(100,1),(3,1),(4,1),(5,1)]

    t.erase(0); // 先頭を削除
    println!("{:?}", t.to_vec()); // [(2,1),(100,1),(3,1),(4,1),(5,1)]

    t.apply(0, 2, 10); // 区間[0,2)に+10
    println!("{:?}", t.to_vec()); // [(12,1),(110,1),(3,1),(4,1),(5,1)]

    t.reverse(1, 4); // 区間[1,4)を反転
    println!("{:?}", t.to_vec()); // [(12,1),(4,1),(3,1),(110,1),(5,1)]

    let (sum, _len) = t.fold(0, t.len());
    println!("total sum = {}", sum);
}
*/
