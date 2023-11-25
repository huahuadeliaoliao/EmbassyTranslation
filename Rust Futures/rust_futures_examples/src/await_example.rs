pub async fn fetch_data() -> String {
    "data from fetch".to_string()
}

pub async fn process_data(data: String) {
    println!("Processing {}", data);
}

pub async fn fetch_and_process() {
    let data = fetch_data().await;
    process_data(data).await;
}
