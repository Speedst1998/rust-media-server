use std::{io, sync::Arc};

use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, Column},
    Application, Command, Element, Error, Length, Sandbox, Settings, Theme,
};
use rusqlite::Error as SqlErr;
use strum_macros::Display;

use crate::db::watched_folders_table::{self, WatchedFolder, WatchedFoldersDb};

pub struct Flags {
    pub watched_folders_db: WatchedFoldersDb,
}

struct Page {
    watched_folders_db: WatchedFoldersDb,
    watched_folders: Vec<WatchedFolder>,
    counter: u8,
    path: String,
    error: Option<MyError>,
}

#[derive(Debug, Clone)]
enum Message {
    Click,
    OpenFilePicker,
    DeleteFilePicker(String),
    SetPath(Result<String, MyError>),
}

pub fn start(flags: Flags) -> Result<(), Error> {
    Page::run(Settings {
        flags,
        id: None,
        window: Default::default(),
        default_font: Default::default(),
        default_text_size: 16.0,
        antialiasing: false,
        exit_on_close_request: true,
    })
}

impl Page {
    fn refresh_folders_list(&mut self) {
        self.watched_folders = self.watched_folders_db.list().unwrap()
    }
}

impl Application for Page {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                watched_folders: flags.watched_folders_db.list().unwrap(),
                watched_folders_db: flags.watched_folders_db,
                counter: 0,
                path: "".to_string(),
                error: Option::None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("cool ass gui")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Click => {
                self.counter = self.counter + 1;
                Command::none()
            }
            Message::OpenFilePicker => Command::perform(pick_file(), Message::SetPath),
            Message::DeleteFilePicker(path) => {
                self.watched_folders_db.delete(&(path)); //TODO deal with this error
                self.refresh_folders_list();
                Command::none()
            }
            Message::SetPath(res) => {
                match res {
                    Ok(path) => {
                        self.watched_folders_db.create(&(path)).unwrap();
                        self.refresh_folders_list();
                        self.path = path;
                    }
                    Err(err) => {
                        self.error = Some(err);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content2: Column<'_, Message, _> = Column::<'_, Message>::new();
        content2 = self
            .watched_folders
            .iter()
            .fold(content2, |acc, watched_folder| {
                acc.push(row![
                    text(format!("{}", watched_folder.path)).width(400),
                    button("-").on_press(Message::DeleteFilePicker(watched_folder.path.clone()))
                ])
            });

        let top_row = row![
            text("Hello, iced!"),
            horizontal_space(Length::Fixed(50.0)),
            text(format!("{}", self.path)),
            button("+").on_press(Message::OpenFilePicker),
            text(format!(
                "err {}",
                match &self.error {
                    Some(err) => format!("{}", err),
                    None => "none".to_string(),
                }
            )),
            content2
        ];

        container(column![top_row, text(format!("{}", self.counter))].padding(10)).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

async fn pick_file() -> Result<String, MyError> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("choose a folder")
        .pick_folder()
        .await
        .ok_or(MyError::DialogClossed)?;
    Ok(path.path().to_string_lossy().to_string())
}

#[derive(Debug, Clone, Display)]
enum MyError {
    DialogClossed,
}
