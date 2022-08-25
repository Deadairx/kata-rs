pub fn binary_search(haystack: &Vec<i32>, needle: i32) -> bool {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let mid = low + (high - low) / 2;

        let value = haystack[mid];

        if value == needle {
            return true;
        } else if value < needle {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    false
}
