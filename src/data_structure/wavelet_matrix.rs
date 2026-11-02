// ------------------------------------------------------------
// Wavelet Matrix
// ------------------------------------------------------------

/// 簡易ビットベクトル（rank/selectをO(1) or O(log n)で提供）
pub struct BitVector {
    pub bits: Vec<u64>,
    pub rank_table: Vec<u32>, // rank_table[i] = bits[0..i*64) 内の1の個数
    pub n: usize,
}

impl BitVector {
    pub fn new(n: usize) -> Self {
        let words = n / 64 + 1;
        Self {
            bits: vec![0u64; words],
            rank_table: vec![0u32; words + 1],
            n,
        }
    }

    pub fn set(&mut self, i: usize) {
        self.bits[i / 64] |= 1u64 << (i % 64);
    }

    pub fn build(&mut self) {
        for i in 0..self.bits.len() {
            self.rank_table[i + 1] = self.rank_table[i] + self.bits[i].count_ones();
        }
    }

    pub fn access(&self, i: usize) -> bool {
        (self.bits[i / 64] >> (i % 64)) & 1 == 1
    }

    /// [0, i) 内の1の個数
    pub fn rank1(&self, i: usize) -> usize {
        let word = i / 64;
        let bit = i % 64;
        let mask = if bit == 0 { 0 } else { (1u64 << bit) - 1 };
        self.rank_table[word] as usize + (self.bits[word] & mask).count_ones() as usize
    }

    /// [0, i) 内の0の個数
    pub fn rank0(&self, i: usize) -> usize {
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
pub struct WaveletMatrix {
    pub bit_len: usize,
    pub matrix: Vec<BitVector>,
    pub mid: Vec<usize>, // 各段で0が何個あったか
    pub len: usize,
}

impl WaveletMatrix {
    pub fn new(a: &[u64], bit_len: usize) -> Self {
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
    pub fn access(&self, mut i: usize) -> u64 {
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
    pub fn rank(&self, x: u64, r: usize) -> usize {
        let (l, r) = self.rank_range(x, 0, r);
        r - l
    }

    // 内部用: [l, r) の中で値がxと一致する要素の、各段を降りた後の区間 [l', r') を返す
    pub fn rank_range(&self, x: u64, mut l: usize, mut r: usize) -> (usize, usize) {
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
    pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
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
    pub fn range_freq(&self, l: usize, r: usize, lower: u64, upper: u64) -> usize {
        self.range_freq_rec(0, l, r, lower, upper, 0, 1u64 << self.bit_len)
    }

    pub fn range_freq_rec(
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
