//! 分割後もライブラリとして正しく動くかを確認するスモークテスト

use ac_library::{LazySegtree, Segtree};
use ac_rust_lib::prelude::*;

#[test]
fn macros() {
    let mut a = 5;
    assert!(chmin!(a, 3));
    assert!(!chmin!(a, 4));
    assert!(chmax!(a, 10));
    assert_eq!(a, 10);
}

#[test]
fn number_theory() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(power(2, 10, 1_000_000_007), 1024);
    assert_eq!(combination(5, 2, MOD), 10);
    let c = Comb::new(10, MOD);
    assert_eq!(c.ncr(10, 3), 120);
    assert!(miller_rabin(998_244_353));
    assert!(!miller_rabin(1_000_000_007 * 3));
    let mut f = pollard_rho(2 * 3 * 3 * 1_000_000_007);
    f.sort();
    assert_eq!(f, vec![2, 3, 3, 1_000_000_007]);
    assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
    assert_eq!(count_divisors(12), 6);
    assert_eq!(isqrt(99), 9);
    assert_eq!(cbrt(27), 3);
    assert_eq!(to_n_base(10, 2), vec![1, 0, 1, 0]);
    assert_eq!(from_n_base(&[1, 0, 1, 0], 2), 10);
    assert_eq!(crt(2, 3, 3, 5), Some((8, 15)));
    assert_eq!(get_nth(12345, 2), 3);
    let p = generate_palindromes(3);
    assert_eq!(p.len(), 10 + 9 + 90);
    assert!(p.contains(&121) && p.contains(&99));
}

#[test]
fn convolution() {
    assert_eq!(ntt_convolution(&[1, 2], &[3, 4]), vec![3, 10, 8]);
}

#[test]
fn algorithm_and_grid() {
    assert_eq!(rle(&[1, 1, 2, 2, 2, 3]), vec![(1, 2), (2, 3), (3, 1)]);
    assert_eq!(lis(&vec![3, 1, 4, 1, 5, 9, 2, 6]).len(), 4);
    assert_eq!(get_tousa(&[1, 7, 10, 13, 14, 19]), 4);
    let g = vec![vec!['.', '#'], vec!['.', '.']];
    assert_eq!(rotate_cw(&g), vec![vec!['.', '.'], vec!['.', '#']]);
    let (c, rows, cols) = compress_grid(&g, '#');
    assert_eq!((c, rows, cols), (vec![vec!['#']], vec![0], vec![1]));
}

#[test]
fn string() {
    let s: Vec<char> = "aabaab".chars().collect();
    assert_eq!(z_algorithm(&s)[3], 3);
    let kmp = Kmp::new(&"ab".chars().collect::<Vec<_>>());
    assert_eq!(kmp.search(&s), vec![1, 4]);
    let rh = RollingHash::new(b"abcabc");
    assert_eq!(rh.get_hash(0, 3), rh.get_hash(3, 6));
}

#[test]
fn segtree_monoids() {
    let mut seg = LazySegtree::<Affine>::from(vec![(1i64, 1i64); 4]);
    seg.apply_range(0..4, (2, 3)); // 1 -> 5
    assert_eq!(seg.prod(0..4).0, 20);
    let mut seg = LazySegtree::<RangeUpdateRangeMin>::from(vec![5i64; 4]);
    seg.apply_range(1..3, Some(2));
    assert_eq!(seg.prod(0..4), 2);
    let seg = Segtree::<Sum>::from(vec![(3i64, 1i64); 4]);
    assert_eq!(seg.prod(..), (12, 4));
}

#[test]
fn data_structures() {
    let st = SparseTable::new(&[5, 3, 8, 1, 9], |a: &i64, b: &i64| *a.min(b));
    assert_eq!(st.query(0, 3), 3);

    let mut ms = Multiset::new();
    ms.insert(3);
    ms.insert(3);
    ms.insert(1);
    assert_eq!(ms.count(&3), 2);
    assert_eq!(ms.pop_min(), Some(1));

    let mut is = IntervalSet::new();
    is.insert(1, 3);
    is.insert(3, 5);
    assert_eq!(is.get(4), Some((1, 5)));

    let mut uf = UnionFind::new(4);
    uf.merge(0, 1);
    assert!(uf.same(0, 1) && !uf.same(1, 2));
    assert_eq!(uf.groups().len(), 3);

    let mut wd = WeightedDsu::new(3);
    wd.merge(0, 1, 5);
    assert_eq!(wd.diff(0, 1), 5);

    let wm = WaveletMatrix::new(&[5, 1, 4, 2, 3], 3);
    assert_eq!(wm.kth_smallest(0, 5, 2), 3);
}

#[test]
fn treap() {
    struct Add;
    impl TMapMonoid for Add {
        type S = (i64, i64);
        type F = i64;
        fn identity() -> Self::S {
            (0, 0)
        }
        fn op(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1)
        }
        fn identity_map() -> Self::F {
            0
        }
        fn mapping(f: &Self::F, x: &Self::S) -> Self::S {
            (x.0 + f * x.1, x.1)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }
    let mut t = RBST::<Add>::build((1..=5).map(|v| (v, 1)).collect());
    t.insert(2, (100, 1));
    t.reverse(0, 6);
    assert_eq!(t.get(0).0, 5);
    assert_eq!(t.fold(0, t.len()).0, 115);
}

#[test]
fn graph() {
    // 0 -1- 1 -2- 2, 0 -10- 2
    let mut g = Graph::new(3, false);
    g.add(0, 1, 1);
    g.add(1, 2, 2);
    g.add(0, 2, 10);
    assert_eq!(g.djk(0), vec![0, 1, 3]);
    assert_eq!(g.kruskal().unwrap().0, 3);
    assert_eq!(g.prim(), Some(3));
    assert!(g.cycle());
    assert!(!g.is_bipartite());

    let mut d = Graph::new(3, true);
    d.add(0, 1, 1);
    d.add(1, 2, 1);
    assert_eq!(d.topo(), Some(vec![0, 1, 2]));
    assert!(!d.cycle());

    let tree = vec![vec![(1, 1), (2, 1)], vec![(0, 1)], vec![(0, 1)]];
    let lca = Lca::new(&tree, 0);
    assert_eq!(lca.query(1, 2), 0);
    assert_eq!(tree_diameter(&tree), 2);

    let mut sat = TwoSat::new(2);
    sat.add_clause(0, true, 1, true);
    sat.add_clause(0, false, 0, false);
    let ans = sat.solve().unwrap();
    assert!(!ans[0] && ans[1]);
}

#[test]
fn geometry() {
    let pts: Vec<Point> = [(0., 0.), (1., 0.), (1., 1.), (0., 1.), (0.5, 0.5)]
        .iter()
        .map(|&(x, y)| Point::new(x, y))
        .collect();
    assert_eq!(convex_hull(&pts).len(), 4);
}
