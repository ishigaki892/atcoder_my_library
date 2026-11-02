use std::collections::VecDeque;

// ------------------------------------------------------------
// グラフ系
// ------------------------------------------------------------

/// 再帰DFS(グラフ)
pub fn dfs(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
    seen[v] = true;

    for &next_v in &g[v] {
        if seen[next_v] {
            continue;
        }

        dfs(next_v, g, seen);
    }
}

/// 再帰DFS(迷路)
pub fn dfs2(
    y: usize,
    x: usize,
    h: usize,
    w: usize,
    maze: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
) {
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
pub fn has_cycle(v: usize, parent: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> bool {
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
pub fn has_cycle2(v: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<u8>) -> bool {
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
pub fn euler_tour(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<usize> {
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
pub fn bfs(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
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
pub fn bfs01(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
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
pub fn djk(graph: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
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
pub fn warshall(graph: &[Vec<(usize, i64)>]) -> Vec<Vec<i64>> {
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
pub fn bellman(graph: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, bool) {
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
pub fn kruskal(graph: &[Vec<(usize, i64)>]) -> Option<(i64, Vec<(usize, usize, i64)>)> {
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
pub fn is_bipartite(graph: &[Vec<(usize, i64)>]) -> bool {
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
pub fn conn_comp(graph: &[Vec<(usize, i64)>]) -> Vec<Vec<usize>> {
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
pub fn topo(graph: &[Vec<(usize, i64)>]) -> Option<Vec<usize>> {
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
pub fn tree_diameter(graph: &[Vec<(usize, i64)>]) -> i64 {
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

/// プリム法による最小全域木のコスト（無向グラフ用、頂点0を含む連結成分）
/// ## 出力
/// * `Some(合計コスト)` - 全頂点が連結な場合
/// * `None` - 非連結な場合
/// ### 計算量 O((N+M) log N)
pub fn prim(graph: &[Vec<(usize, i64)>]) -> Option<i64> {
    let n = graph.len();
    if n == 0 {
        return Some(0);
    }
    let mut used = vec![false; n];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0i64, 0usize)));
    let mut total_cost = 0i64;
    let mut cnt = 0;
    while let Some(std::cmp::Reverse((d, v))) = pq.pop() {
        if used[v] {
            continue;
        }
        used[v] = true;
        total_cost += d;
        cnt += 1;
        for &(next_v, cost) in &graph[v] {
            if !used[next_v] {
                pq.push(std::cmp::Reverse((cost, next_v)));
            }
        }
    }
    if cnt == n {
        Some(total_cost)
    } else {
        None
    }
}
