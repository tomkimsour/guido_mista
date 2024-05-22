use anyhow::Result;
use opencv::{highgui, prelude::*, videoio};
use std::sync::{Arc, Mutex};
use std_msgs::msg::String as StringMsg;

struct PerceptionNode {
    node: Arc<rclrs::Node>,
}

impl PerceptionNode {
    fn new(context: &rclrs::Context) -> Result<Self> {
        let node = rclrs::Node::new(context, "perception")?;
        Ok(Self { node })
    }
}

fn main() -> Result<()> {
    let context = rclrs::Context::new(std::env::args())?;
    let perception = Arc::new(PerceptionNode::new(&context)?);
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    let opened = videoio::VideoCapture::is_opened(&cam)?;
    if !opened {
        panic!("Unable to open default camera!");
    }
    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
        }
        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    // Map the error for anyhow::Error https://users.rust-lang.org/t/anyhow-result-fails-to-compile-expected-struct/46687
    rclrs::spin(Arc::clone(&perception.node)).map_err(|err| err.into())
}
