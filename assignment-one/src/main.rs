use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use rand::prelude::*;

// Task struct
struct Task {
    // Task id
    id: u32,
    // Task payload
    payload: String,
}

impl Task {
    // Create a task
    fn create_task(id: u32, payload: String) -> Task {
        Task { id, payload }
    }
}

// Worker struct
struct Worker {
    id: u32,
}

impl Worker {
    // Create a worker
    fn create_worker(id: u32) -> Worker {
        Worker { id }
    }

    // Process a task
    fn process_task(&self, task: Task) -> &'static str {
        println!(
            "worker {} processing task {} payload {}",
            self.id, task.id, task.payload
        );
        // Random sleep time
        let rand_time = random::<u64>() % 10;
        std::thread::sleep(std::time::Duration::from_secs(rand_time));
        "processed"
    }
}

fn main() {
    // Create vector of tasks
    let tasks = vec![
        Task::create_task(1, "task 1".to_string()),
        Task::create_task(2, "task 2".to_string()),
        Task::create_task(3, "task 3".to_string()),
        Task::create_task(4, "task 4".to_string()),
        Task::create_task(5, "task 5".to_string()),
    ];
    // Worker thread count
    let worker_thread_count = 3;

    // Create channels to send tasks and receive results
    let (task_tx, task_rx) = mpsc::channel::<Task>();
    let (result_tx, result_rx) = mpsc::channel();

    // Create a shared receiver for all worker threads
    let task_rx = Arc::new(Mutex::new(task_rx));
    for _ in 0..worker_thread_count {
        let result_tx_clone = result_tx.clone();
        let task_rx_clone = task_rx.clone();
        let _ = thread::spawn(move || loop {
            // Create a worker and assign a random id
            let worker = Worker::create_worker(random::<u32>() % 100);
            // Random timeout for receiving a task
            let timeout = random::<u64>() % 10;
            // Try Locking the receiver
            if let Ok(locked_rcvr) = task_rx_clone.try_lock() {
                // Receive a task
                if let Ok(task) = locked_rcvr.recv_timeout(Duration::from_secs(timeout)) {
                    // Unlock the receiver
                    drop(locked_rcvr);
                    println!("worker {} got task {}", worker.id, task.id);
                    let result = worker.process_task(task);
                    let _ = result_tx_clone.send(format!(
                        "worker {} processed task result {}",
                        worker.id, result
                    ));
                }
            }
        });
    }
    // Send tasks to the worker threads
    for task in tasks {
        let _ = task_tx.send(task);
    }
    // Receive results from the worker threads
    for msg in result_rx {
        println!("task result: {}", msg);
    }
}
