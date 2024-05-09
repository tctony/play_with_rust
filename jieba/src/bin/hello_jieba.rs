use jieba_rs::Jieba;

fn main() {
    let jieba = Jieba::new();
    let words = jieba.cut("我们中出了一个叛徒", false);
    println!("{:?}", words);
}
