use std::mem::swap;

/// LCA（ダブリングによる最小共通祖先）
/// ## 使い方
/// let lca = Lca::new(&graph, root);
/// lca.query(u, v)     -> uとvのLCA
/// lca.dist(u, v)      -> uとv間の辺数（非重み距離）
/// ### 構築 O(N log N)、クエリ O(log N)
pub struct Lca {
    pub depth: Vec<i64>,
    pub parent: Vec<Vec<usize>>, // parent[k][v] = vから2^k個祖先を遡った頂点
    pub log: usize,
}

impl Lca {
    pub fn new(graph: &[Vec<(usize, i64)>], root: usize) -> Self {
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

    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
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

    pub fn dist(&self, u: usize, v: usize) -> i64 {
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
pub struct Hld {
    pub par: Vec<usize>,
    pub depth: Vec<usize>,
    pub size: Vec<usize>,
    pub heavy: Vec<i64>, // heavy child (無ければ -1)
    pub head: Vec<usize>,
    pub id: Vec<usize>,     // id[v] = オイラーツアー順でのインデックス
    pub rev_id: Vec<usize>, // rev_id[i] = idがiとなる頂点
}

impl Hld {
    pub fn new(graph: &[Vec<usize>], root: usize) -> Self {
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

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
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
    pub fn for_each_vertex_path<F: FnMut(usize, usize)>(
        &self,
        mut u: usize,
        mut v: usize,
        mut f: F,
    ) {
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
    pub fn for_each_edge_path<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F) {
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
    pub fn subtree_range(&self, v: usize) -> (usize, usize) {
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
pub struct Rerooting<S, F1, F2>
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
    pub fn new(n: usize, edges: &[(usize, usize)], identity: S, merge: F1, add_root: F2) -> Self {
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
    pub fn solve(&self) -> Vec<S> {
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
