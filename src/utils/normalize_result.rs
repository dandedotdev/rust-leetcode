pub fn normalize_result(mut result: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for group in &mut result {
        group.sort();
    }

    result.sort();

    result
}
