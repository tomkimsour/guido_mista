use anyhow::Result;
use opencv::{highgui, prelude::*, videoio};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
// use std_msgs::msg::String as StringMsg;

struct PerceptionNode {
    node: Arc<rclrs::Node>,
    cam: videoio::VideoCapture,
    window: String,
}

impl PerceptionNode {
    fn new(context: &rclrs::Context) -> Result<Self> {
        let node = rclrs::Node::new(context, "perception")?;
        let window = "video capture";
        highgui::named_window("Video capute", highgui::WINDOW_AUTOSIZE)?;
        // 0 is the default camera
        let cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
        let opened = videoio::VideoCapture::is_opened(&cam)?;
        if !opened {
            panic!("Unable to open default camera!");
        }
        Ok(Self {
            node,
            cam,
            window: window.into(),
        })
    }
    fn republish(&mut self) -> Result<()> {
        loop {
            let mut frame = Mat::default();
            self.cam.read(&mut frame)?;
            if frame.size()?.width > 0 {
                highgui::imshow(&self.window.as_str(), &frame)?;
            }
            let key = highgui::wait_key(10)?;
            if key > 0 && key != 255 {
                break;
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let context = rclrs::Context::new(std::env::args())?;
    let perception_node = Arc::new(Mutex::new(PerceptionNode::new(&context)?));
    let mut republisher_other_thread = Arc::clone(&perception_node);
    republisher_other_thread.lock().unwrap().republish()?;

    // let executor = rclrs::SingleThreadedExecutor::new();

    // Map the error for anyhow::Error https://users.rust-lang.org/t/anyhow-result-fails-to-compile-expected-struct/46687
    // rclrs::spin(Arc::clone(&perception_node.node)).map_err(|err| err.into())
    Ok(())
}
