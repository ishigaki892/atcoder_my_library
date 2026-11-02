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
pub struct BitLowerBound {
    pub n: usize,
    pub data: Vec<i64>, // 1-indexed
    pub log: usize,
}

impl BitLowerBound {
    pub fn new(n: usize) -> Self {
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
    pub fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    /// a[0] + ... + a[i-1] (0-indexed prefix sum, [0, i))
    pub fn sum(&self, i: usize) -> i64 {
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
    pub fn lower_bound(&self, w: i64) -> usize {
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
pub struct Bit2D {
    pub h: usize,
    pub w: usize,
    pub data: Vec<Vec<i64>>, // 1-indexed
}

impl Bit2D {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            data: vec![vec![0; w + 1]; h + 1],
        }
    }

    /// a[i][j] += x (0-indexed)
    pub fn add(&mut self, i: usize, j: usize, x: i64) {
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
    pub fn prefix_sum(&self, i: usize, j: usize) -> i64 {
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
    pub fn sum(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
        self.prefix_sum(r2, c2) - self.prefix_sum(r1, c2) - self.prefix_sum(r2, c1)
            + self.prefix_sum(r1, c1)
    }
}
