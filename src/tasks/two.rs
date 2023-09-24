use std::thread;
use std::time::Duration;

// Define a generic function to increment each element in a collection by n
fn increment_by_n<T>(collection: Vec<T>, n: T) -> Vec<T>
    where
        T: std::ops::Add<Output = T> + Copy,
{
    collection.iter().map(|&x| x + n).collect()
}

// Define a generic function to filter elements based on a closure's condition
fn filter_by_condition<T, F>(collection: Vec<T>, condition: F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
{
    collection.into_iter().filter(|x| condition(x)).collect()
}

// Define a generic function to transform elements to strings based on a closure
fn transform_to_string<T, F>(collection: Vec<T>, transform: F) -> Vec<String>
    where
        F: Fn(T) -> String,
{
    collection.into_iter().map(|x| transform(x)).collect()
}

fn run() {
    // Example usage of data transformation functions
    let numbers = vec![1, 2, 3, 4, 5];

    let incremented = increment_by_n(numbers.clone(), 10);
    println!("Incremented by 10: {:?}", incremented);

    let filtered = filter_by_condition(numbers.clone(), |&x| x % 2 == 0);
    println!("Filtered (even numbers only): {:?}", filtered);

    let to_string = transform_to_string(numbers.clone(), |x| x.to_string());
    println!("Transformed to strings: {:?}", to_string);


    let expensive_closure = |num: u32| -> u32 {
        thread::sleep(Duration::from_secs(2));
        num
    };

    // let add_one_v1 = |x: u32| -> u32 {x + 1};
    // let add_one_v2 = |x|{x + 1};
    // let add_one_v3 = |x| x + 1;

}