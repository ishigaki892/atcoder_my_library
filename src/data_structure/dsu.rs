use std::mem::swap;

/// 重み付きUnion-Find
/// weight[x] = 「xの根からの相対値」を管理する
/// merge(x, y, w) は value[y] - value[x] = w となるようにマージする
/// 経路圧縮 + union by size を正しく実装（以前のバージョンにあった swap のバグを修正）
pub struct WeightedDsu {
    pub parent: Vec<i64>, // 負なら根(その場合は-サイズ), 非負なら親のインデックス
    pub weight: Vec<i64>, // 親との差分 (value[v] - value[parent[v]])
}

impl WeightedDsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
            weight: vec![0; n],
        }
    }

    /// 根を求めつつ経路圧縮し、weight[x] を「根からの相対値」に更新する
    pub fn root(&mut self, x: usize) -> usize {
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
    pub fn diff(&mut self, x: usize, y: usize) -> i64 {
        self.root(x);
        self.root(y);
        self.weight[y] - self.weight[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// value[y] - value[x] = w となるように merge する
    /// 矛盾があれば false を返す（マージはしない）
    pub fn merge(&mut self, x: usize, y: usize, w: i64) -> bool {
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

/// Union-Find（自作版。基本は ac_library::Dsu を使えばよい）
pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
    pub n: usize,
}

impl UnionFind {
    /// n頂点0辺の無向グラフを作成
    /// ### 計算量 O(N)
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            n,
        }
    }

    /// 頂点aの代表元を返す
    /// ### 計算量 ならし O(α(N))
    pub fn leader(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            a
        } else {
            self.parent[a] = self.leader(self.parent[a]);
            self.parent[a]
        }
    }

    /// 辺(a, b)を追加する
    /// 連結だった場合はその代表元、非連結だった場合は新たな代表元を返す
    /// ### 計算量 ならし O(α(N))
    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let pa = self.leader(a);
        let pb = self.leader(b);
        if pa == pb {
            return pa;
        }
        let (large, small) = if self.size[pa] >= self.size[pb] {
            (pa, pb)
        } else {
            (pb, pa)
        };
        self.parent[small] = large;
        self.size[large] += self.size[small];
        large
    }

    /// 頂点aとbが連結かどうかを返す
    /// ### 計算量 ならし O(α(N))
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }

    /// 頂点aの属する連結成分のサイズを返す
    /// ### 計算量 ならし O(α(N))
    pub fn size(&mut self, a: usize) -> usize {
        let la = self.leader(a);
        self.size[la]
    }

    /// グラフを連結成分に分けて返す
    /// 戻り値は「一つの連結成分の頂点番号リスト」のリスト
    /// ### 計算量 O(N)
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_to_idx = vec![self.n; self.n];
        let mut idx = 0;
        let mut result: Vec<Vec<usize>> = vec![];

        for i in 0..self.n {
            let l = self.leader(i);
            if leader_to_idx[l] == self.n {
                leader_to_idx[l] = idx;
                idx += 1;
                result.push(vec![]);
            }
            result[leader_to_idx[l]].push(i);
        }
        result
    }
}
