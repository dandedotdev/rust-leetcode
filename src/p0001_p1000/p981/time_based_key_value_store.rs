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
        self.map
            .get(&key)
            .and_then(|v| match v.partition_point(|&(t, _)| t <= timestamp) {
                0 => None,
                pp => Some(v[pp - 1].1.clone()),
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut time_map = TimeMap::new();
        time_map.set(String::from("foo"), String::from("bar"), 1);
        time_map.get(String::from("foo"), 1);
        time_map.get(String::from("foo"), 3);
        time_map.set(String::from("foo"), String::from("bar2"), 4);
        time_map.get(String::from("foo"), 4);
        time_map.get(String::from("foo"), 5);
    }
}
