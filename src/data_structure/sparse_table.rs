/// Sparse Table
/// 静的配列（更新なし）に対する区間min/max/gcdなどの冪等モノイドクエリを O(1) で処理する
/// 構築 O(n log n)、クエリ O(1)
/// ## 使い方
/// let st = SparseTable::new(&a, |x, y| *x.min(y)); // 区間min
/// st.query(l, r) // [l, r) の最小値
pub struct SparseTable<T, F> {
    pub table: Vec<Vec<T>>,
    pub op: F,
}

impl<T: Copy, F: Fn(&T, &T) -> T> SparseTable<T, F> {
    pub fn new(a: &[T], op: F) -> Self {
        let n = a.len();
        let mut log = 1usize;
        while (1 << log) <= n {
            log += 1;
        }
        let mut table = vec![a.to_vec()];
        for k in 1..log {
            let prev = &table[k - 1];
            let half = 1usize << (k - 1);
            let mut row = Vec::with_capacity(n - (1 << k) + 1);
            for i in 0..=(n - (1 << k)) {
                row.push(op(&prev[i], &prev[i + half]));
            }
            table.push(row);
        }
        Self { table, op }
    }

    /// [l, r) の集約値（冪等演算であること: min, max, gcd, and, or など）
    pub fn query(&self, l: usize, r: usize) -> T {
        let len = r - l;
        let k = (usize::BITS - 1 - len.leading_zeros()) as usize; // floor(log2(len))
        (self.op)(&self.table[k][l], &self.table[k][r - (1 << k)])
    }
}
