
fn median(arr: &[isize]) -> isize {
    let len = arr.len();
    let mid = &arr[len/2];
    return if len & 1 > 0 {
        *mid
    } else {
        let other_mid = &arr[len / 2 - 1];
        (mid + other_mid) / 2
    }
}

fn merge(arr_one: &[isize], arr_two: &[isize]) -> Vec<isize> {
    let n: usize = arr_one.len(); // length of arr_one
    let m: usize = arr_two.len(); // length of arr_two
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
    merged_arr
}
pub fn find_median(arr_one: &[isize], arr_two: &[isize]) -> isize {
    let merged_arr = merge(arr_one, arr_two);
    return median(&merged_arr);
}