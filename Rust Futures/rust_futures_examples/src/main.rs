mod basic_future;
mod async_block;
mod await_example;
mod multiple_futures;

use futures::executor::block_on;

fn main() {
    block_on(basic_future::hello_world());
    block_on(async_block::run_async_block());
    block_on(await_example::fetch_and_process());
    block_on(multiple_futures::multiple_tasks());
}
