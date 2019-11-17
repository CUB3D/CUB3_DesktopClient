use std::net::TcpStream;
use tungstenite::Message;
use url::Url;
use tungstenite::protocol::frame::coding::Control::Ping;
use tungstenite::protocol::frame::coding::Data::Text;
use serde::{Serialize, Deserialize};
use notify_rust::Notification;


#[derive(Debug, Serialize, Deserialize)]
struct NotificationMessage {
    title: String,
    content: String
}

#[derive(Debug, Serialize, Deserialize)]
struct NotificationPayload {
    key: String,
    value: String
}


#[derive(Debug, Serialize, Deserialize)]
struct NotificationData {
    targetAppID: String,
    message: Option<NotificationMessage>,
    dataPayload: Option<Vec<NotificationPayload>>
}

impl NotificationData {
    fn get_payload(&self, key: &str) -> Option<&String> {
        if let Some(payload) = &self.dataPayload {
            let iter = payload.iter()
                .filter(| items | items.key == key)
                .collect::<Vec<&NotificationPayload>>();

            return  iter.first().map(| payload | &payload.value);
        }


        return None
    }
}

fn handle_notification_payload(notification: NotificationData) {
    if let Some(msg) = &notification.message {
        let notification_handle = Notification::new()
            .appname("CUB3D")
            .summary(msg.title.as_str())
            .body(msg.content.as_str())
            .icon("firefox")
            .show();

        if let Ok(notification_handle) = notification_handle {
            println!("Notification sent successfully");
        }
    }

    if notification.targetAppID == "cub3d.notify" {
        let title = notification.get_payload("title");
        let body = notification.get_payload("body");

        let mut note = Notification::new();
        let mut notification = note
            .appname("CUB3D");

        if let Some(title_text) = title {
            notification = notification.summary(title_text);
        }

        if let Some(body_text) = body {
            notification = notification.body(body_text);
        }

        let notification_handle = notification.show();

        if let Ok(notification_handle) = notification_handle {
            println!("Notification sent successfully");
        }
    }
}

fn handle_message(msg: Message) {
    match msg {
        Message::Text(txt) => {
            println!("Got text msg: {:?}", txt);

            let json = serde_json::from_str::<NotificationData>(txt.as_str());

            if let Ok(data) = json {
                println!("Got notification data: {:?}", data);
                handle_notification_payload(data);
            }
        }
        Message::Binary(_) => {}
        Message::Ping(_) => {}
        Message::Pong(_) => {
            println!("Got server ping")
        }
        Message::Close(_) => {
            println!("Server disconnected")
        }
    }
}

fn main() {
    let client = tungstenite::connect(Url::parse("wss://cbns.cub3d.pw/poll/123456").unwrap());
    match client {
        Ok(stream) => {
            println!("Socket connected");

            let mut socket = stream.0;

            loop {
                let msg = socket.read_message();

                if let Ok(msg) = msg {
                    handle_message(msg);
                }
            }
        }
        Err(err) => println!("{:?}", err)
    }

    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!")
    }
    // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
    // w.set_tooltip(&"Whatever".to_string());
    app.set_icon_from_file(&"/usr/share/gxkb/flags/ua.png".to_string()).ok();
    app.add_menu_item(&"Print a thing".to_string(), |_| {
        println!("Printing a thing!");
    }).ok();
    app.add_menu_item(&"Add Menu Item".to_string(), |window| {
        window.add_menu_item(&"Interior item".to_string(), |_| {
            println!("what");
        }).ok();
        window.add_menu_separator().ok();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
    }).ok();
    println!("Waiting on message!");
    app.wait_for_message();
}
