use opencv as cv;

fn main() {
    println!(
        "open cv version: {}",
        cv::core::get_version_string().unwrap()
    );
}
