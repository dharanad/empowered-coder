use std::cmp::Ordering;

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

pub fn reverse_vowels(s: String) -> String {
    let is_vowel = |b: u8| matches!(b, b'a' | b'e' | b'o' | b'i' | b'u' | b'A' | b'E' | b'O' | b'I' | b'U');
    let mut arr = s.into_bytes();
    let (mut left, mut right) = (0, arr.len() - 1);
    while left < right {
        while left < right && !is_vowel(arr[left]) {
            left += 1;
        }
        while left < right && !is_vowel(arr[right]) {
            right -= 1;
        }
        if left < right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    String::from_utf8(arr).unwrap()
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut res : Vec<bool> = Vec::new();
    let max = candies.iter().max().unwrap();
    for x in candies.iter() {
        res.push(x + extra_candies >= *max);
    }
    return res;
}


pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, numbers.len() - 1);
    while left < right {
        let sum = numbers[left] + numbers[right];
        match sum.cmp(&target) {
            Ordering::Equal => return vec![(left + 1) as i32, right + 1],
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }
    vec![]
}

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut buffer: Vec<i32> = Vec::new();
    let mut _candidates = candidates.clone();
    _candidates.sort_unstable();
    solve(&_candidates, target, 0, &mut buffer, &mut res);
    return res;
}

fn solve(arr: &[i32], target: i32, idx : usize, buffer: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    let len = arr.len();
    if idx == len || target == 0 {
        if target == 0 {
            res.push(buffer.clone());
        }
        return;
    }
    for i in idx..len {
        if i > idx && arr[i] == arr[i-1] {
            continue;
        }
        match target.cmp(&arr[i]) {
            Ordering::Greater | Ordering::Equal => {
                buffer.push(arr[i]);
                solve(arr, target - arr[i], i + 1, buffer, res);
                buffer.pop();
            }
            _ => ()
        }
    }
}