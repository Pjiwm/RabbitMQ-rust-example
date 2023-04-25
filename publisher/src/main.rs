use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let mut p =
        match CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()) {
            Ok(p) => p,
            Err(e) => {
                println!("Error creating publisher: {:?}", e);
                return;
            }
        };
    let mut idx = 0;
    loop {
        let names = vec![
            "Gamer", "Coder", "Hacker", "Nerd", "Coomer", "Weeb", "Otaku", "Loser", "Dork", "Geek",
        ];
        std::thread::sleep(std::time::Duration::from_secs(2));
        let a = p.publish_event(
            "user_created".to_owned(),
            UserCreatedEventMessage {
                user_id: "5".to_owned(),
                user_name: String::from(names[idx]),
            },
        );
        if let Err(e) = a {
            println!("Error publishing event: {:?}", e);
        }
        idx += 1;
        if idx > 9 {
            idx = 0;
        }
    }
}
