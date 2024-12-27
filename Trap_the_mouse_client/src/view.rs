use std::any::Any;

use iced::{widget::canvas::{self, Canvas, Frame, Geometry, Path}, Color, Length, Point, Rectangle, Renderer, Theme, Alignment, Top, Center, Pixels};
use iced::window;
use iced::widget::{Column, Button, Stack, Container, Row, container};
use iced::{widget, Element};
use iced::mouse::Interaction::Idle;
use iced::wgpu::naga::ImageQuery::Size;
use iced::widget::{Text, button};
use iced::widget::container::background;
use iced::window::{get_latest, get_size, Id, Settings};
use crate::{forme, Model};
use crate::model::Page;
use crate::model::Role;
use crate::messages::Message;
use crate::forme::{button_style, ShapeCollection, ShapeType};
use crate::forme::ShapeType::Hexagon;
use std::sync::Arc;
use iced::advanced::text::Difference::Shape;

pub fn view<'a>(model: &'a Model, dimensions: &'a (u32, u32),tema: & 'a forme::Tema) -> Element<'a, Message> {
    match model.current_page {
        Page::Menu => view_menu(model, dimensions, tema),
        Page::SinglePlayer => view_single_player(model),
        Page::TwoPlayers => view_two_players(model),
        Page::Join => view_join(model),
        Page::Create => view_create(model),
        Page::GameBoard => view_game_board(model),
        Page::Option =>view_option(model, dimensions, tema),
    }
}

fn view_menu<'a>(model: &'a Model, dimensions: &'a (u32, u32),tema: &'a forme::Tema) -> Element<'a, Message> {
    let dim = dimensions;


    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,tema),
    });

    // Canvas pentru trapez
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);


    // Definirea dreptunghiului (pentru fundal)
    let rectangle = ShapeCollection::new(ShapeType::Rectangle {
        x: 0.0,
        y: 0.0,
        width: dim.0 as f32,
        height: dim.1 as f32,
        culoare: forme::culori(3,tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;
    let status = button::Status::Active;

    let style = forme::button_style(&theme, status,tema);

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Single Player").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::NavigateToSinglePlayer)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        // Setam o inaltime fixa pentru toate butoanele
        .push(Button::new(Text::new("Two Players").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::NavigateToTwoPlayers)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Options").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::NavigateToOption)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Exit").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::Exit)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));

    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Trap The Mouse").size((dimensions.0/25) as f32)).align_x(Alignment::Center)
            .align_y(Alignment::Start)
            .width(Length::Fill))
            .height(Length::Fill)// Aliniaza containerul pe axa Y
        )
        .push(Container::new(buttons)
            .align_x(Center) // Aliniere orizontala
            .align_y(Center) // Aliniere verticala
            .width(Length::Fill)    // Latime completa
            .height(Length::Fill)

        ); // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()
}

fn view_single_player(model: &Model) -> Element<'_, Message> {
    let column = widget::column![
        widget::text("Single Player"),
        widget::button("Back to Menu").on_press(Message::NavigateToMenu),
        view_game_board(model)
        ];
    column.into()
}

fn view_two_players(model: &Model) -> Element<'_, Message> {
    let column = widget::column![
       widget::button("Back to Menu").on_press(Message::NavigateToMenu),
        widget::button("Join").on_press(Message::NavigateToJoin),
        widget::button("Create").on_press(Message::NavigateToCreate)
         ];
    column.into()
}

fn view_join(model: &Model) -> Element<'_, Message> {
    let column = widget::column![
      widget::text("Enter PIN (4 digits)"),

            widget::text_input(
                "PIN", // Eticheta
                &model.pin.clone().unwrap_or_default(), // Valoarea curenta
            ).on_input(Message::SetPin),

        widget::button("Join Game").on_press(Message::NavigateToGameBoard),
        widget::button("Back to Two Players").on_press(Message::NavigateToTwoPlayers)
        ];
    column.into()
}

fn view_create(model: &Model) -> Element<'_, Message> {
    let column = widget::column![
       widget::text("Create PIN and choose Role"),
        widget::button("Create PIN").on_press(Message::SetPin("1234".to_string())), // Generarea PIN-ului pentru test
         widget::button("Mouse").on_press(Message::SetRole(Role::Mouse)),
         widget::button ("Hunter").on_press(Message::SetRole(Role::Hunter)),
         widget::button ("Start Game").on_press(Message::NavigateToGameBoard),
         widget::button("Back to Two Players").on_press(Message::NavigateToTwoPlayers)
        ];
    column.into()
}


pub fn view_game_board(model: &Model) -> Element<'_, Message> {
    // Crearea butoanelor folosind un iterator
    let buttons: Vec<Element<Message>> = (0..121).map(|i| {
        Button::new(Text::new(i.to_string()))
            .on_press(Message::NavigateToMenu)
            .into()  // Transforma butonul intr-un Element<Message>
    }).collect();

    // Construirea unui Column cu copiii generati
    Column::with_children(buttons).into()  // Creaza Column folosind butoanele
}

pub fn view_option<'a>(model: &'a Model, dimensions: & 'a(u32, u32),tema: & 'a forme::Tema) -> Element<'a, Message> {
    let dim = dimensions;


    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,tema),
    });

    // Canvas pentru trapez
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);


    // Definirea dreptunghiului (pentru fundal)
    let rectangle = ShapeCollection::new(ShapeType::Rectangle {
        x: 0.0,
        y: 0.0,
        width: dim.0 as f32,
        height: dim.1 as f32,
        culoare: forme::culori(3,tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;
    let status = button::Status::Active;

    let style = forme::button_style(&theme, status,tema);

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Emerald").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::EMERALD)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        // Setam o inaltime fixa pentru toate butoanele
        .push(Button::new(Text::new("Candy").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::CANDY)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Aqua").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::AQUA)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Default").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::DEFAULT)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
            .push(Button::new(Text::new("Sniper").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::SNIPER)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));

    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Select Theme").size((dimensions.0/25) as f32)).align_x(Alignment::Center)
            .align_y(Alignment::Start)
            .width(Length::Fill))
                  .height(Length::Fill)// Aliniaza containerul pe axa Y
        )
        .push(Container::new(buttons)
            .align_x(Center) // Aliniere orizontala
            .align_y(Center) // Aliniere verticala
            .width(Length::Fill)    // Latime completa
            .height(Length::Fill)

        )
        .push(Button::new(Text::new("Back").align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|theme, status| button_style(theme, status,tema))
            .on_press(Message::NavigateToMenu)
            .width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50)));
        ; // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()
}