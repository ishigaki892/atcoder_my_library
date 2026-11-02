use crate::prelude::*;

// ------------------------------------------------------------
// 幾何系
// ------------------------------------------------------------

/// 2次元座標を表す点。f64 だけでなく i64 でも使えるよう型引数を持たせる想定なら
/// 別途ジェネリック化してもよいが、ここでは f64 前提のシンプル版を用意する。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

/// 外積 (cross product) : a × b
/// > 0 なら b は a から見て反時計回り、< 0 なら時計回り、0 なら平行
pub fn cross(a: Point, b: Point) -> f64 {
    a.x * b.y - a.y * b.x
}

/// 内積 (dot product)
pub fn dot(a: Point, b: Point) -> f64 {
    a.x * b.x + a.y * b.y
}

/// ccw (counter-clockwise) : 点 p0, p1, p2 の位置関係を判定する
/// 1  : 反時計回り (COUNTER_CLOCKWISE)
/// -1 : 時計回り (CLOCKWISE)
/// 2  : p2, p0, p1 の順で一直線上 (p0 が p1-p2 の外側、逆方向)  (ONLINE_BACK)
/// -2 : p0, p1, p2 の順で一直線上 (p2 が p0-p1 の先) (ONLINE_FRONT)
/// 0  : p0, p1 の線分上に p2 が乗っている (ON_SEGMENT)
pub fn ccw(p0: Point, p1: Point, p2: Point) -> i32 {
    let a = p1 - p0;
    let b = p2 - p0;
    if cross(a, b) > EPS {
        return 1;
    }
    if cross(a, b) < -EPS {
        return -1;
    }
    if dot(a, b) < -EPS {
        return 2;
    }
    if a.x * a.x + a.y * a.y < b.x * b.x + b.y * b.y - EPS {
        return -2;
    }
    0
}

/// 凸包（Andrewのモノトーンチェイン法）O(n log n)
/// ## 出力
/// 凸包を構成する点を反時計回りに並べたもの（重複・共線点は含まない）
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap()
            .then(a.y.partial_cmp(&b.y).unwrap())
    });
    pts.dedup_by(|a, b| (a.x - b.x).abs() < EPS && (a.y - b.y).abs() < EPS);
    let n = pts.len();
    if n <= 2 {
        return pts;
    }

    let mut hull: Vec<Point> = Vec::with_capacity(2 * n);
    // 下側
    for &p in &pts {
        while hull.len() >= 2
            && cross(
                hull[hull.len() - 1] - hull[hull.len() - 2],
                p - hull[hull.len() - 2],
            ) <= EPS
        {
            hull.pop();
        }
        hull.push(p);
    }
    // 上側
    let lower_size = hull.len() + 1;
    for &p in pts.iter().rev().skip(1) {
        while hull.len() >= lower_size
            && cross(
                hull[hull.len() - 1] - hull[hull.len() - 2],
                p - hull[hull.len() - 2],
            ) <= EPS
        {
            hull.pop();
        }
        hull.push(p);
    }
    hull.pop(); // 最後は始点と重複するので除く
    hull
}
