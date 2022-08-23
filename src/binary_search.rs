pub fn binary_search(haystack: &Vec<i32>, needle: i32) -> bool {
    let mut lo = 0;
    let mut hi = haystack.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;
        let value = haystack[m];

        if value == needle {
            return true;

        } else if value > needle {
            hi = m;
        } else {
            lo = m + 1;
        }
    }

    false
}
