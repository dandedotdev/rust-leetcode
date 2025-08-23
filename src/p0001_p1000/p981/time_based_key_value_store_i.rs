use std::collections::HashMap;

// <Binary Search, Design, Hash Table, String>
// Time:
//   - set: O(1)
//   - get: O(log n)
// Space: O(n)

pub struct TimeMap {
    pub map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::default(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(values) = self.map.get(&key) else {
            return String::default();
        };
        let mut ans = "";
        let (mut left, mut right) = (0, values.len());
        while left < right {
            let mid = (left + right) >> 1;
            if values[mid].0 <= timestamp {
                ans = &values[mid].1;
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        time_map.get("foo".to_string(), 1);
        time_map.get("foo".to_string(), 3);
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        time_map.get("foo".to_string(), 4);
        time_map.get("foo".to_string(), 5);
    }
}
