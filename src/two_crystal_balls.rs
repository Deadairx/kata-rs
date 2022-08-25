pub fn find_break(breaks: Vec<bool>) -> i32 {
    let floatN = breaks.len() as f64;
    let jumpAmount = floatN.sqrt() as usize;

    let mut i = jumpAmount;

    while i < breaks.len() {
        if breaks[i] {
            break;
        }

        i += jumpAmount;
    }

    i -= jumpAmount;
    
    let mut j = 0;

    while j <= jumpAmount && i < breaks.len() {
        if breaks[i] {
            return i as i32;
        }

        j += 1;
        i += 1;
    }

    -1
}
