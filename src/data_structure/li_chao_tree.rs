// ------------------------------------------------------------
// Li Chao Tree（直線・線分の集合に対する最小値/最大値クエリ）
// ------------------------------------------------------------

/// Li Chao Tree（最小値クエリ版）
/// 直線 y = a*x + b を追加していき、任意のxに対する最小値を O(log W) で求める
/// 最大値が欲しい場合は a, b の符号を反転させて追加し、結果も反転させればよい
/// x の範囲は [x_min, x_max) を事前に決めて構築する（座標圧縮したxのindexで管理してもよい）
pub struct LiChaoTree {
    pub n: usize,
    pub xs: Vec<i64>,          // 各leafに対応するx座標（昇順）
    pub line: Vec<(i64, i64)>, // 各ノードに乗っている直線 (a, b) : y = a*x + b
    pub inf: i64,
}

impl LiChaoTree {
    pub fn new(xs: Vec<i64>) -> Self {
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

    pub fn get_x(&self, i: usize) -> i64 {
        if i < self.xs.len() {
            self.xs[i]
        } else {
            self.xs[self.xs.len() - 1]
        }
    }

    pub fn eval(a: i64, b: i64, x: i64) -> i64 {
        a * x + b
    }

    pub fn add_line_rec(&mut self, node: usize, l: usize, r: usize, mut a: i64, mut b: i64) {
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
    pub fn add_line(&mut self, a: i64, b: i64) {
        self.add_line_rec(0, 0, self.n, a, b);
    }

    /// x座標のインデックス（xsの何番目か）を指定してクエリする
    pub fn query_by_index(&self, i: usize) -> i64 {
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
    pub fn query(&self, x: i64) -> i64 {
        let i = self
            .xs
            .binary_search(&x)
            .expect("LiChaoTree::query: x が構築時の xs に含まれていません");
        self.query_by_index(i)
    }
}
