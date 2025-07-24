#![warn(unused_variables)]
use crate::model::{Role};

#[derive(Debug, Clone)]
pub enum Message {
    NavigateToMenu,
    NavigateToSinglePlayer,
    NavigateToTwoPlayers,
    NavigateToJoin,
    NavigateToCreate,
    NavigateToGameBoard,
    NavigateToOption,
    ButtonPressed(u32),
    Exit,
    SetRole(Role),
    SetPin(String),
    Resize(iced::Size),
    None,
    Emerald,
    Candy,
    Aqua,
    Default,
    Coffee,
    Princess,
    Received(String),
}
