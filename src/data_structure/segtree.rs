use ac_library::{MapMonoid, Max, Min, Monoid};

// ------------------------------------------------------------
// セグメントツリー
// ------------------------------------------------------------

//let mut seg = Segtree::<Sum>::from(vec![(0i64, 1i64); n]); // 各要素 (値, 1) で初期化
//一点更新+区間和（要素数も持つので遅延セグ木の RangeAddRangeSum 等と共用できる）

pub struct Sum;
impl Monoid for Sum {
    type S = (i64, i64); // (合計, 要素数)
    fn identity() -> Self::S {
        (0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
}

//let mut seg = Segtree::<InvCount>::from(vec![(0u64, 0u64, 0u64); n]); // 各要素(1,0,0)or(0,1,0)で初期化
// 0/1列の転倒数

pub struct InvCount;
impl Monoid for InvCount {
    type S = (u64, u64, u64); // (#0の数, #1の数, 転倒数)
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2 + a.1 * b.0)
    }
}
//let mut seg = Segtree::<MinMax>::from(vec![(0i64, 0i64); n]);
// 区間最小値と最大値

pub struct MinMax;
impl Monoid for MinMax {
    type S = (i64, i64); // (最小値, 最大値)
    fn identity() -> Self::S {
        (i64::MAX, i64::MIN)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0.min(b.0), a.1.max(b.1))
    }
}

//let mut seg = Segtree::<Xor>::from(vec![0i64; n]);
// 区間XOR

pub struct Xor;
impl Monoid for Xor {
    type S = i64;
    fn identity() -> Self::S {
        0
    } // 0 XOR x = x
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

//let mut seg = Segtree::<Gcd>::from(vec![0i64; n]);
// 区間最大公約数

pub struct Gcd;
impl Monoid for Gcd {
    type S = i64;
    fn identity() -> Self::S {
        0
    } // gcd(x, 0) = x
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        gcd(*a, *b)
    }
}
//let mut seg = Segtree::<MinCount>::from(vec![(1i64, 1i64); n]); // 各要素(値, 1)で初期化
// 区間最小値とその個数

pub struct MinCount;
impl Monoid for MinCount {
    type S = (i64, i64); // (最小値, その個数)
    fn identity() -> Self::S {
        (i64::MAX, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 < b.0 {
            *a
        } else if a.0 > b.0 {
            *b
        } else {
            (a.0, a.1 + b.1)
        }
    }
}

// ------------------------------------------------------------
// 遅延セグメントツリー
// ------------------------------------------------------------

//let mut seg = LazySegtree::<RangeAddRangeSum>::from(vec![(0, 1); n]);
//区間加算+区間和

pub struct RangeAddRangeSum;
impl MapMonoid for RangeAddRangeSum {
    type M = Sum;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (x.0 + f * x.1, x.1)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<RangeAddRangeMin>::from(vec![0i64; n]);
//区間加算 + 区間最小値

pub struct RangeAddRangeMin;
impl MapMonoid for RangeAddRangeMin {
    type M = Min<i64>;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<Flip>::from(vec![(0u64, 0u64, 0u64); n]); // 各要素(1,0,0)or(0,1,0)で初期化
// 区間反転 + 0/1列の転倒数

pub struct Flip;
impl MapMonoid for Flip {
    type M = InvCount;
    type F = bool;
    fn identity_map() -> Self::F {
        false
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f {
            (x.1, x.0, x.0 * x.1 - x.2)
        } else {
            *x
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
    }
}

//let mut seg = LazySegtree::<RangeUpdateRangeMax>::from(vec![i64::MIN; n]);
// 区間更新(代入) + 区間最大値

pub struct RangeUpdateRangeMax;
impl MapMonoid for RangeUpdateRangeMax {
    type M = Max<i64>;
    type F = Option<i64>;
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.unwrap_or(*x)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

//let mut seg = LazySegtree::<RangeAddRangeMax>::from(vec![0i64; n]);
// 区間加算 + 区間最大値

pub struct RangeAddRangeMax;
impl MapMonoid for RangeAddRangeMax {
    type M = Max<i64>;
    type F = i64;
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

//let mut seg = LazySegtree::<RangeXorRangeXor>::from(vec![0i64; n]);
// 区間XOR + 区間一括XOR

pub struct RangeXorRangeXor;
impl MapMonoid for RangeXorRangeXor {
    type M = Xor;
    type F = i64; // 一括でXORする値
    fn identity_map() -> Self::F {
        0
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f ^ x
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f ^ g
    }
}

//let init: Vec<(i64, i64)> = vec![(0, 1); n];
//let mut seg = LazySegtree::<RangeUpdateRangeSum>::from(init);
// 区間更新(代入) + 区間和

pub struct RangeUpdateRangeSum;
impl MapMonoid for RangeUpdateRangeSum {
    type M = Sum;
    type F = Option<i64>;
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        match f {
            Some(v) => (v * x.1, x.1),
            None => *x,
        }
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}

// 区間更新(代入) + 区間最小値
// let mut seg = LazySegtree::<RangeUpdateRangeMin>::from(vec![i64::MAX; n]);

pub struct RangeUpdateRangeMin;
impl MapMonoid for RangeUpdateRangeMin {
    type M = Min<i64>;
    type F = Option<i64>; // Noneなら更新なし
    fn identity_map() -> Self::F {
        None
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f.unwrap_or(*x)
    }
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g) // 新しい更新fを優先
    }
}

// 区間アフィン変換(x -> a*x + b) + 区間和
// let mut seg = LazySegtree::<Affine>::from(vec![(0i64, 1i64); n]); // 各要素 (値, 1) で初期化

pub struct Affine;
impl MapMonoid for Affine {
    type M = Sum;
    type F = (i64, i64); // (a, b): x -> a*x + b
    fn identity_map() -> Self::F {
        (1, 0)
    }
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        (f.0 * x.0 + f.1 * x.1, x.1)
    }
    // g適用後にfを適用: f(g(x)) = f.0*(g.0*x + g.1) + f.1
    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        (f.0 * g.0, f.0 * g.1 + f.1)
    }
}
