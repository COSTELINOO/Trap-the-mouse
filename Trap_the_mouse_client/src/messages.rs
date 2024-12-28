use crate::model::{ Role};
use iced::window::Event as WindowEvent;
use iced::{executor, Application, Element, Settings, window,application};
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
  Resized(iced::Size),
    Resize(iced::Size),
    EventOccurred(iced::Event),
    None,
    GotDimensions(Option<iced::Size>),
    NoDimensions,
    EMERALD,
    CANDY,
    AQUA,
    DEFAULT,

    COFFEE,
    PRINCESS
}
