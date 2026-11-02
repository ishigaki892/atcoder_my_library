//! 重み付き隣接リストを包んだグラフ構造体（rust.json の「グラフ構造体」スニペット由来）
//! 各アルゴリズムは `graph::basic` の自由関数に委譲している

use super::basic;

pub struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<(usize, i64)>>,
    pub h: u64,
    pub directed: bool,
}

impl Graph {
    /// グラフを新規作成
    /// ## 入力
    /// * `n` - 頂点数
    /// * `directed` - 有向/無向グラフ(有向グラフなら`true`)
    /// ### 計算量 O(N)
    pub fn new(n: usize, directed: bool) -> Self {
        Self {
            n,
            edges: vec![vec![]; n],
            h: 0,
            directed,
        }
    }

    /// 頂点aからbに辺を張る
    /// ## 入力
    /// * `a` `b` - 頂点
    /// * `x` - コスト
    /// ### 計算量 O(1)
    pub fn add(&mut self, a: usize, b: usize, x: i64) {
        self.edges[a].push((b, x));
        if !self.directed {
            self.edges[b].push((a, x));
        }
        self.h += 1;
    }

    /// グラフの辺の総数
    /// ### 計算量 O(1)
    pub fn edge_cnt(&self) -> u64 {
        self.h
    }

    /// 頂点aの次数（隣接辺の数）
    /// ### 計算量 O(1)
    pub fn degree(&self, a: usize) -> usize {
        self.edges[a].len()
    }

    /// 頂点aの隣接頂点リスト
    /// ## 出力
    /// 隣接頂点がある場合`Some(&Vec)` ない場合`None`
    /// ### 計算量 O(1)
    pub fn get(&self, a: usize) -> Option<&Vec<(usize, i64)>> {
        if !self.edges[a].is_empty() {
            Some(&self.edges[a])
        } else {
            None
        }
    }

    /// 頂点sからのオイラーツアー（頂点の訪問順列） O(N+M)
    pub fn euler_tour(&self, s: usize) -> Vec<usize> {
        basic::euler_tour(&self.edges, s)
    }

    /// BFSによる頂点sから各頂点への最短ステップ数（到達不可能なら`-1`） O(N+M)
    pub fn bfs(&self, s: usize) -> Vec<i64> {
        basic::bfs(&self.edges, s)
    }

    /// 01-BFS（コストが0か1のみ） O(N+M)
    pub fn bfs01(&self, s: usize) -> Vec<i64> {
        basic::bfs01(&self.edges, s)
    }

    /// ダイクストラ法（非負コストのみ、到達不可能なら`i64::MAX`） O((N+M) log N)
    pub fn djk(&self, s: usize) -> Vec<i64> {
        basic::djk(&self.edges, s)
    }

    /// サイクル検出（有向・無向を自動判定）
    /// ## 出力
    /// サイクルがあるなら`true`
    /// ### 計算量 O(N+M)
    pub fn cycle(&self) -> bool {
        if self.directed {
            self.cycle_directed()
        } else {
            self.cycle_undirected()
        }
    }

    pub fn cycle_undirected(&self) -> bool {
        fn has_cycle(
            v: usize,
            parent: usize,
            g: &Vec<Vec<(usize, i64)>>,
            seen: &mut Vec<bool>,
        ) -> bool {
            seen[v] = true;
            for &(next_v, _) in &g[v] {
                if next_v == parent {
                    continue;
                }
                if seen[next_v] || has_cycle(next_v, v, g, seen) {
                    return true;
                }
            }
            false
        }
        let mut seen = vec![false; self.n];
        for s in 0..self.n {
            if !seen[s] && has_cycle(s, s, &self.edges, &mut seen) {
                return true;
            }
        }
        false
    }

    pub fn cycle_directed(&self) -> bool {
        fn has_cycle(
            v: usize,
            g: &Vec<Vec<(usize, i64)>>,
            seen: &mut Vec<bool>,
            in_stack: &mut Vec<bool>,
        ) -> bool {
            seen[v] = true;
            in_stack[v] = true;
            for &(next_v, _) in &g[v] {
                if !seen[next_v] && has_cycle(next_v, g, seen, in_stack) {
                    return true;
                } else if in_stack[next_v] {
                    return true;
                }
            }
            in_stack[v] = false;
            false
        }
        let mut seen = vec![false; self.n];
        let mut in_stack = vec![false; self.n];
        for s in 0..self.n {
            if !seen[s] && has_cycle(s, &self.edges, &mut seen, &mut in_stack) {
                return true;
            }
        }
        false
    }

    /// ワーシャルフロイド法（到達不可能なら`i64::MAX/2`） O(N^3)
    pub fn warshall(&self) -> Vec<Vec<i64>> {
        basic::warshall(&self.edges)
    }

    /// ベルマンフォード法 (距離, 負閉路の有無) O(NM)
    pub fn bellman(&self, s: usize) -> (Vec<i64>, bool) {
        basic::bellman(&self.edges, s)
    }

    /// クラスカル法による最小全域木 (合計コスト, 辺リスト)。非連結なら None。O(M log M)
    pub fn kruskal(&self) -> Option<(i64, Vec<(usize, usize, i64)>)> {
        basic::kruskal(&self.edges)
    }

    /// プリム法による最小全域木のコスト。非連結なら None。O((N+M) log N)
    pub fn prim(&self) -> Option<i64> {
        basic::prim(&self.edges)
    }

    /// 二部グラフ判定（無向グラフ用） O(N+M)
    pub fn is_bipartite(&self) -> bool {
        debug_assert!(!self.directed, "is_bipartiteは無向グラフ専用です");
        basic::is_bipartite(&self.edges)
    }

    /// 連結成分分解（無向グラフ用） O(N+M)
    pub fn conn_comp(&self) -> Vec<Vec<usize>> {
        debug_assert!(!self.directed, "conn_compは無向グラフ専用です");
        basic::conn_comp(&self.edges)
    }

    /// トポロジカルソート（有向グラフ用）。サイクルがあれば None。O(N+M)
    pub fn topo(&self) -> Option<Vec<usize>> {
        debug_assert!(self.directed, "topoは有向グラフ専用です");
        basic::topo(&self.edges)
    }

    /// 木の直径（最も遠い2頂点間のステップ数） O(N+M)
    pub fn tree_diameter(&self) -> i64 {
        basic::tree_diameter(&self.edges)
    }
}
