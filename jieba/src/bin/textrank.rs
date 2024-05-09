use jieba_rs::{Jieba, KeywordExtract, TextRank};

fn main() {
    let jieba = Jieba::new();
    let extractor = TextRank::default();
    let top_k = extractor.extract_keywords(
        &jieba,
        "此外，公司拟对全资子公司吉林欧亚置业有限公司增资4.3亿元，增资后，吉林欧亚置业注册资本由7000万元增加到5亿元。吉林欧亚置业主要经营范围为房地产开发及百货零售等业务。目前在建吉林欧亚城市商业综合体项目。2013年，实现营业收入0万元，实现净利润-139.13万元。",
        6,
        vec![String::from("ns"), String::from("n"), String::from("vn"), String::from("v")],
    );
    println!("{:?}", top_k);
}
