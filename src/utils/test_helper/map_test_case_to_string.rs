pub fn map_test_case_to_string(test_case: Vec<&str>) -> Vec<String> {
    test_case.into_iter().map(String::from).collect()
}
