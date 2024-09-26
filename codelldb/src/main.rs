fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    loop {
        let a = 1;
        let b = a + 1;
        let c = b + 1;
        println!("{} {} {}", a, b, c);

        rt.block_on(async {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        });
    }
}
