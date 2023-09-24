/// median find the median of element in input array
/// Note: Input array should be sorted in non decreasing order
fn median(arr: Vec<isize>) -> Option<isize> {
    let len = arr.len();
    if len == 0{
        return None
    }
    let mid = &arr[len/2];
    return if len & 1 > 0 {
        Some(*mid)
    } else {
        let other_mid = &arr[len / 2 - 1];
        Some((mid + other_mid) / 2)
    }
}

/// merge function merge the input arrays into one single array in non decreasing order
/// the input vector will be moved as they wont have any used once merged
/// Note: Input array should be sorted in non decreasing order
fn merge(arr_one: Vec<isize>, arr_two: Vec<isize>) -> Option<Vec<isize>> {
    let n: usize = arr_one.len(); // length of arr_one
    let m: usize = arr_two.len(); // length of arr_two
    if n == 0 && m == 0 {
        // there is nothing to merge
        return None
    }
    // output vector to hold merged output of arr_one and arr_two in asc order
    let mut merged_arr = Vec::new();
    // pointer to track current element to merge
    let mut ptr_one = 0;
    let mut ptr_two = 0;
    // let mut ptr = 0; // ptr to track insert of next element in merged_arr
    while ptr_one < n || ptr_two < m {
        if ptr_one < n && ptr_two < m {
            if arr_one[ptr_one] <= arr_two[ptr_two] {
                merged_arr.push(arr_one[ptr_one]);
                ptr_one += 1;
            } else {
                merged_arr.push(arr_two[ptr_two]);
                ptr_two += 1;
            }
        } else if ptr_one < n {
            merged_arr.push(arr_one[ptr_one]);
            ptr_one += 1;
        } else {
            merged_arr.push(arr_two[ptr_two]);
            ptr_two += 1;
        }
    }
    Some(merged_arr)
}

/// Find the median of input two array
/// Assuming that input array might not be sorted
/// this function make a copy of the array of the input array as intended not to have any side
/// effects on input array
pub fn find_median(arr_one: &[isize], arr_two: &[isize]) -> Option<isize> {
    // Sorting can th
    let mut _arr_one = arr_one.clone().to_vec();
    let mut _arr_two = arr_two.clone().to_vec();
    // sort the array
    _arr_one.sort_unstable();
    _arr_two.sort_unstable();
    return match merge(_arr_one, _arr_two) {
        Some(arr) => {
            return median(arr);
        }
        None => None
    }
}