mod linear_search_test;
mod linear_search;

fn main() {
}

fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    arr
}

#[cfg(test)]
mod bubble_sort {
    #[test]
    fn test_bubble_sort_1() {
        let arr: Vec<i32> = vec!(3, 5, 1, 2, 4);
        let expected_sorted: Vec<i32> = vec!(1, 2, 3, 4, 5);

        let actual_sorted = crate::bubble_sort(arr);

        assert_eq!(expected_sorted, actual_sorted);
    }

    #[test]
    fn test_bubble_sort_2() {
        let arr: Vec<i32> = vec!(9, 7, 2, 20, 420, 69);
        let expected_sorted: Vec<i32> = vec!(2, 7, 9, 20, 69, 420);

        let actual_sorted = crate::bubble_sort(arr);

        assert_eq!(expected_sorted, actual_sorted);
    }
}
