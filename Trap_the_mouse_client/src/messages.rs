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
    EMERALD,
    CANDY,
    AQUA,
    DEFAULT,
    COFFEE,
    PRINCESS,
    Received(String),
}
