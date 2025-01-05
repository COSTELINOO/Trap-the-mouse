mod model;
mod update;
mod view;
mod messages;
mod forme;
mod to_server;
mod parser;


use to_server::*;
use iced::{Error, Font, Pixels, Program, Settings, Size, Task};
use model::Model;
use iced::Application;
use model::Page;
use messages::Message;
use update::update;
use view::view;
use  iced::application;
use iced::window::{settings, Event, Level, Position};
use iced::window;
use iced::window::Event as WindowEvent;
use iced::Subscription;
use iced::window::Id;
use iced::advanced::subscription;
use iced::window::settings::PlatformSpecific;
use std::borrow::Cow;
use iced::Executor;
use iced::futures;
use iced_futures::backend::default;

use futures::executor::block_on;
use crate::forme::set_tema;
use crate::model::Role;
use iced::futures::channel::mpsc;

use iced::executor;
use tokio::net::TcpStream;
use tokio::sync::mpsc as tokio_mpsc;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use winit::event_loop::EventLoop;




pub struct MyApp {
    matrix:[[i8; 11]; 11],
    model: Model,
    dimensions: (u32, u32),
    pub id: Id,
    pub id_tema:u32,
    pub tema: forme::Tema,
    pub sender: Option<tokio_mpsc::Sender<String>>, // Canal pentru a trimite mesaje
    receiver: Option<Arc<Mutex<tokio_mpsc::Receiver<String>>>>,
    winner: Option<String>,
    ready_room:bool
}


async fn do_async_work(rx: Arc<Mutex<tokio_mpsc::Receiver<String>>>) -> String {
    let mut rx = rx.lock().await;  // Așteaptă pentru a obține lock-ul
    rx.recv().await.unwrap_or_else(|| "".to_string())
}


impl  MyApp {


    fn new() -> (Self, Task<Message>) {
        let (tx, rx) = tokio_mpsc::channel(32); // Canal pentru comenzi
        let (server_tx, server_rx) = tokio_mpsc::channel(32); // Canal pentru răspunsuri de la server

        // Lansăm thread-ul TCP
      /*  tokio::spawn(async move {
            if let Err(e) = to_server::tcp_handler(rx, server_tx).await {
                eprintln!("Eroare în thread-ul TCP: {}", e);
            }
        });*/

        let receiver = Arc::new(Mutex::new(server_rx));
        let receiver_for_task = Arc::clone(&receiver);
        let command = Task::perform(async move{
            // Exemplu de task asincron
            let res = do_async_work(receiver_for_task).await;
            Message::Received(res)
        }, |msg| msg);

    let app= MyApp {
        ready_room: false,
            matrix:[[0i8;11];11],
            model: Model {
                current_page: Page::Menu,
                role: Some(Role::Hunter),
                current_move:Some(Role::Hunter),
                pin: None,
            },
            dimensions: (800, 600),
            id: Id::unique(),
            id_tema: 4,
            tema: forme::Tema {
                COLORS: [
                    "2a3517".to_string(),
                    "8f9c77".to_string(),
                    "cfe1b9".to_string(),
                    "e7f5dc".to_string(),
                ],
            },
            sender: Some(tx),
            receiver: Some(receiver),
        winner:None
        };
        (app,command)

    }

    fn update(&mut self, message: Message) -> Task<Message> {

        update(message, self)


    }

    fn view(& self) -> iced::Element<'_,Message> {


        view(&self)
    }

    fn subscription(&self) -> Subscription<Message> {
        window::events().map(|(id, event)| match event {
            // Daca evenimentul este de tip Resized, trimite dimensiunile
            iced::window::Event::Resized(size) => Message::Resize(size),
            // În cazul altor evenimente, trimite-le doar ca Message::EventOccurred
            _ => Message::None,
        })

    }


    fn window_event(&mut self, event: Event) {

        match event {
            Event::Resized(size) => {
                self.dimensions.0 = size.width as u32;
                self.dimensions.1 = size.height as u32;
            }
            _ => {}
        }
    }
    fn default(&mut self) {
        self.matrix = [[0i8; 11]; 11];
        self.model = Model {
            current_page: Page::Menu,
            role: Some(Role::Hunter),
            current_move: Some(Role::Hunter),
            pin: None,
        };

self.ready_room=false;

        self.id = Id::unique();


        self.winner = None;
    }

}
impl Default for MyApp {
    fn default() -> Self {

        MyApp {
            ready_room: false,
            matrix:[[0i8;11];11],
            model: Model {
                current_page: Page::Menu,
                role: Some(Role::Hunter),
                current_move:Some(Role::Hunter),
                pin: None,
            },
            dimensions: (800, 600),
            id: Id::unique(),
            id_tema: 4,
            tema: forme::Tema {
                COLORS: [
                    "2a3517".to_string(),
                    "8f9c77".to_string(),
                    "cfe1b9".to_string(),
                    "e7f5dc".to_string(),
                ],
            },sender: None,
            receiver: None,
            winner:None,
        }
    }
}
pub trait Title {
    fn title(&self) -> String;
}
impl Title for MyApp {
    fn title(&self) -> String {
        "My Application Title".to_string()  // Poti sa schimbi acest titlu dupa dorinta
    }
}


#[tokio::main]
async fn main() ->iced:: Result{
    // Configurarea aplicatiei
let mut app=MyApp::new();



    // Ruleaza aplicatia cu dimensiunea configurata
   iced::application("Trap The Mouse", MyApp::update, MyApp::view)
        .window(window::Settings {
            size: iced::Size::new(800.0, 670.0),  // Setezi dimensiunea ferestrei
            position: Position::Centered,    // Pozitia implicita a ferestrei
            min_size: Some(iced::Size::new(670.0, 670.0)),                 // Fereastra nu are dimensiune minima
            max_size: None,                 // Fereastra nu are dimensiune maxima
            visible: true,                  // Fereastra este vizibila
            resizable: true,                // Fereastra este redimensionabila
            decorations: true,              // Fereastra are decoratiuni
            transparent: false,             // Fereastra nu este transparenta
            level: Level::default(),        // Nivelul ferestrei este implicit
            icon: Some( iced::window::icon::from_file("C:\\Users\\Costelino\\Desktop\\Proiect_rust\\Trap_the_mouse_client\\files/mouse.png").expect("Failed to load icon")),                     // Fereastra nu are iconita
            platform_specific: PlatformSpecific::default(), // Setari platforma specifica
            exit_on_close_request: bool::default(),         // Comportament implicit la inchiderea ferestrei
        }) .subscription(MyApp::subscription) .run_with(|| (app))  // Rulam aplicatia cu starea initiala

  /*  application(
        |arg0: &MyApp| "My Application Title".to_string(),
        MyApp::update,         // Functia care actualizeaza aplicatia
        MyApp::view).     window ({
            iced::window::Settings {
                size: Size::new(800.0, 600.0),
                resizable: true,

                ..Default::default()
            }
        }
    ).subscription(MyApp::subscription)
        .run()*/
}
