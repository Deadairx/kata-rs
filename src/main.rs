fn main() {
}

fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut new_arr = arr.clone();
    for i in 0..n {
        for j in 0..(n-1-i as usize) {
            if new_arr[j] > new_arr[j+1] {
                let bubble = new_arr[j];
                new_arr[j] = new_arr[j+1];
                new_arr[j+1] = bubble;
            }
        }
    }

    new_arr
}

#[cfg(test)]
mod tests {
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
