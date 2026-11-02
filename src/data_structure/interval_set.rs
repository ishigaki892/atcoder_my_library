use std::cmp::{max, min};
use std::collections::BTreeMap;

// ------------------------------------------------------------
// IntervalSet（半開区間 [l, r) の集合）
// ------------------------------------------------------------
// let mut s = IntervalSet::new();
// s.insert(l, r);        // [l, r) を追加。新たに埋まった長さを返す
// s.remove(l, r);        // [l, r) を削除。実際に消えた長さを返す
// s.get(x);              // x を含む区間 Some((l, r))、なければ None
// s.contains(x);         // x が含まれるか
// s.same(a, b);          // a と b が同じ区間に含まれるか
// s.mex(x);              // x 以上で、どの区間にも含まれない最小の整数
// s.covered();           // 全区間の長さの合計（O(1)）
// s.len();               // 区間の個数
// s.iter();              // (l, r) を昇順に列挙
// ・隣り合う区間 ([1,3) と [3,5)) は [1,5) に結合される
// ・閉区間 [l, r] を扱うなら insert(l, r + 1) のように渡す
// ・複数持つなら vec![IntervalSet::new(); n] で OK（Clone 実装済み）

#[derive(Clone, Debug, Default)]
pub struct IntervalSet {
    pub map: BTreeMap<i64, i64>, // 開始 -> 終端（半開）
    pub total: i64,              // カバーしている長さの合計
}

impl IntervalSet {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            total: 0,
        }
    }

    /// [l, r) を追加する。新たにカバーされた長さを返す
    pub fn insert(&mut self, mut l: i64, mut r: i64) -> i64 {
        if l >= r {
            return 0;
        }
        let mut removed_len = 0; // 結合して消した既存区間の長さ

        // 左: l 以下で始まる最後の区間が l に届く（隣接含む）なら結合
        let left = self.map.range(..=l).next_back().map(|(&a, &b)| (a, b));
        if let Some((pl, pr)) = left {
            if pr >= l {
                removed_len += pr - pl;
                self.map.remove(&pl);
                l = pl;
                r = max(r, pr);
            }
        }

        // 右: 開始が r 以下の区間（隣接含む）を全部取り込む
        loop {
            let right = self.map.range(l..).next().map(|(&a, &b)| (a, b));
            match right {
                Some((nl, nr)) if nl <= r => {
                    removed_len += nr - nl;
                    self.map.remove(&nl);
                    r = max(r, nr);
                }
                _ => break,
            }
        }

        self.map.insert(l, r);
        let added = (r - l) - removed_len;
        self.total += added;
        added
    }

    /// [l, r) を削除する。実際に消えた長さを返す
    pub fn remove(&mut self, l: i64, r: i64) -> i64 {
        if l >= r {
            return 0;
        }
        let mut removed = 0;

        // l 以上 r 未満で始まる区間
        let mids: Vec<(i64, i64)> = self.map.range(l..r).map(|(&a, &b)| (a, b)).collect();
        for (a, b) in mids {
            self.map.remove(&a);
            removed += min(b, r) - a;
            if b > r {
                self.map.insert(r, b); // 右にはみ出た部分を残す
            }
        }

        // l より前で始まり、l をまたぐ区間
        let left = self.map.range(..l).next_back().map(|(&a, &b)| (a, b));
        if let Some((pl, pr)) = left {
            if pr > l {
                removed += min(pr, r) - l;
                self.map.insert(pl, l); // 左側を残す
                if pr > r {
                    self.map.insert(r, pr); // 右側も残す
                }
            }
        }

        self.total -= removed;
        removed
    }

    /// x を含む区間 [l, r) を返す
    pub fn get(&self, x: i64) -> Option<(i64, i64)> {
        self.map
            .range(..=x)
            .next_back()
            .filter(|&(_, &r)| x < r)
            .map(|(&l, &r)| (l, r))
    }

    pub fn contains(&self, x: i64) -> bool {
        self.get(x).is_some()
    }

    /// a と b が同じ区間に含まれるか
    pub fn same(&self, a: i64, b: i64) -> bool {
        match (self.get(a), self.get(b)) {
            (Some(p), Some(q)) => p == q,
            _ => false,
        }
    }

    /// x 以上で、どの区間にも含まれない最小の整数 (mex)
    /// 隣接区間は結合済みなので、区間の終端 r は必ず未カバー
    pub fn mex(&self, x: i64) -> i64 {
        match self.get(x) {
            Some((_, r)) => r,
            None => x,
        }
    }

    /// カバーしている長さの合計
    pub fn covered(&self) -> i64 {
        self.total
    }

    /// 区間の個数
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// (l, r) を昇順に列挙
    pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.map.iter().map(|(&l, &r)| (l, r))
    }
}
