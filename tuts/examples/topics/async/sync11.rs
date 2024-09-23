//! This is to see `.await` vs `join!` in a unlocked activity
//!
//! Summary:
//! - Use `.await` on spawned tasks when you would want the task-1 to start first.
//! - Use `join!` on spawned tasks when you would want any task to start first.

use tokio::{
    join,
    time::{sleep, Duration, Instant},
};

#[tokio::main]
pub(crate) async fn main() {
    let start = Instant::now();

    let task1 = tokio::spawn(async {
        println!("task-1 started...");
        sleep(Duration::from_secs(4)).await;
        println!("task-1 completed!");
    });
    let task2 = tokio::spawn(async {
        println!("task-2 started...");
        sleep(Duration::from_secs(2)).await;
        println!("task-2 completed!");
    });

    // collect spawned tasks

    // ===== M-1 (start) =====
    // Result: only 1 possibility here:
    // ```sh
    // task-1 started...
    // task-2 started...
    // task-2 completed!
    // task-1 completed!
    // ```
    // task1.await.unwrap();
    // task2.await.unwrap();
    // ===== M-1 (end) =====

    // ===== M-2 (start) =====
    // 2 possibilities here:
    // ```sh
    // task-1 started...
    // task-2 started...
    // task-2 completed!
    // task-1 completed!
    // ```
    // ```sh
    // task-1 started...
    // task-2 started...
    // task-2 completed!
    // task-1 completed!
    // ```
    // Used for concurrent tasks start i.e. anyone can begin first.
    // And accordingly the task would end.
    let _ = join!(task1, task2);
    // ===== M-2 (end) =====

    let elapsed = start.elapsed();

    println!("took {} ms", elapsed.as_millis());
}
