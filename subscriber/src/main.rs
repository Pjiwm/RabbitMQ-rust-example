use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, Subscriber};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct Message {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    let listener =
        CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
        // let msg = Arc::new(Subscriber::<Message>::new());
        // let msg = Subscriber::<Message>::new();
        let x: Subscriber<Message> = Subscriber::new();
    // let user_handler = Arc::new(UserCreatedHandler::default());
    _ = listener.listen(
        "user_created".to_owned(),
        x.clone(),
        crosstown_bus::QueueProperties {
            auto_delete: false,
            durable: false,
            use_dead_letter: true,
        },
    );

    loop {
        println!("{:?}", x.get_subscribed());
        std::thread::sleep(std::time::Duration::from_secs(1));

    }
}
