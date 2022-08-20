#[cfg(test)]
mod linear_search_test {
    #[test]
    fn linear_search_array() {
        let arr = vec!(1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420);

        assert_eq!(crate::linear_search::linear_search(&arr, 69), true);
        assert_eq!(crate::linear_search::linear_search(&arr, 1336), false);
        assert_eq!(crate::linear_search::linear_search(&arr, 69420), true);
        assert_eq!(crate::linear_search::linear_search(&arr, 69421), false);
        assert_eq!(crate::linear_search::linear_search(&arr, 1), true);
        assert_eq!(crate::linear_search::linear_search(&arr, 0), false);
    }
}
