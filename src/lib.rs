//! AtCoder 用 Rust ライブラリ
//!
//! `main.rs`（テンプレート version5）と `rust.json`（VSCode スニペット）に
//! 含まれていた関数・構造体をモジュールごとに整理したもの。
//!
//! ```ignore
//! use ac_rust_lib::prelude::*; // 全部まとめて使う
//! use ac_rust_lib::graph::Lca; // 個別に使う
//! ```
//!
//! | モジュール | 内容 |
//! |---|---|
//! | [`consts`] | MOD / INF / 4・8近傍 / アルファベット |
//! | [`macros`] | `chmin!` `chmax!` `debug!` `yn!` `YN!` |
//! | [`math`] | 数論（gcd, 素因数分解, Miller-Rabin, Pollard-rho, CRT, floor_sum 等）, 組合せ, 畳み込み |
//! | [`algorithm`] | 二分探索, 累積和, 座標圧縮, RLE, LIS 等 |
//! | [`grid`] | グリッド範囲判定・回転・座標圧縮 |
//! | [`string`] | Z-algorithm, KMP, Manacher, Trie, Aho-Corasick, Suffix Array, Rolling Hash |
//! | [`geometry`] | 点, 外積・内積, ccw, 凸包 |
//! | [`data_structure`] | セグ木用モノイド, Sparse Table, BIT, Multiset, IntervalSet, Li Chao Tree, Wavelet Matrix, Treap, DSU |
//! | [`graph`] | DFS/BFS/Dijkstra/Bellman-Ford/Warshall-Floyd/MST/トポソ, LCA, HLD, 全方位木DP, 2-SAT |
//! | [`random`] | 乱数 |

#![allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    clippy::needless_range_loop,
    clippy::ptr_arg
)]

#[macro_use]
pub mod macros;

pub mod algorithm;
pub mod consts;
pub mod data_structure;
pub mod geometry;
pub mod graph;
pub mod grid;
pub mod math;
pub mod random;
pub mod string;

/// 全モジュールの公開項目をまとめて import するための prelude
pub mod prelude {
    pub use crate::algorithm::*;
    pub use crate::consts::*;
    pub use crate::data_structure::*;
    pub use crate::geometry::*;
    pub use crate::graph::*;
    pub use crate::grid::*;
    pub use crate::math::*;
    pub use crate::random::*;
    pub use crate::string::*;
    pub use crate::{chmax, chmin, debug, yn, YN};
}
