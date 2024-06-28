use std::sync::{Arc, Mutex};
use std_msgs::msg::String as StringMsg;

struct RepublisherNode {
    node: Arc<rclrs::Node>,
    _subscription: Arc<rclrs::Subscription<StringMsg>>,
    data: Arc<Mutex<Option<StringMsg>>>,
    publisher: Arc<rclrs::Publisher<StringMsg>>,
}

impl RepublisherNode {
    fn republish(&self) -> Result<(), rclrs::RclrsError> {
        if let Some(s) = &*self.data.lock().unwrap() {
            self.publisher.publish(s)?;
        }
        Ok(())
    }

    fn new(context: &rclrs::Context) -> Result<Self, rclrs::RclrsError> {
        let node = rclrs::Node::new(context, "republisher")?;
        let data = Arc::new(Mutex::new(None));
        let data_cb = Arc::clone(&data);
        let _subscription = node.create_subscription(
            "in_topic",
            rclrs::QOS_PROFILE_DEFAULT,
            move |msg: StringMsg| {
                *data_cb.lock().unwrap() = Some(msg);
            },
        )?;
        let publisher = node.create_publisher("out_topic", rclrs::QOS_PROFILE_DEFAULT)?;
        Ok(Self {
            publisher,
            node,
            _subscription,
            data,
        })
    }
}

fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let republisher = Arc::new(RepublisherNode::new(&context)?);

    let republisher_other_thread = Arc::clone(&republisher);
    std::thread::spawn(move || -> Result<(), rclrs::RclrsError> {
        loop {
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(1000));
            republisher_other_thread.republish()?;
        }
    });
    rclrs::spin(Arc::clone(&republisher.node))
}
