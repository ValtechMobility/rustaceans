use tokio::task;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {

    let common_counter = 0;
    let mut counter_a = 0;
    let mut counter_b = 0;

    let common_counter_mutex = Arc::new(Mutex::new(common_counter));

    let ccm_a = Arc::clone(&common_counter_mutex);
    let ccm_b = Arc::clone(&common_counter_mutex);

    let task_a = task::spawn(
        async move {
            for _ in 0..10 {
                counter_a += 1;
                let mut ccm = ccm_a.lock().unwrap();
                *ccm += 1;

                println!("counter a: {}", counter_a);
                println!("common counter: {}", *ccm);
            }
        }
    );

    let task_b = task::spawn(
        async move {
            for _ in 0..10 {
                counter_b += 1;
                let mut ccm = ccm_b.lock().unwrap();
                *ccm += 1;

                println!("counter b: {}", counter_b);
                println!("common counter: {}", *ccm);
            }
        }
    );
    let _ = tokio::join!(task_a, task_b);
}
