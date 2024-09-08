use futures::executor::block_on;

async fn do_sth() {
    println!("do sth");
}

#[tokio::main]
async fn main() {
    block_on(do_sth());
}
