pub fn visible_len(s: &str) -> usize {
    let mut count = 0;
    let mut skip = false;
    for ch in s.chars() {
        if skip {
            if ch == 'm' {
                skip = false;
            }
            continue;
        }
        if ch == '\x1b' {
            skip = true;
            continue;
        }
        count += 1;
    }
    count
}