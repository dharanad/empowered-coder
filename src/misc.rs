use std::cmp::max;

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

pub fn merge_alternately(word1: String, word2: String) -> String {
    let n = word1.len().max(word2.len());
    let mut m = String::new();
    let mut word1 = word1.chars();
    let mut word2 = word2.chars();
    for _ in 0..n {
        if let Some(c) = word1.next() {
            m.push(c);
        }
        if let Some(c) = word2.next() {
            m.push(c);
        }
    }
    m
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut res : Vec<bool> = Vec::new();
    let max = candies.iter().max().unwrap();
    for x in candies.iter() {
        res.push(x + extra_candies >= *max);
    }
    return res;
}
