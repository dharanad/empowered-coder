mod task_one;
mod misc;
mod task_three;

fn main() {
    let a = vec![1,4,6];
    let b = vec![];
    let median = task_three::find_median(&a, &b);
    println!("{median}")
}
