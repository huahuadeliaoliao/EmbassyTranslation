use futures::join;

async fn task_one() -> String {
    "task one complete".to_string()
}

async fn task_two() -> String {
    "task two complete".to_string()
}

pub async fn multiple_tasks() {
    let future_one = task_one();
    let future_two = task_two();

    let (result_one, result_two) = join!(future_one, future_two);

    println!("{}, {}", result_one, result_two);
}
