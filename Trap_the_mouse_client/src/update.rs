
use crate::messages::{Message};
use crate::model::{Model};
use iced::window::Event ;
use crate::forme::{set_tema, Tema};
use crate::messages::Message::SetPin;

pub fn update(message: Message, model: &mut Model, dimensions: &mut (u32, u32), tema: &mut Tema) {

    match message {
        Message::EMERALD=>set_tema(1,tema),
        Message::CANDY=>set_tema(2,tema),
        Message::AQUA=>set_tema(3,tema),
        Message::DEFAULT=>set_tema(4,tema),
        Message::SNIPER=>set_tema(5,tema),
        Message::NavigateToMenu => model.go_to_menu(),
        Message::NavigateToSinglePlayer => model.go_to_single_player(),
        Message::NavigateToTwoPlayers => model.go_to_two_players(),
        Message::NavigateToJoin => model.go_to_join(),
        Message::NavigateToCreate => model.go_to_create(),
        Message::NavigateToGameBoard => model.go_to_game_board(),
        Message::NavigateToOption => model.go_to_option(),
        Message::Exit => std::process::exit(0),
        Message::SetRole(role) => model.set_role(role),
        Message::SetPin(pin) => model.set_pin(pin),
        Message::GotDimensions(size)=>
            {
                *dimensions = (size.unwrap().width as u32, size.unwrap().height as u32);
            }
        Message::Resize(size) => {
            // Actualizam dimensiunile pe baza mesajului
            *dimensions = (size.width as u32, size.height as u32);
            println!("Dimensiunea ferestrei a fost actualizata: {}x{}", size.width, size.height);
        },
        Message::Resized(size) => {
            // Actualizam dimensiunile pe baza mesajului
            *dimensions = (size.width as u32, size.height as u32);
            println!("Dimensiunea ferestrei a fost actualizata: {}x{}", size.width, size.height);
        }
        _ => {}
    }
}
