fn vowel_count(s: &str) -> usize {
    let mut count = 0usize;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
            }
            _ => {
                // no op
            }
        }
    }
    return count
}

fn binary_search(arr : &[usize], target: usize) -> bool {
    let mut lo = 0usize;
    let mut hi = arr.len() - 1;
    while lo <= hi {
        let mut mid = lo + (hi - lo) / 2;
        if arr[mid] == target {
            return true;
        } else if arr[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    return false;
}
