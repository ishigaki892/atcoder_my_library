/// 2-SAT
/// 変数 x_0, ..., x_{n-1} (真偽値) に対する「または」節を追加していき、
/// 全ての節を満たす割り当てが存在するかどうかと、存在するならその割り当てを求める
/// 内部では 2*i を x_i が偽、2*i+1 を x_i が真 に対応させて SCC で処理する（ac_library::scc を使用）
/// ## 使い方
/// let mut sat = TwoSat::new(n);
/// sat.add_clause(i, true, j, false); // 「x_i is true」 or 「x_j is false」
/// if let Some(ans) = sat.solve() { ... } // ans[i] = x_i の真偽値
pub struct TwoSat {
    pub n: usize,
    pub scc: ac_library::SccGraph,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            scc: ac_library::SccGraph::new(2 * n),
        }
    }

    /// 「x_i == f」 または 「x_j == g」を追加する (f, g は true/false)
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        // not(x_i == f) => (x_j == g) の対偶を2本の有向辺として追加
        self.scc
            .add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.scc
            .add_edge(2 * j + if g { 0 } else { 1 }, 2 * i + if f { 1 } else { 0 });
    }

    /// 充足可能なら各変数の真偽値の割り当てを返す。不可能なら None
    pub fn solve(self) -> Option<Vec<bool>> {
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
