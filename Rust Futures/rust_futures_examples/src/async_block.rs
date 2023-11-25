use futures::executor::block_on;

pub async fn run_async_block() {
    let my_future = async {
        // 执行一些异步操作
        println!("processing...");
    };

    block_on(my_future);
}
