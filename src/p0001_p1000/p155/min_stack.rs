// <Array, Design, Stack>
// Time:
//   - push: O(1)
//   - pop: O(1)
//   - top: O(1)
//   - get_min: O(1)
// Space: O(n)

pub struct MinStack {
    main: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            main: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.main.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        assert!(!self.main.is_empty());
        if self.main.last() == self.min.last() {
            self.min.pop();
        }
        self.main.pop();
    }

    fn top(&self) -> i32 {
        assert!(!self.main.is_empty());
        *self.main.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        assert!(!self.min.is_empty());
        *self.min.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut min_stk = MinStack::new();
        min_stk.push(-2);
        min_stk.push(0);
        min_stk.push(-3);
        assert_eq!(min_stk.get_min(), -3);
        min_stk.pop();
        assert_eq!(min_stk.top(), 0);
        assert_eq!(min_stk.get_min(), -2);
    }
}
