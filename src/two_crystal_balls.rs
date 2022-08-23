pub fn find_break(breaks: Vec<bool>) -> i32 {
    let n_float = breaks.len() as f64;
    let jump_amount = n_float.sqrt() as usize;

    let mut i = jump_amount;
    
    while i < breaks.len() {
        if breaks[i] {
            break;
        }

        i += jump_amount;
    }
    i -= jump_amount;

    let mut j = 0;
    while j <= jump_amount && i < breaks.len() {
        if breaks[i] {
            return i as i32;
        }

        j += 1;
        i += 1;
    }

    -1
}
