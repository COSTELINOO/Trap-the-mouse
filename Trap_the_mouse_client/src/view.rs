use std::any::Any;
use emojis;
use iced::widget::button::Status;
use iced::{widget::canvas::{self, Canvas, Frame, Geometry, Path}, Color, Length, Point, Rectangle, Renderer, Theme, Alignment, Top, Center, Pixels, theme};
use iced::window;
use iced::widget::{Column, Button, Stack, Container, Row, container, text_input};
use iced::{widget, Element};
use iced::mouse::Interaction::{Idle};
use iced::wgpu::naga::ImageQuery::Size;
use iced::widget::{Text, button};
use iced::widget::container::background;
use iced::window::{get_latest, get_size, Id, Settings};
use crate::{forme, Model, MyApp};
use crate::model::Page;
use crate::model::Role;
use crate::messages::Message;
use crate::forme::{button_style, container_style, culori, text_input_style, ShapeCollection, ShapeType};
use crate::forme::ShapeType::Hexagon;
use std::sync::Arc;
use iced::advanced::text::Difference::Shape;
use iced::advanced::Widget;
use iced::keyboard::on_key_release;
use rusttype::{Font, Scale};
use iced::widget::image::Handle;
use iced::widget::image;
use tokio::net::windows::named_pipe::PipeMode;

pub fn view<'a>( app: &'a MyApp) -> Element<'a, Message> {
    match app.model.current_page {
        Page::Menu => view_menu(app),
        Page::SinglePlayer => view_single_player(app),
        Page::TwoPlayers => view_two_players(app),
        Page::Join => view_join(app),
        Page::Create => view_create(app),
        Page::GameBoard => view_game_board(app),
        Page::Option =>view_option(app),
    }
}

fn view_menu<'a>(app: &'a MyApp) -> Element<'a, Message>
{
    let dim = app.dimensions;


    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,&app.tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,&app.tema),
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
        culoare: forme::culori(3,&app.tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;
    let status = button::Status::Active;

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Single Player").align_x(Alignment::Center).size(25.0)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToSinglePlayer)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        // Setam o inaltime fixa pentru toate butoanele
        .push(Button::new(Text::new("Two Players").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Options").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToOption)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Exit").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::Exit)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));

    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Trap The Mouse").size((app.dimensions.0/25) as f32)).align_x(Alignment::Center)
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
pub fn view_single_player(app: &MyApp) -> Element<Message>
{
    let mut game_over=false;
    let mut winner=String::from("");
    if let Some(ceva)=&app.winner
    {
        game_over=true;
        winner=ceva.clone();

    }

   // if game_over==false
   // {
    let dim = app.dimensions;

    // Definirea dreptunghiului (pentru fundal)
    let rectangle = ShapeCollection::new(ShapeType::Rectangle {
        x: 0.0,
        y: 0.0,
        width: dim.0 as f32,
        height: dim.1 as f32,
        culoare: forme::culori(3, &app.tema), // Folosim culoarea specifică pentru fundal
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

    // Butoane
    let mut column = Column::new().spacing(0); // Fără spațiu între butoane

    for i in 0..11 {
        let mut row = Row::new().spacing(0); // Fără spațiu între butoane

        for j in 0..11 {
            let index = i * 11 + j;

let cnt=app.matrix[((index)/11)as usize][((index)%11) as usize];
            // Adăugăm jumatate de buton de culoarea fundalului in fata randurilor impare pentru a intercala randurile
            if (index % 11 + 1) == 1 && index % 2 == 1 {
                let mut button1 = Button::new(Text::new(""))
                    .style(|_, status| button_style(&Theme::Dark, status,&app.tema, culori(3,&app.tema)))
                    .width(Length::from(25)) // Butonul mai mic pe rândurile impare
                    .height(Length::from(50));
                row = row.push(button1);
            }
            //cream buton
            if cnt==1
            {



                    let mut button;
                    button = Button::new(Text::new(""))

                        .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(2,&app.tema)))
                        .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                        .height(Length::from(50));
                    row = row.push(button);



            }
            else if cnt==0 {


            let mut button;
            button = Button::new(Text::new(""))
                .on_press(Message::ButtonPressed(index + 1))
                .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                .height(Length::from(50));
                row = row.push(button);
           }
            else {
                let image_handle =image::Handle::from_path("files/mouse.png");
                let imagee = Container::new(
                    image::Viewer::new(image_handle)
                )
                    .width(Length::from(50)) // Butonul mai mic pe rândurile impare
                    .height(Length::from(50))
                    .style(|_| container_style(&Theme::Light, Status::Active,&app.tema, culori(3,&app.tema)))
                    ;
                row=row.push(imagee);
            }

        }

        // Adăugăm rândul în matrice
        column = column.push(row);
    }

    // Container pentru butoane
    let buttons = Container::new(column)
        .align_x(Center) // Aliniere pe centru pe axa orizontală
        .align_y(Center) // Aliniere pe centru pe axa verticală
        .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
        .height(Length::Fill); // Lărgire pe întreaga înălțime disponibilă

    // Adăugăm elementele într-un Stack
    let mut stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))  // Fundalul dreptunghiular
        .push(Container::new(buttons))
    .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
        .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
        .on_press(Message::ButtonPressed(2222))
        .width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
        .height(Length::from(50)));
    ; // Butoane// Butoanele

    // Returnăm Stack-ul final
    //stacked_elements.into()
   // }

    if game_over==true {
       /* let dim = app.dimensions;

        // Definirea dreptunghiului (pentru fundal)
        let rectangle = ShapeCollection::new(ShapeType::Rectangle {
            x: 0.0,
            y: 0.0,
            width: dim.0 as f32,
            height: dim.1 as f32,
            culoare: forme::culori(3, &app.tema), // Folosim culoarea specifică pentru fundal
        });

        // Canvas pentru dreptunghi
        let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

        // Butoane
        let mut column = Column::new().spacing(0); // Fără spațiu între butoane

        for i in 0..11 {
            let mut row = Row::new().spacing(0); // Fără spațiu între butoane

            for j in 0..11 {
                let index = i * 11 + j;

                // Creăm butonul cu indexul corect

                //conditie daca pozitia este fierita de o, nu putem face buton(avem obstacol sau e soarecele acolo)
                /*  if index%10==1
                  {
                      if index !=41
                      {
                          let mut button;
                          button = Button::new(Text::new(""))
                              .on_press(Message::ButtonPressed(index + 1))
                              .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(3,&app.tema)))
                              .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                              .height(Length::from(50));
                          row = row.push(button);

                      }
                      else {

                      let image_handle =image::Handle::from_path("files/mouse.png");
                      let imagee = Container::new(
                          image::Viewer::new(image_handle)
                      )
                          .width(Length::from(50)) // Butonul mai mic pe rândurile impare
                          .height(Length::from(50))
                          .style(|_| container_style(&Theme::Light, Status::Active,&app.tema, culori(3,&app.tema)))
                          ;
      row=row.push(imagee);}

                  }
                  else {*/

                // Adăugăm jumatate de buton de culoarea fundalului in fata randurilor impare pentru a intercala randurile
                if (index % 11 + 1) == 1 && index % 2 == 1 {
                    let mut button1 = Button::new(Text::new(""))
                        .style(|_, status| button_style(&Theme::Dark, status,&app.tema, culori(3,&app.tema)))
                        .width(Length::from(25)) // Butonul mai mic pe rândurile impare
                        .height(Length::from(50));
                    row = row.push(button1);
                }
                //cream buton
                let mut button;
                button = Button::new(Text::new(""))

                    .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                    .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                    .height(Length::from(50));
                row = row.push(button);
            }

            // }

            // Adăugăm rândul în matrice
            column = column.push(row);
        }

        // Container pentru butoane
        let buttons = Container::new(column)
            .align_x(Center) // Aliniere pe centru pe axa orizontală
            .align_y(Center) // Aliniere pe centru pe axa verticală
            .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
            .height(Length::Fill); // Lărgire pe întreaga înălțime disponibilă

*/
        let rectangle1 = ShapeCollection::new(ShapeType::Rectangle {
            x: (dim.0/2) as f32-150.0,
            y: (dim.1/2) as f32-150.0,
            width: 300.0,
            height: 300.0,
            culoare: culori(3,&app.tema),
        });
        let rectangle2 = ShapeCollection::new(ShapeType::Rectangle {
            x: (dim.0/2) as f32-160.0,
            y: (dim.1/2) as f32-160.0,
            width: 320.0,
            height: 320.0,
            culoare: culori(0,&app.tema),
        });

        // Canvas pentru dreptunghi
        let rec_canvas1 = Canvas::new(rectangle1).width(Length::Fill).height(Length::Fill);
        let rec_canvas2 = Canvas::new(rectangle2).width(Length::Fill).height(Length::Fill);
        let  mut text=Text::new(" GAME OVER\nHUNTER WIN").color(culori(0,&app.tema)).size(45).align_x(Center);

        if winner=="HUNTER"{
             text=Text::new(" GAME OVER\nHUNTER WIN").color(culori(0,&app.tema)).size(45).align_x(Center);
        }
        else {
             text=Text::new(" GAME OVER\nMOUSE WIN").color(culori(0,&app.tema)).size(45).align_x(Center);
        }


        // Adăugăm elementele într-un Stack
      stacked_elements=stacked_elements
            .push(Container::new(rec_canvas2))
            .push(Container::new(rec_canvas1))
            .push(Container::new((Column::new().push(text).align_x(Alignment::Center)
                .push(Button::new(Text::new("Return Menu").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center) ).on_press(Message::NavigateToMenu)
                    .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
                .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                .height(Length::from(75)))))
                .align_x(Center) // Aliniere pe centru pe axa orizontală
                .align_y(Center) // Aliniere pe centru pe axa verticală
                .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
                .height(Length::Fill));

         // Butoane// Butoanele

        // Returnăm Stack-ul final

    }
    stacked_elements.into()
}




fn view_two_players(app: & MyApp) -> Element< Message> {
    let dim = app.dimensions;
    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,&app.tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,&app.tema),
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
        culoare: forme::culori(3,&app.tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Join").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToJoin)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        // Setam o inaltime fixa pentru toate butoanele
        .push(Button::new(Text::new("Create").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToCreate)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));

    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Two Players").size((app.dimensions.0/25) as f32)).align_x(Alignment::Center)
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
        .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToMenu)
            .width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50)));
    ; // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()
}

fn view_join(app: & MyApp) -> Element<Message> {

    let dim = app.dimensions;
    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,&app.tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,&app.tema),
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
        culoare: forme::culori(3,&app.tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;


    let pin_input = text_input(
        "ENTER PIN",
        &app.model.pin.clone().unwrap_or_default(),

    ).on_input(Message::SetPin)

        .width(Length::Fill).size(25).align_x(Alignment::Center).style(|_, status|  text_input_style(&Theme::Light, status,&app.tema, culori(3,&app.tema)));

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Join Game").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToGameBoard)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
       .push(pin_input.width(Length::from(250)))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila


        ;
        // Setam o inaltime fixa pentru toate butoanele


    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Two Players").size((app.dimensions.0/25) as f32)).align_x(Alignment::Center)
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
        .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers).width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50))); // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()

}

fn view_create(app: &MyApp) -> Element< Message> {

    let dim = app.dimensions;
    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,&app.tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,&app.tema),
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
        culoare: forme::culori(3,&app.tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;


    let pin_input = text_input(
        "CREATE PIN",
        &app.model.pin.clone().unwrap_or_default(),

    ).on_input(Message::SetPin)

        .width(Length::Fill).size(25).align_x(Alignment::Center).style(|_, status|  text_input_style(&Theme::Light, status,&app.tema, culori(3,&app.tema)));

    // Butoane
    let mut buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Create Game").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToGameBoard)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));
    let role = app.model.role.clone();

        if role.unwrap() == Role::Hunter {
            let row= Row::new().spacing(10)
        .push(Button::new(Text::new("MODE: Mouse").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center))
        .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
        .on_press(Message::SetRole(Role::Mouse))
        .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
        .height(Length::from(75)))
            .push(Button::new(Text::new("MODE: Hunter").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center))
                .style(|_, status| button_style(&Theme::Dracula, status,&app.tema,culori(2,&app.tema)))
                .on_press(Message::SetRole(Role::Hunter))
                .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                .height(Length::from(75)))  ;
            buttons=buttons.push(row).push(pin_input.width(Length::from(250))).align_x(Center);
    }
        else {
            let row= Row::new().spacing(10)
                .push(Button::new(Text::new("MODE: Mouse").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center))
                    .style(|_, status| button_style(&Theme::Dracula, status,&app.tema,culori(2,&app.tema)))
                    .on_press(Message::SetRole(Role::Mouse))
                    .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                    .height(Length::from(75)))
             .push(Button::new(Text::new("MODE: Hunter").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center))
                 .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
                 .on_press(Message::SetRole(Role::Hunter))
                 .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                 .height(Length::from(75)))
             ;
            buttons=buttons.push(row).push(pin_input.width(Length::from(250))).align_x(Center);
        }


       // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
    // Folosim Fill pentru a extinde butonul pe toata latimea disponibila


        ;
    // Setam o inaltime fixa pentru toate butoanele


    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Two Players").size((app.dimensions.0/25) as f32)).align_x(Alignment::Center)
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
        .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers).width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50))); // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()
}


pub fn view_game_board<'a>(app: &'a MyApp) -> Element<'a, Message>
{
    let mut poz :i32=0;
    let mut i=0;
    let mut j=0;
    while i<=10
    {
        j=0;
        while j<=10
        {
            if app.matrix[i][j]==-1
            {
                poz=(i*11+j)as i32;
                break;
            }
            j+=1;
        }
        i+=1;
    }

    let mut game_over=false;
    let mut winner=String::from("");
    if let Some(ceva)=&app.winner
    {
        game_over=true;
        winner=ceva.clone();
        println!("wienenoerijdfjikgfd: {}",winner);

    }

    // if game_over==false
    // {
    let dim = app.dimensions;

    // Definirea dreptunghiului (pentru fundal)
    let rectangle = ShapeCollection::new(ShapeType::Rectangle {
        x: 0.0,
        y: 0.0,
        width: dim.0 as f32,
        height: dim.1 as f32,
        culoare: forme::culori(3, &app.tema), // Folosim culoarea specifică pentru fundal
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

    // Butoane
    let mut column = Column::new().spacing(0); // Fără spațiu între butoane


    for i in 0..11 {
        let mut row = Row::new().spacing(0); // Fără spațiu între butoane

        for j in 0..11 {
            let index = i * 11 + j;

            let cnt=app.matrix[((index)/11)as usize][((index)%11) as usize];
            // Adăugăm jumatate de buton de culoarea fundalului in fata randurilor impare pentru a intercala randurile
            if (index % 11 + 1) == 1 && index % 2 == 1 {
                let mut button1 = Button::new(Text::new(""))
                    .style(|_, status| button_style(&Theme::Dark, status,&app.tema, culori(3,&app.tema)))
                    .width(Length::from(25)) // Butonul mai mic pe rândurile impare
                    .height(Length::from(50));
                row = row.push(button1);
            }
            //cream buton
            if cnt==1
            {


                let mut button;
                button = Button::new(Text::new(""))

                    .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(2,&app.tema)))
                    .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                    .height(Length::from(50));
                row = row.push(button);



            }
            else if cnt==0 {
                if  let Some(rol)=app.model.clone().role
                {
    let mut cntt=0;
                    if i%2==1
                    {  cntt=1;}
                    else
                    {
                        cntt=0;
                    }

                if rol == Role::Mouse
                {
                    if !((poz-1)as i32==index as i32||(poz+1)as i32==index as i32||(poz-10-cntt) as i32==index as i32||(poz-11-cntt)as i32==index as i32||(poz+11-cntt) as i32==index as i32||(poz+12-cntt) as i32==index as i32)
                    {
                    let mut button;
                    button = Button::new(Text::new(""))

                        .style(|_, status| button_style(&Theme::Light, status, &app.tema, culori(2,&app.tema)))
                        .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                        .height(Length::from(50));
                    row = row.push(button);}
                        else{
                            let mut button;
                            button = Button::new(Text::new(""))
                                .on_press(Message::ButtonPressed(index + 1))
                                .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                                .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                                .height(Length::from(50));
                            row = row.push(button);
                        }

                }
                    else {
                        let mut button;
                        button = Button::new(Text::new(""))
                            .on_press(Message::ButtonPressed(index + 1))
                            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                            .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                            .height(Length::from(50));
                        row = row.push(button);

                    }
                }
            }
            else {
                let image_handle =image::Handle::from_path("C:\\Users\\Costelino\\Desktop\\Proiect_rust\\Trap_the_mouse_client\\files/mouse.png");
                let imagee = Container::new(
                    image::Viewer::new(image_handle)
                )
                    .width(Length::from(50)) // Butonul mai mic pe rândurile impare
                    .height(Length::from(50))
                    .style(|_| container_style(&Theme::Light, Status::Active,&app.tema, culori(3,&app.tema)))
                    ;
                row=row.push(imagee);
            }

        }

        // Adăugăm rândul în matrice
        column = column.push(row);
    }

    // Container pentru butoane
    let buttons = Container::new(column)
        .align_x(Center) // Aliniere pe centru pe axa orizontală
        .align_y(Center) // Aliniere pe centru pe axa verticală
        .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
        .height(Length::Fill); // Lărgire pe întreaga înălțime disponibilă

    // Adăugăm elementele într-un Stack
    let mut stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))  // Fundalul dreptunghiular
        .push(Container::new(buttons))
        .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::ButtonPressed(2222))
            .width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50)));
    ; // Butoane// Butoanele

    // Returnăm Stack-ul final
    //stacked_elements.into()
    // }

    if game_over==true {
        /* let dim = app.dimensions;

         // Definirea dreptunghiului (pentru fundal)
         let rectangle = ShapeCollection::new(ShapeType::Rectangle {
             x: 0.0,
             y: 0.0,
             width: dim.0 as f32,
             height: dim.1 as f32,
             culoare: forme::culori(3, &app.tema), // Folosim culoarea specifică pentru fundal
         });

         // Canvas pentru dreptunghi
         let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

         // Butoane
         let mut column = Column::new().spacing(0); // Fără spațiu între butoane

         for i in 0..11 {
             let mut row = Row::new().spacing(0); // Fără spațiu între butoane

             for j in 0..11 {
                 let index = i * 11 + j;

                 // Creăm butonul cu indexul corect

                 //conditie daca pozitia este fierita de o, nu putem face buton(avem obstacol sau e soarecele acolo)
                 /*  if index%10==1
                   {
                       if index !=41
                       {
                           let mut button;
                           button = Button::new(Text::new(""))
                               .on_press(Message::ButtonPressed(index + 1))
                               .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(3,&app.tema)))
                               .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                               .height(Length::from(50));
                           row = row.push(button);

                       }
                       else {

                       let image_handle =image::Handle::from_path("files/mouse.png");
                       let imagee = Container::new(
                           image::Viewer::new(image_handle)
                       )
                           .width(Length::from(50)) // Butonul mai mic pe rândurile impare
                           .height(Length::from(50))
                           .style(|_| container_style(&Theme::Light, Status::Active,&app.tema, culori(3,&app.tema)))
                           ;
       row=row.push(imagee);}

                   }
                   else {*/

                 // Adăugăm jumatate de buton de culoarea fundalului in fata randurilor impare pentru a intercala randurile
                 if (index % 11 + 1) == 1 && index % 2 == 1 {
                     let mut button1 = Button::new(Text::new(""))
                         .style(|_, status| button_style(&Theme::Dark, status,&app.tema, culori(3,&app.tema)))
                         .width(Length::from(25)) // Butonul mai mic pe rândurile impare
                         .height(Length::from(50));
                     row = row.push(button1);
                 }
                 //cream buton
                 let mut button;
                 button = Button::new(Text::new(""))

                     .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                     .width(Length::from(50))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                     .height(Length::from(50));
                 row = row.push(button);
             }

             // }

             // Adăugăm rândul în matrice
             column = column.push(row);
         }

         // Container pentru butoane
         let buttons = Container::new(column)
             .align_x(Center) // Aliniere pe centru pe axa orizontală
             .align_y(Center) // Aliniere pe centru pe axa verticală
             .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
             .height(Length::Fill); // Lărgire pe întreaga înălțime disponibilă

 */
        let rectangle1 = ShapeCollection::new(ShapeType::Rectangle {
            x: (dim.0/2) as f32-150.0,
            y: (dim.1/2) as f32-150.0,
            width: 300.0,
            height: 300.0,
            culoare: culori(3,&app.tema),
        });
        let rectangle2 = ShapeCollection::new(ShapeType::Rectangle {
            x: (dim.0/2) as f32-160.0,
            y: (dim.1/2) as f32-160.0,
            width: 320.0,
            height: 320.0,
            culoare: culori(0,&app.tema),
        });

        // Canvas pentru dreptunghi
        let rec_canvas1 = Canvas::new(rectangle1).width(Length::Fill).height(Length::Fill);
        let rec_canvas2 = Canvas::new(rectangle2).width(Length::Fill).height(Length::Fill);
        let  mut text=Text::new(" GAME OVER\nHUNTER WIN").color(culori(0,&app.tema)).size(45).align_x(Center);

        if winner=="HUNTER"{
            text=Text::new(" GAME OVER\nHUNTER WIN").color(culori(0,&app.tema)).size(45).align_x(Center);
        }
        else if winner=="MOUSE"{
            text=Text::new(" GAME OVER\nMOUSE WIN").color(culori(0,&app.tema)).size(45).align_x(Center);
        }
        else {
            text=Text::new(" PLAYER\nOFFLINE").color(culori(0,&app.tema)).size(45).align_x(Center);
        }


        // Adăugăm elementele într-un Stack
        stacked_elements=stacked_elements
            .push(Container::new(rec_canvas2))
            .push(Container::new(rec_canvas1))
            .push(Container::new((Column::new().push(text).align_x(Alignment::Center)
                .push(Button::new(Text::new("Return Menu").size(25.0).align_x(Alignment::Center).align_y(Alignment::Center) ).on_press(Message::NavigateToMenu)
                    .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
                    .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
                    .height(Length::from(75)))))
                .align_x(Center) // Aliniere pe centru pe axa orizontală
                .align_y(Center) // Aliniere pe centru pe axa verticală
                .width(Length::Fill)  // Lărgire pe întreaga lățime disponibilă
                .height(Length::Fill));

        // Butoane// Butoanele

        // Returnăm Stack-ul final

    }
    stacked_elements.into()
}

pub fn view_option<'a>(app: &'a MyApp) -> Element<'a, Message> {
    let dim = app.dimensions;


    // Definirea trapezului
    let shape_trapez_1 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0/15) as f32,
        culoare: forme::culori(1,&app.tema),
    });
    let shape_trapez_2 = ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,  // Dimensiunea pe latime
        y: 0.0, // Dimensiunea pe inaltime
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0/15) as f32+5.0,
        culoare: forme::culori(0,&app.tema),
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
        culoare: forme::culori(3,&app.tema),
    });

    // Canvas pentru dreptunghi
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);
    let theme = Theme::Light;
    let status = button::Status::Active;

    let style = forme::button_style(&theme, status,&app.tema,Color::BLACK);

    // Butoane
    let buttons = Column::new()
        .spacing(10)
        // Aliniere pe orizontala la centru
        .push(Button::new(Text::new("Emerald").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)

            .align_y(Alignment::Center))
            .style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::EMERALD)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        // Setam o inaltime fixa pentru toate butoanele
        .push(Button::new(Text::new("Candy").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::CANDY)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Aqua").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::AQUA)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Default").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::DEFAULT)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
        .push(Button::new(Text::new("Coffee").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::COFFEE)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)))
            .push(Button::new(Text::new("Princess").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::PRINCESS)
            .width(Length::from(250))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(75)));

    // Suprapunerea cu Stack
    let stacked_elements = Stack::new()
        .push(Container::new(rec_canvas))
        .push(Container::new(trapez_2))// Fundal dreptunghiular
        .push(Container::new(trapez_1)) // Fundal trapezoidal
        .push(Container::new((Text::new("Select Theme").size((app.dimensions.0/25) as f32)).align_x(Alignment::Center)
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
        .push(Button::new(Text::new("Back").size(25.0).align_x(Alignment::Center)  // Aliniaza pe axa X (orizontal)
            .align_y(Alignment::Center)).style(|_, status| button_style(&Theme::Light, status,&app.tema,Color::BLACK))
            .on_press(Message::NavigateToMenu)
            .width(Length::from(100))  // Folosim Fill pentru a extinde butonul pe toata latimea disponibila
            .height(Length::from(50)));
        ; // Butoane

    // Returneaza Stack-ul final
    stacked_elements.into()
}