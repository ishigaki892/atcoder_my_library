//! 遅延伝播・反転可能・乱択平衡二分探索木 (Implicit Treap / RBST)
//!
//! 配列を「木構造」として保持し、以下をすべて O(log N) (期待値) で行える:
//!   - 任意位置への要素の挿入 / 削除            ← セグ木にはできない
//!   - 区間の一括更新 (作用素の適用, LazySegtreeのapply_rangeと同じ)
//!   - 区間の反転 (reverse)                     ← セグ木にはできない
//!   - 区間の集約値取得 (fold, LazySegtreeのprodと同じ)
//!   - 木の分割 (split) / 結合 (merge) を基本操作として持つ
//!
//! ac_library::MapMonoid と同じ考え方で「値のモノイド」と「作用」を1つの
//! トレイトにまとめている。追加で reverse() を要求する点だけが違う。

use std::mem::swap;

/// 値の集約(モノイド)と、区間への作用(マッピング)を定義するトレイト。
pub trait TMapMonoid {
    /// 集約される値の型 (区間の集約結果)
    type S: Clone;
    /// 区間に適用する作用の型
    type F: Clone;

    /// Sの単位元
    fn identity() -> Self::S;
    /// Sの二項演算 (結合律を満たすこと)
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
    /// 区間反転時に集約値Sをどう変換するか。
    /// 和・min・max・xorなど「可換」な演算はデフォルト実装(そのまま返す)でよい。
    /// 文字列連結・行列積など「非可換」な演算では左右を入れ替える処理を書く。
    fn reverse(x: &Self::S) -> Self::S {
        x.clone()
    }

    /// Fの単位元 (何もしない作用)
    fn identity_map() -> Self::F;
    /// Sに作用Fを適用する
    fn mapping(f: &Self::F, x: &Self::S) -> Self::S;
    /// 作用の合成: 「先にgを適用、後にfを適用」した結果と同じ効果になるようにする
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
}

// --- 優先度生成用の乱数 (xorshift64) ---
pub struct Xorshift(u64);
impl Xorshift {
    pub fn new() -> Self {
        Xorshift(0x2545_F491_4F6C_DD1D)
    }
    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

pub struct Node<M: TMapMonoid> {
    pub val: M::S,  // このノード単体の値
    pub acc: M::S,  // 部分木全体の集約値
    pub lazy: M::F, // 未反映の作用
    pub rev: bool,  // 反転フラグ
    pub priority: u64,
    pub size: usize,
    pub left: Option<Box<Node<M>>>,
    pub right: Option<Box<Node<M>>>,
}

impl<M: TMapMonoid> Node<M> {
    pub fn new(val: M::S, priority: u64) -> Self {
        Node {
            acc: val.clone(),
            val,
            lazy: M::identity_map(),
            rev: false,
            priority,
            size: 1,
            left: None,
            right: None,
        }
    }
}

pub fn size<M: TMapMonoid>(node: &Option<Box<Node<M>>>) -> usize {
    node.as_ref().map_or(0, |n| n.size)
}

pub fn acc<M: TMapMonoid>(node: &Option<Box<Node<M>>>) -> M::S {
    node.as_ref().map_or(M::identity(), |n| n.acc.clone())
}

/// 子を更新した後、部分木サイズ・集約値を再計算する
pub fn update<M: TMapMonoid>(node: &mut Box<Node<M>>) {
    node.size = 1 + size::<M>(&node.left) + size::<M>(&node.right);
    node.acc = M::op(
        &M::op(&acc::<M>(&node.left), &node.val),
        &acc::<M>(&node.right),
    );
}

/// 未反映の反転フラグ・作用を子に伝播する
pub fn push_down<M: TMapMonoid>(node: &mut Box<Node<M>>) {
    if node.rev {
        swap(&mut node.left, &mut node.right);
        if let Some(l) = node.left.as_mut() {
            l.rev ^= true;
            l.val = M::reverse(&l.val);
            l.acc = M::reverse(&l.acc);
        }
        if let Some(r) = node.right.as_mut() {
            r.rev ^= true;
            r.val = M::reverse(&r.val);
            r.acc = M::reverse(&r.acc);
        }
        node.rev = false;
    }
    let f = node.lazy.clone();
    node.lazy = M::identity_map();
    if let Some(l) = node.left.as_mut() {
        apply_node::<M>(l, &f);
    }
    if let Some(r) = node.right.as_mut() {
        apply_node::<M>(r, &f);
    }
}

pub fn apply_node<M: TMapMonoid>(node: &mut Box<Node<M>>, f: &M::F) {
    node.val = M::mapping(f, &node.val);
    node.acc = M::mapping(f, &node.acc);
    node.lazy = M::composition(f, &node.lazy);
}

/// 木を先頭k個 [0,k) と残り [k,size) に分割する
pub fn split<M: TMapMonoid>(
    node: Option<Box<Node<M>>>,
    k: usize,
) -> (Option<Box<Node<M>>>, Option<Box<Node<M>>>) {
    match node {
        None => (None, None),
        Some(mut n) => {
            push_down::<M>(&mut n);
            let ls = size::<M>(&n.left);
            if k <= ls {
                let (l, r) = split::<M>(n.left.take(), k);
                n.left = r;
                update::<M>(&mut n);
                (l, Some(n))
            } else {
                let (l, r) = split::<M>(n.right.take(), k - ls - 1);
                n.right = l;
                update::<M>(&mut n);
                (Some(n), r)
            }
        }
    }
}

/// 2つの木を結合する (左木の全要素が右木より前という前提)
pub fn merge<M: TMapMonoid>(
    l: Option<Box<Node<M>>>,
    r: Option<Box<Node<M>>>,
) -> Option<Box<Node<M>>> {
    match (l, r) {
        (None, r) => r,
        (l, None) => l,
        (Some(mut l), Some(mut r)) => {
            if l.priority > r.priority {
                push_down::<M>(&mut l);
                l.right = merge::<M>(l.right.take(), Some(r));
                update::<M>(&mut l);
                Some(l)
            } else {
                push_down::<M>(&mut r);
                r.left = merge::<M>(Some(l), r.left.take());
                update::<M>(&mut r);
                Some(r)
            }
        }
    }
}

pub struct RBST<M: TMapMonoid> {
    pub root: Option<Box<Node<M>>>,
    pub rng: Xorshift,
}

impl<M: TMapMonoid> RBST<M> {
    pub fn new() -> Self {
        RBST {
            root: None,
            rng: Xorshift::new(),
        }
    }

    /// 配列から構築する O(N log N)
    pub fn build(v: Vec<M::S>) -> Self {
        let mut t = Self::new();
        for x in v {
            let n = t.len();
            t.insert(n, x);
        }
        t
    }

    pub fn len(&self) -> usize {
        size::<M>(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 位置pに値xを挿入する (0 <= p <= len())
    pub fn insert(&mut self, p: usize, x: M::S) {
        let new_node = Some(Box::new(Node::new(x, self.rng.next())));
        let root = self.root.take();
        let (l, r) = split::<M>(root, p);
        self.root = merge::<M>(merge::<M>(l, new_node), r);
    }

    /// 位置pの要素を削除する
    pub fn erase(&mut self, p: usize) {
        let root = self.root.take();
        let (l, r) = split::<M>(root, p);
        let (_, r) = split::<M>(r, 1);
        self.root = merge::<M>(l, r);
    }

    /// 位置pの値を取得する
    pub fn get(&mut self, p: usize) -> M::S {
        let root = self.root.take();
        let (l, mid_r) = split::<M>(root, p);
        let (mid, r) = split::<M>(mid_r, 1);
        let val = mid.as_ref().unwrap().val.clone();
        self.root = merge::<M>(merge::<M>(l, mid), r);
        val
    }

    /// 位置pの値をxに設定する
    pub fn set(&mut self, p: usize, x: M::S) {
        let root = self.root.take();
        let (l, mid_r) = split::<M>(root, p);
        let (_mid, r) = split::<M>(mid_r, 1);
        let new_mid = Some(Box::new(Node::new(x, self.rng.next())));
        self.root = merge::<M>(merge::<M>(l, new_mid), r);
    }

    /// 区間[l, r)の集約値を取得する
    pub fn fold(&mut self, l: usize, r: usize) -> M::S {
        let root = self.root.take();
        let (a, bc) = split::<M>(root, l);
        let (b, c) = split::<M>(bc, r - l);
        let result = acc::<M>(&b);
        self.root = merge::<M>(merge::<M>(a, b), c);
        result
    }

    /// 区間[l, r)に作用fを適用する
    pub fn apply(&mut self, l: usize, r: usize, f: M::F) {
        let root = self.root.take();
        let (a, bc) = split::<M>(root, l);
        let (mut b, c) = split::<M>(bc, r - l);
        if let Some(node) = b.as_mut() {
            apply_node::<M>(node, &f);
        }
        self.root = merge::<M>(merge::<M>(a, b), c);
    }

    /// 区間[l, r)を反転する
    pub fn reverse(&mut self, l: usize, r: usize) {
        let root = self.root.take();
        let (a, bc) = split::<M>(root, l);
        let (mut b, c) = split::<M>(bc, r - l);
        if let Some(node) = b.as_mut() {
            node.rev ^= true;
            node.val = M::reverse(&node.val);
            node.acc = M::reverse(&node.acc);
        }
        self.root = merge::<M>(merge::<M>(a, b), c);
    }

    /// 配列として復元する (デバッグ・出力用)
    pub fn to_vec(&mut self) -> Vec<M::S> {
        let mut result = Vec::with_capacity(self.len());
        let root = self.root.take();
        self.root = Self::collect(root, &mut result);
        result
    }

    pub fn collect(node: Option<Box<Node<M>>>, out: &mut Vec<M::S>) -> Option<Box<Node<M>>> {
        match node {
            None => None,
            Some(mut n) => {
                push_down::<M>(&mut n);
                n.left = Self::collect(n.left.take(), out);
                out.push(n.val.clone());
                n.right = Self::collect(n.right.take(), out);
                update::<M>(&mut n);
                Some(n)
            }
        }
    }
}

// ========================= 使用例 =========================

// 例1: 区間加算 + 区間和 (可換なのでreverseはデフォルト実装のままでよい)
// ※ data_structure::segtree の RangeAddRangeSum と名前が被るので、使うときは名前を変えること
/*
struct RangeAddRangeSum;
impl TMapMonoid for RangeAddRangeSum {
    type S = (i64, i64); // (合計, 要素数)
    type F = i64;

    fn identity() -> Self::S {
        (0, 0)
    }
    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1)
    }
    // fn reverse(x: &Self::S) -> Self::S { x.clone() } // デフォルトのままでOK(和は可換)

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
fn main() {
    let init: Vec<(i64, i64)> = vec![1, 2, 3, 4, 5].into_iter().map(|v| (v, 1)).collect();
    let mut t = RBST::<RangeAddRangeSum>::build(init);
    // 現在の配列: [1, 2, 3, 4, 5]

    t.insert(2, (100, 1)); // 位置2に100を挿入
    println!("{:?}", t.to_vec()); // [(1,1),(2,1),(100,1),(3,1),(4,1),(5,1)]

    t.erase(0); // 先頭を削除
    println!("{:?}", t.to_vec()); // [(2,1),(100,1),(3,1),(4,1),(5,1)]

    t.apply(0, 2, 10); // 区間[0,2)に+10
    println!("{:?}", t.to_vec()); // [(12,1),(110,1),(3,1),(4,1),(5,1)]

    t.reverse(1, 4); // 区間[1,4)を反転
    println!("{:?}", t.to_vec()); // [(12,1),(4,1),(3,1),(110,1),(5,1)]

    let (sum, _len) = t.fold(0, t.len());
    println!("total sum = {}", sum);
}
*/
