use anyhow::Result;
use opencv::{highgui, prelude::*, videoio};

fn main() -> Result<()> {
    let window_name = "video capture";
    highgui::named_window(window_name, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window_name, &frame)?;
        }
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}
