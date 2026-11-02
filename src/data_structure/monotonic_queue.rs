use std::collections::VecDeque;

/// Index Monotonic Queue
pub struct IndexMonotonicQueue<T> {
    pub deque: VecDeque<(usize, T)>,
}

impl<T: Ord> IndexMonotonicQueue<T> {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    pub fn push(&mut self, index: usize, value: T) {
        while let Some(&(_, ref back)) = self.deque.back() {
            if *back < value {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back((index, value));
    }

    pub fn pop(&mut self, index: usize) {
        if let Some(&(front_index, _)) = self.deque.front() {
            if front_index == index {
                self.deque.pop_front();
            }
        }
    }

    pub fn max(&self) -> Option<&T> {
        self.deque.front().map(|&(_, ref value)| value)
    }
}
