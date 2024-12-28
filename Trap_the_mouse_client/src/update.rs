
use crate::messages::{Message};
use crate::model::{Model};
use iced::window::Event ;
use crate::forme::{set_tema, Tema};
use crate::messages::Message::SetPin;
use crate::MyApp;

pub fn update(message: Message,app: &mut MyApp) {

    match message {
        Message::EMERALD=>set_tema(1,&mut app.tema),
        Message::CANDY=>set_tema(2,&mut app.tema),
        Message::AQUA=>set_tema(3,&mut app.tema),
        Message::DEFAULT=>set_tema(4,&mut app.tema),
        Message::COFFEE=>set_tema(5,&mut app.tema),
        Message::PRINCESS=>set_tema(6,&mut app.tema),
        Message::NavigateToMenu => app.model.go_to_menu(),
        Message::NavigateToSinglePlayer => app.model.go_to_single_player(),
        Message::NavigateToTwoPlayers => app.model.go_to_two_players(),
        Message::NavigateToJoin => app.model.go_to_join(),
        Message::NavigateToCreate => app.model.go_to_create(),
        Message::NavigateToGameBoard => app.model.go_to_game_board(),
        Message::NavigateToOption => app.model.go_to_option(),
        Message::Exit => std::process::exit(0),
        Message::SetRole(role) =>  app.model.set_role(role),
        Message::SetPin(pin) => app.model.set_pin(pin),
        Message::GotDimensions(size)=>
            {
                *&mut app.dimensions = (size.unwrap().width as u32, size.unwrap().height as u32);
            }
        Message::Resize(size) => {
            // Actualizam dimensiunile pe baza mesajului
            *&mut app.dimensions = (size.width as u32, size.height as u32);
            println!("Dimensiunea ferestrei a fost actualizata: {}x{}", size.width, size.height);
        },
        Message::Resized(size) => {
            // Actualizam dimensiunile pe baza mesajului
            *&mut app.dimensions = (size.width as u32, size.height as u32);
            println!("Dimensiunea ferestrei a fost actualizata: {}x{}", size.width, size.height);
        }
        Message::ButtonPressed(numar)=> {
            // Actualizam dimensiunile pe baza mesajului

            println!("S-a apasdat{}", numar);
        },
        _ => {}
    }
}
