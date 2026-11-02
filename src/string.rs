use crate::prelude::*;
use std::collections::VecDeque;

/// Z algorithm: z[i] = s と s[i..] の最長共通接頭辞長 (z[0]=s.len())
pub fn z_algorithm(s: &[char]) -> Vec<usize> {
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
pub struct Kmp {
    pub pattern: Vec<char>,
    pub fail: Vec<usize>,
}

impl Kmp {
    pub fn new(pattern: &[char]) -> Self {
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
    pub fn search(&self, text: &[char]) -> Vec<usize> {
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
pub fn palindrome_len(r: &[usize], i: usize) -> usize {
    if (i + r[i]) % 2 == 0 {
        r[i] + 1
    } else {
        r[i]
    }
}

pub fn manacher<T: PartialEq + Clone>(s: &[T]) -> Vec<usize> {
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
pub struct Trie {
    pub children: Vec<[i32; 26]>,
    pub is_end: Vec<bool>,
    pub count: Vec<usize>, // このノードを通過した単語数（prefixカウント用）
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: vec![[-1; 26]],
            is_end: vec![false],
            count: vec![0],
        }
    }

    pub fn insert(&mut self, s: &[u8]) {
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

    pub fn contains(&self, s: &[u8]) -> bool {
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
    pub fn count_prefix(&self, s: &[u8]) -> usize {
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
pub struct AhoCorasick {
    pub children: Vec<[i32; 26]>,
    pub fail: Vec<usize>,
    // このノードで終了するパターンのindex一覧（build後はfail先の分も含む）
    pub outputs: Vec<Vec<usize>>,
    pub pattern_cnt: usize,
}

impl AhoCorasick {
    pub fn new() -> Self {
        Self {
            children: vec![[-1; 26]],
            fail: vec![0],
            outputs: vec![vec![]],
            pattern_cnt: 0,
        }
    }

    /// パターンを追加し、そのパターンindex（追加した順に 0, 1, 2, ...）を返す
    pub fn add_pattern(&mut self, s: &[u8]) -> usize {
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
    pub fn build(&mut self) {
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
    pub fn search(&self, text: &[u8]) -> Vec<(usize, usize)> {
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
    pub fn count_matches(&self, text: &[u8]) -> i64 {
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
pub fn suffix_array(s: &[u8]) -> Vec<usize> {
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
pub fn lcp_array(s: &[u8], sa: &[usize]) -> Vec<usize> {
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
pub struct DoubleRollingHash {
    pub hash1: Vec<u64>,
    pub hash2: Vec<u64>,
    pub pow1: Vec<u64>,
    pub pow2: Vec<u64>,
    pub bases: (u64, u64),
}

impl DoubleRollingHash {
    pub const MOD1: u64 = 1_000_000_007;
    pub const MOD2: u64 = 998_244_353;

    pub fn new(s: &[u8]) -> Self {
        let r = random_u64();
        let bases = (
            (r & 0xFFFF_FFFF) % (Self::MOD1 - 1000) + 500,
            (r >> 32) % (Self::MOD2 - 1000) + 500,
        );
        Self::with_base(s, bases)
    }

    pub fn with_base(s: &[u8], bases: (u64, u64)) -> Self {
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
    pub fn get(&self, l: usize, r: usize) -> (u64, u64) {
        let h1 = (self.hash1[r] + Self::MOD1 - self.hash1[l] * self.pow1[r - l] % Self::MOD1)
            % Self::MOD1;
        let h2 = (self.hash2[r] + Self::MOD2 - self.hash2[l] * self.pow2[r - l] % Self::MOD2)
            % Self::MOD2;
        (h1, h2)
    }
}

/// Rolling Hash（mod 2^61-1, 基数は実行ごとにランダム）
/// 単一mod 1e9 程度だと 10^5 個の比較で普通に衝突するため、2^61-1 を使う
/// let rh = RollingHash::new(s.as_bytes());
/// rh.get_hash(l, r) // s[l..r) のハッシュ
/// 2つの文字列を比較するときは RollingHash::with_base(t, rh.base) で同じ基数を使うこと
pub struct RollingHash {
    pub hash: Vec<u64>,
    pub power: Vec<u64>,
    pub base: u64,
}

impl RollingHash {
    pub const MOD: u64 = (1u64 << 61) - 1;

    pub fn mul(a: u64, b: u64) -> u64 {
        let t = a as u128 * b as u128;
        let t = (t >> 61) as u64 + (t as u64 & Self::MOD);
        if t >= Self::MOD {
            t - Self::MOD
        } else {
            t
        }
    }

    pub fn new(s: &[u8]) -> Self {
        let base = random_u64() % (Self::MOD - 1000) + 500;
        Self::with_base(s, base)
    }

    pub fn with_base(s: &[u8], base: u64) -> Self {
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
    pub fn get_hash(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + Self::MOD - Self::mul(self.hash[l], self.power[r - l])) % Self::MOD
    }
}
