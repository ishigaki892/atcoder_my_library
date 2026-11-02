use std::collections::BTreeMap;

// ------------------------------------------------------------
// その他データ構造系
// ------------------------------------------------------------

/// 多重集合
pub struct Multiset<T> {
    pub map: BTreeMap<T, usize>,
    pub len: usize,
}

impl<T: Ord> Multiset<T> {
    pub fn new() -> Self {
        Multiset {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_insert(0) += 1;
        self.len += 1;
    }

    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.get_mut(value) {
            if *count > 0 {
                *count -= 1;
                self.len -= 1;
                if *count == 0 {
                    self.map.remove(value);
                }
                return true;
            }
        }
        false
    }

    pub fn count(&self, value: &T) -> usize {
        *self.map.get(value).unwrap_or(&0)
    }

    /// 集合全体の要素数（種類数ではなく、多重度を含めた総数）
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn min_element(&self) -> Option<&T> {
        self.map.keys().next()
    }

    pub fn max_element(&self) -> Option<&T> {
        self.map.keys().next_back()
    }

    pub fn pop_min(&mut self) -> Option<T>
    where
        T: Clone,
    {
        let min_key = self.min_element()?.clone();

        self.remove(&min_key);

        Some(min_key)
    }
}
