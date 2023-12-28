use iced::{widget::text, Element, Error, Sandbox, Settings};

struct Page;

#[derive(Debug)]
enum Message {}

pub fn start() -> Result<(), Error> {
    Page::run(Settings::default())
}

impl Sandbox for Page {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("cool ass gui")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        text("Hello, iced!").into()
    }
}
