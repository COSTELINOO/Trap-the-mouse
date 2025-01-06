#![warn(unused_variables)]

mod model;
mod update;
mod view;
mod messages;
mod forme;
mod to_server;
mod parser;
use model::*;
use parser::*;
use update::*;
use view::*;
use messages::*;
use std::env;
use iced::Task;
use iced::window::{Event, Level, Position, settings::PlatformSpecific};
use iced::window;
use iced::Subscription;
use tokio::sync::mpsc as tokio_mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;


pub struct MyApp {
    matrix: [[i8; 11]; 11],
    model: Model,
    dimensions: (u32, u32),
    tema: forme::Tema,
    sender: Option<tokio_mpsc::Sender<String>>,
    receiver: Option<Arc<Mutex<tokio_mpsc::Receiver<String>>>>,
    winner: Option<String>,
    ready_room: bool,
    locatie: String,
}


impl MyApp {
    fn new() -> (Self, Task<Message>) {
        let (tx, server_rx) = tokio_mpsc::channel(32);

        let exe_path = env::current_exe().expect("");
        let mut file_path = exe_path.to_str().unwrap().to_string().replace("Trap_the_mouse_client.exe", "");
        file_path.push_str("../../files/mouse.png");


        let receiver = Arc::new(Mutex::new(server_rx));
        let receiver_for_task = Arc::clone(&receiver);

        let command = Task::perform(async move {
            let res = do_async_work(receiver_for_task).await;
            Message::Received(res)
        }, |msg| msg);

        let app = MyApp {
            ready_room: false,
            matrix: [[0i8; 11]; 11],
            model: Model {
                current_page: Page::Menu,
                role: Some(Role::Hunter),
                current_move: Some(Role::Hunter),
                pin: None,
            },
            dimensions: (800, 600),
            tema: forme::Tema {
                colors: [
                    "2a3517".to_string(),
                    "8f9c77".to_string(),
                    "cfe1b9".to_string(),
                    "e7f5dc".to_string(),
                ],
            },
            sender: Some(tx),
            receiver: Some(receiver),
            winner: None,
            locatie: file_path,
        };
        (app, command)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        update(message, self)
    }

    fn view(&self) -> iced::Element<'_, Message> {
        view(self)
    }

    fn subscription(&self) -> Subscription<Message> {
        window::events().map(|(_, event)| match event {
            Event::Resized(size) => Message::Resize(size),
            _ => Message::None,
        })
    }

    fn default(&mut self) {
        self.matrix = [[0i8; 11]; 11];
        self.model = Model {
            current_page: Page::Menu,
            role: Some(Role::Hunter),
            current_move: Some(Role::Hunter),
            pin: None,
        };
        self.ready_room = false;
        self.winner = None;
    }

    fn get_locatie(&self) -> String {
        self.locatie.clone()
    }
}

#[tokio::main]
async fn main() -> iced::Result {
    let app = MyApp::new();

    iced::application("Trap The Mouse", MyApp::update, MyApp::view)
        .window(window::Settings {
            size: iced::Size::new(800.0, 670.0),
            position: Position::Centered,
            min_size: Some(iced::Size::new(670.0, 670.0)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: Level::default(),
            icon: Some(window::icon::from_file(app.0.get_locatie()).expect("Failed to load icon")),
            platform_specific: PlatformSpecific::default(),
            exit_on_close_request: bool::default(),
        }).subscription(MyApp::subscription).run_with(|| app)
}
