use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};
use std::collections::HashMap;


// iced latest in git expects to return iced::Result; 0.1 does not.
pub fn main() /*-> iced::Result*/ {
    Client::run(Settings::default())
}

struct IRCMessage {
    raw: String,
    text: String,
    recipient: String,
    msg_type: String,
}

impl IRCMessage {
    pub fn new(message: String) -> Self {
        let mut msg = Self {
            raw: String::from(message),
            text: String::from(""),
            recipient: String::from(""),
            msg_type: String::from(""),
        };
        msg.parse();
        msg
    }

    fn parse(&mut self) {
        // todo
    }
}

struct IRCChannel {
    name: String,
    messages: Vec<IRCMessage>
}

impl IRCChannel {
    fn new(name: String) -> Self {
        Self { name: String::from(name), messages: Vec::new() }
    }

    fn receive(&mut self, msg: IRCMessage) {
        self.messages.push(msg)
    }
}

struct IRCServer {
    name: String,
    address: String,
    port: i32,
    channels: HashMap<String, IRCChannel>
}

impl IRCServer {
    fn new(address: &'static str, port: i32) -> Self {
        Self {
            name: String::from(address),
            address: String::from(address),
            port: port,
            channels: HashMap::new()
        }
    }

    fn send(&mut self, text: String) {
        println!("{}", text)
    }

    // TODO: support channel keys
    fn join(&mut self, channel: String) {
        self.send(format!("JOIN {}", channel));
        let chan = IRCChannel::new(channel);
        self.channels.insert(channel, chan)
    }

    fn receive(&mut self, message: String) {
        let msg = IRCMessage::new(message);
        self.channels.insert(msg.channel, msg)
    }
}

#[derive(Default)]
struct Client {
    servers: Vec<IRCServer>,
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Client {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Client - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
