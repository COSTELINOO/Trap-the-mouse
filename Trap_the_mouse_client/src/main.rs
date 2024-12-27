mod model;
mod update;
mod view;
mod messages;
mod forme;
use iced::{Error, Font, Pixels, Program, Settings, Size, Task};
use model::Model;

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
use iced_futures::executor;
use futures::executor::block_on;
use crate::forme::set_tema;

#[derive(Debug,Clone)]
pub struct MyApp {
    model: Model,
    dimensions: (u32, u32),
    pub id: Id,
    pub id_tema:u32,
    pub tema: forme::Tema,

}

impl  MyApp {

   pub fn new() -> Self {
        MyApp {
            model: Model { current_page: Page::Menu, role: None, pin: None },
            dimensions: (800, 600),id: Id::unique(),id_tema:4,
            tema: forme::Tema{COLORS: [
                "1b1b1b".to_string(),"77abb6".to_string(), "f1ac20".to_string(),"023336".to_string()
            ],},


        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {

        update(message, &mut self.model,&mut self.dimensions,& mut self.tema);

        Task::none()
    }

    fn view(& self) -> iced::Element<'_,Message> {


        view(&self.model,&self.dimensions,&self.tema)
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

}
impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            model: Model {
                current_page: Page::Menu,
                role: None,
                pin: None
            },
            dimensions: (800, 600),
            id: Id::unique(),
            id_tema:4,
            tema: forme::Tema{COLORS: [
                "0233336".to_string(),
                "4da674".to_string(),
                "c1e6ba".to_string(),
                "eaf8e7".to_string(),
            ]}
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
fn main() ->iced:: Result{
    // Configurarea aplicatiei

    // Ruleaza aplicatia cu dimensiunea configurata
   iced::application("Counter Example", MyApp::update, MyApp::view)
        .window(window::Settings {
            size: iced::Size::new(800.0, 600.0),  // Setezi dimensiunea ferestrei
            position: Position::Centered,    // Pozitia implicita a ferestrei
            min_size: Some(iced::Size::new(660.0, 600.0)),                 // Fereastra nu are dimensiune minima
            max_size: None,                 // Fereastra nu are dimensiune maxima
            visible: true,                  // Fereastra este vizibila
            resizable: true,                // Fereastra este redimensionabila
            decorations: true,              // Fereastra are decoratiuni
            transparent: false,             // Fereastra nu este transparenta
            level: Level::default(),        // Nivelul ferestrei este implicit
            icon: None,                     // Fereastra nu are iconita
            platform_specific: PlatformSpecific::default(), // Setari platforma specifica
            exit_on_close_request: bool::default(),         // Comportament implicit la inchiderea ferestrei
        }) .subscription(MyApp::subscription) .run_with(|| (MyApp::new(), Task::none()))  // Rulam aplicatia cu starea initiala

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
