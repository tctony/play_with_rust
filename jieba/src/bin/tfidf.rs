use jieba_rs::{Jieba, KeywordExtract, TfIdf};

fn main() {
    let jieba = Jieba::new();
    let extractor = TfIdf::default();
    let top_k = extractor.extract_keywords(
        &jieba,
        "今天纽约的天气真好啊，京华大酒店的张尧经理吃了一只北京烤鸭。后天纽约的天气不好，昨天纽约的天气也不好，北京烤鸭真好吃",
          3,
          vec![],
    );
    println!("{:?}", top_k);
}
