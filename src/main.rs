mod linear_search_test;
mod linear_search;
mod binary_search_test;
mod binary_search;
mod two_crystal_balls_test;
mod two_crystal_balls;

fn main() {
}

fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut new_arr = arr.clone();

    for i in 0..new_arr.len() {
        for j in 0..new_arr.len()-1-i {
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
