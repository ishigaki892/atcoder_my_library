// ------------------------------------------------------------
// グリッド系
// ------------------------------------------------------------

/// グリッドの範囲内判定
pub fn in_grid(ny: isize, nx: isize, h: usize, w: usize) -> bool {
    ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize
}

/// グリッドを時計回りに90度回転する
/// 入力: N行M列 → 出力: M行N列
/// 変換式: new[j][N-1-i] = old[i][j]
pub fn rotate_cw<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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
pub fn rotate_ccw<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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
pub fn rotate_180<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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

/// グリッドを指定した要素で座標圧縮する（target を1つも含まない行・列を取り除く）
/// ## 返り値
/// * `compressed` - 圧縮後のグリッド
/// * `row_map` - 圧縮後の行インデックス → 元の行インデックス
/// * `col_map` - 圧縮後の列インデックス → 元の列インデックス
pub fn compress_grid(
    grid: &Vec<Vec<char>>,
    target: char,
) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let rows = grid.len();
    let cols = grid[0].len();

    // target が1つでも存在する行・列のインデックスを収集
    let row_map: Vec<usize> = (0..rows)
        .filter(|&r| grid[r].iter().any(|&c| c == target))
        .collect();
    let col_map: Vec<usize> = (0..cols)
        .filter(|&c| (0..rows).any(|r| grid[r][c] == target))
        .collect();

    // 圧縮後のグリッドを構築
    let compressed = row_map
        .iter()
        .map(|&r| col_map.iter().map(|&c| grid[r][c]).collect())
        .collect();

    (compressed, row_map, col_map)
}
