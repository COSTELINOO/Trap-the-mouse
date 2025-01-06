use iced::widget::button::Status;
use iced::{widget::canvas::{Canvas}, Color, Length, Theme, Alignment::Start, Center};
use iced::widget::{Column, Button, Stack, Container, Row, text_input};
use iced::{Element};
use iced::widget::Text;
use crate::{MyApp};
use crate::model::Page;
use crate::model::Role;
use crate::messages::Message;
use crate::forme::{button_style, container_style, culori, text_input_style, ShapeCollection, ShapeType};
use iced::widget::image;


pub fn view(app: &MyApp) -> Element<Message> {
    match app.model.current_page {
        Page::Menu => view_menu(app),
        Page::SinglePlayer => view_single_player(app),
        Page::TwoPlayers => view_two_players(app),
        Page::Join => view_join(app),
        Page::Create => view_create(app),
        Page::GameBoard => view_game_board(app),
        Page::Option => view_option(app),
    }
}
fn trapez_1(app: &MyApp) -> ShapeCollection
{
    let dim = app.dimensions;
    ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,
        y: 0.0,
        top_width: (dim.0 as f32) - 400.0,
        bottom_width: (dim.0 as f32) - 500.0,
        height: (dim.0 / 15) as f32,
        culoare: culori(1, &app.tema),
    })
}

fn trapez_2(app: &MyApp) -> ShapeCollection
{
    let dim = app.dimensions;

    ShapeCollection::new(ShapeType::Trapezoid {
        x: (dim.0 as f32) / 2.0,
        y: 0.0,
        top_width: (dim.0 as f32) - 390.0,
        bottom_width: (dim.0 as f32) - 490.0,
        height: (dim.0 / 15) as f32 + 5.0,
        culoare: culori(0, &app.tema),
    })
}

fn dreptunghi(app: &MyApp) -> ShapeCollection
{
    let dim = app.dimensions;
    ShapeCollection::new(ShapeType::Rectangle {
        x: 0.0,
        y: 0.0,
        width: dim.0 as f32,
        height: dim.1 as f32,
        culoare: culori(3, &app.tema),
    })
}

fn drept_1_game_over(app: &MyApp) -> ShapeCollection
{
    let dim = app.dimensions;
    ShapeCollection::new(ShapeType::Rectangle {
        x: (dim.0 / 2) as f32 - 150.0,
        y: (dim.1 / 2) as f32 - 150.0,
        width: 300.0,
        height: 300.0,
        culoare: culori(3, &app.tema),
    })
}

fn drept_2_game_over(app: &MyApp) -> ShapeCollection
{
    let dim = app.dimensions;
    ShapeCollection::new(ShapeType::Rectangle {
        x: (dim.0 / 2) as f32 - 160.0,
        y: (dim.1 / 2) as f32 - 160.0,
        width: 320.0,
        height: 320.0,
        culoare: culori(0, &app.tema),
    })
}

fn view_menu(app: &MyApp) -> Element<Message>
{
    let shape_trapez_1 = trapez_1(app);
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let shape_trapez_2 = trapez_2(app);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);


    let buttons = Column::new()
        .spacing(10)

        .push(Button::new(Text::new("Single Player").align_x(Center).size(25.0)
            .align_y(Center))
            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToSinglePlayer)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Two Players").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Options").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToOption)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Exit").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::Exit)
            .width(Length::from(250))
            .height(Length::from(75)));


    let stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(trapez_2))

        .push(Container::new(trapez_1))

        .push(Container::new(Text::new("Trap The Mouse")
            .size((app.dimensions.0 / 25) as f32)
            .align_x(Center)
            .align_y(Start)
            .width(Length::Fill)).height(Length::Fill)
        )

        .push(Container::new(buttons)
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
        );


    stacked_elements.into()
}

fn view_single_player(app: &MyApp) -> Element<Message>
{
    let mut game_over = false;

    let mut winner = String::from("");

    if let Some(ceva) = &app.winner
    {
        game_over = true;
        winner = ceva.clone();
    }

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);


    let mut column = Column::new().spacing(0);

    for i in 0..11 {
        let mut row = Row::new().spacing(0);

        for j in 0..11 {
            let index = i * 11 + j;

            let cnt = app.matrix[((index) / 11) as usize][((index) % 11) as usize];

            if (index % 11 + 1) == 1 && index % 2 == 1 {
                let button1 =
                    Button::new(Text::new(""))
                        .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(3, &app.tema)))
                        .width(Length::from(25))
                        .height(Length::from(50));

                row = row.push(button1);
            }

            if cnt == 1
            {
                let button = Button::new(Text::new(""))
                    .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(2, &app.tema)))
                    .width(Length::from(50))
                    .height(Length::from(50));

                row = row.push(button);

            } else if cnt == 0 {
                let button = Button::new(Text::new(""))
                    .on_press(Message::ButtonPressed(index + 1))
                    .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                    .width(Length::from(50))
                    .height(Length::from(50));

                row = row.push(button);

            } else {

                let image_handle = image::Handle::from_path(app.get_locatie());

                let imagee = Container::new(image::Viewer::new(image_handle))
                    .width(Length::from(50))
                    .height(Length::from(50))
                    .style(|_| container_style(&Theme::Light, Status::Active, &app.tema, culori(3, &app.tema)));

                row = row.push(imagee);
            }
        }


        column = column.push(row);
    }

    let buttons = Container::new(column)
        .align_x(Center)
        .align_y(Center)
        .width(Length::Fill)
        .height(Length::Fill);


    let mut stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(buttons))

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::ButtonPressed(2222))
            .width(Length::from(100))
            .height(Length::from(50)));

    if game_over == true {

        let rectangle1 = drept_1_game_over(app);
        let rec_canvas1 = Canvas::new(rectangle1).width(Length::Fill).height(Length::Fill);

        let rectangle2 = drept_2_game_over(app);
        let rec_canvas2 = Canvas::new(rectangle2).width(Length::Fill).height(Length::Fill);

        let  text;

        if winner == "HUNTER" {
            text = Text::new(" GAME OVER\nHUNTER WIN").color(culori(0, &app.tema)).size(45).align_x(Center);
        } else {
            text = Text::new(" GAME OVER\nMOUSE WIN").color(culori(0, &app.tema)).size(45).align_x(Center);
        }

        stacked_elements = stacked_elements

            .push(Container::new(rec_canvas2))

            .push(Container::new(rec_canvas1))

            .push(Container::new(Column::new().push(text).align_x(Center)
                .push(Button::new(Text::new("Return Menu").size(25.0).align_x(Center).align_y(Center)).on_press(Message::NavigateToMenu)
                    .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                    .width(Length::from(250))
                    .height(Length::from(75))))
                .align_x(Center)
                .align_y(Center)
                .width(Length::Fill)
                .height(Length::Fill));
    }

    stacked_elements.into()
}

fn view_two_players(app: &MyApp) -> Element<Message> {
    let shape_trapez_1 = trapez_1(app);
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let shape_trapez_2 = trapez_2(app);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

    let buttons = Column::new()
        .spacing(10)

        .push(Button::new(Text::new("Join").size(25.0).align_x(Center)
            .align_y(Center))
            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToJoin)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Create").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToCreate)
            .width(Length::from(250))
            .height(Length::from(75)));


    let stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(trapez_2))

        .push(Container::new(trapez_1))

        .push(Container::new(Text::new("Two Players").size((app.dimensions.0 / 25) as f32).align_x(Center)
            .align_y(Start)
            .width(Length::Fill))
                  .height(Length::Fill)
        )

        .push(Container::new(buttons)
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
        )

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToMenu)
            .width(Length::from(100))
            .height(Length::from(50)));

    stacked_elements.into()
}

fn view_join(app: &MyApp) -> Element<Message> {

    let shape_trapez_1 = trapez_1(app);
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let shape_trapez_2 = trapez_2(app);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

    let pin_input = text_input(
        "ENTER PIN",
        &app.model.pin.clone().unwrap_or_default(),
    ).on_input(Message::SetPin)
        .width(Length::Fill).size(25).align_x(Center).style(|_, status| text_input_style(&Theme::Light, status, &app.tema, culori(3, &app.tema)));

    let buttons = Column::new()
        .spacing(10)

        .push(Button::new(Text::new("Join Game").size(25.0).align_x(Center)
            .align_y(Center))
            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToGameBoard)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(pin_input.width(Length::from(250)));


    let stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(trapez_2))

        .push(Container::new(trapez_1))

        .push(Container::new(Text::new("Two Players").size((app.dimensions.0 / 25) as f32).align_x(Center)
            .align_y(Start)
            .width(Length::Fill))
                  .height(Length::Fill)
        )

        .push(Container::new(buttons)
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
        )

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers).width(Length::from(100))
            .height(Length::from(50)));

    stacked_elements.into()
}

fn view_create(app: &MyApp) -> Element<Message> {

    let shape_trapez_1 = trapez_1(app);
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let shape_trapez_2 = trapez_2(app);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);

    let pin_input = text_input(
        "CREATE PIN",
        &app.model.pin.clone().unwrap_or_default(),
    ).on_input(Message::SetPin)
        .width(Length::Fill).size(25).align_x(Center).style(|_, status| text_input_style(&Theme::Light, status, &app.tema, culori(3, &app.tema)));


    let mut buttons = Column::new()
        .spacing(10)

        .push(Button::new(Text::new("Create Game").size(25.0).align_x(Center)
            .align_y(Center))
            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToGameBoard)
            .width(Length::from(250))
            .height(Length::from(75)));

    let role = app.model.role.clone();

    if role.unwrap() == Role::Hunter {

        let row = Row::new().spacing(10)

            .push(Button::new(Text::new("MODE: Mouse").size(25.0).align_x(Center).align_y(Center))
                .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                .on_press(Message::SetRole(Role::Mouse))
                .width(Length::from(250))
                .height(Length::from(75)))

            .push(Button::new(Text::new("MODE: Hunter").size(25.0).align_x(Center).align_y(Center))
                .style(|_, status| button_style(&Theme::Dracula, status, &app.tema, culori(2, &app.tema)))
                .on_press(Message::SetRole(Role::Hunter))
                .width(Length::from(250))
                .height(Length::from(75)));

        buttons = buttons
            .push(row).

            push(pin_input.width(Length::from(250))).align_x(Center);

    } else {

        let row = Row::new().spacing(10)

            .push(Button::new(Text::new("MODE: Mouse").size(25.0).align_x(Center).align_y(Center))
                .style(|_, status| button_style(&Theme::Dracula, status, &app.tema, culori(2, &app.tema)))
                .on_press(Message::SetRole(Role::Mouse))
                .width(Length::from(250))
                .height(Length::from(75)))

            .push(Button::new(Text::new("MODE: Hunter").size(25.0).align_x(Center).align_y(Center))
                .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                .on_press(Message::SetRole(Role::Hunter))
                .width(Length::from(250))
                .height(Length::from(75)))
            ;
        buttons = buttons.push(row).push(pin_input.width(Length::from(250))).align_x(Center);
    }


    let stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(trapez_2))

        .push(Container::new(trapez_1))

        .push(Container::new(Text::new("Two Players").size((app.dimensions.0 / 25) as f32).align_x(Center)
            .align_y(Start)
            .width(Length::Fill))
                  .height(Length::Fill)
        )

        .push(Container::new(buttons)
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
        )

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToTwoPlayers).width(Length::from(100))
            .height(Length::from(50)));

    stacked_elements.into()
}

fn view_game_board(app: &MyApp) -> Element<Message> {

    let mut poz: i32 = 0;
    let mut i = 0;
    let mut j ;

    while i <= 10
    {
        j = 0;

        while j <= 10
        {
            if app.matrix[i][j] == -1
            {
                poz = (i * 11 + j) as i32;
                break;
            }

            j += 1;
        }

        i += 1;
    }

    let mut game_over = false;
    let mut winner = String::from("");

    if let Some(ceva) = &app.winner
    {
        game_over = true;
        winner = ceva.clone();
    }

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);


    let mut column = Column::new().spacing(0);

    for i in 0..11 {

        let mut row = Row::new().spacing(0);

        for j in 0..11 {

            let index = i * 11 + j;

            let cnt = app.matrix[((index) / 11) as usize][((index) % 11) as usize];


            if (index % 11 + 1) == 1 && index % 2 == 1 {

                let button1 = Button::new(Text::new(""))
                    .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(3, &app.tema)))
                    .width(Length::from(25))
                    .height(Length::from(50));

                row = row.push(button1);
            }

            if cnt == 1
            {
                let button = Button::new(Text::new(""))
                    .style(|_, status| button_style(&Theme::Dark, status, &app.tema, culori(2, &app.tema)))
                    .width(Length::from(50))
                    .height(Length::from(50));

                row = row.push(button);

            } else if cnt == 0 {

                if let Some(rol) = app.model.clone().role
                {
                    let  cntt ;

                    if i % 2 == 1
                    { cntt = 1; } else {
                        cntt = 0;
                    }

                    if rol == Role::Mouse
                    {
                        if !((poz - 1) == index as i32 || (poz + 1) == index as i32 || (poz - 10 - cntt) == index as i32 || (poz - 11 - cntt) == index as i32 || (poz + 11 - cntt) == index as i32 || (poz + 12 - cntt) == index as i32)
                        {
                            let button = Button::new(Text::new(""))
                                .style(|_, status| button_style(&Theme::Light, status, &app.tema, culori(2, &app.tema)))
                                .width(Length::from(50))
                                .height(Length::from(50));

                            row = row.push(button);

                        } else {

                            let button= Button::new(Text::new(""))
                                .on_press(Message::ButtonPressed(index + 1))
                                .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                                .width(Length::from(50))
                                .height(Length::from(50));

                            row = row.push(button);
                        }

                    } else {

                        let button = Button::new(Text::new(""))
                            .on_press(Message::ButtonPressed(index + 1))
                            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                            .width(Length::from(50))
                            .height(Length::from(50));

                        row = row.push(button);
                    }
                }

            } else {

                let image_handle = image::Handle::from_path(app.get_locatie());

                let imagee = Container::new(image::Viewer::new(image_handle))
                    .width(Length::from(50))
                    .height(Length::from(50))
                    .style(|_| container_style(&Theme::Light, Status::Active, &app.tema, culori(3, &app.tema)));

                row = row.push(imagee);
            }
        }

        column = column.push(row);
    }


    let buttons = Container::new(column)
        .align_x(Center)
        .align_y(Center)
        .width(Length::Fill)
        .height(Length::Fill);


    let mut stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(buttons))

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::ButtonPressed(2222))
            .width(Length::from(100))
            .height(Length::from(50)));


    if game_over == true {

        let rectangle1 =drept_1_game_over(app);
        let rec_canvas1 = Canvas::new(rectangle1).width(Length::Fill).height(Length::Fill);

        let rectangle2 = drept_2_game_over(app);
        let rec_canvas2 = Canvas::new(rectangle2).width(Length::Fill).height(Length::Fill);


        let  text;

        if winner == "HUNTER" {
            text = Text::new(" GAME OVER\nHUNTER WIN").color(culori(0, &app.tema)).size(45).align_x(Center);
        } else if winner == "MOUSE" {
            text = Text::new(" GAME OVER\nMOUSE WIN").color(culori(0, &app.tema)).size(45).align_x(Center);
        } else {
            text = Text::new(" PLAYER\nOFFLINE").color(culori(0, &app.tema)).size(45).align_x(Center);
        }

        stacked_elements = stacked_elements

            .push(Container::new(rec_canvas2))

            .push(Container::new(rec_canvas1))

            .push(Container::new(Column::new().push(text).align_x(Center)
                .push(Button::new(Text::new("Return Menu").size(25.0).align_x(Center).align_y(Center)).on_press(Message::NavigateToMenu)
                    .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
                    .width(Length::from(250))
                    .height(Length::from(75))))
                .align_x(Center)
                .align_y(Center)
                .width(Length::Fill)
                .height(Length::Fill));

    }
    stacked_elements.into()
}

fn view_option(app: &MyApp) -> Element<Message> {

    let shape_trapez_1 = trapez_1(app);
    let trapez_1 = Canvas::new(shape_trapez_1)
        .width(Length::Fill)
        .height(Length::Fill);

    let shape_trapez_2 = trapez_2(app);
    let trapez_2 = Canvas::new(shape_trapez_2)
        .width(Length::Fill)
        .height(Length::Fill);

    let rectangle = dreptunghi(app);
    let rec_canvas = Canvas::new(rectangle).width(Length::Fill).height(Length::Fill);


    let theme = Theme::Light;
    let status = Status::Active;
    button_style(&theme, status, &app.tema, Color::BLACK);


    let buttons = Column::new()
        .spacing(10)

        .push(Button::new(Text::new("Emerald").size(25.0).align_x(Center)
            .align_y(Center))
            .style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::EMERALD)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Candy").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::CANDY)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Aqua").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::AQUA)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Default").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::DEFAULT)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Coffee").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::COFFEE)
            .width(Length::from(250))
            .height(Length::from(75)))

        .push(Button::new(Text::new("Princess").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::PRINCESS)
            .width(Length::from(250))
            .height(Length::from(75)));


    let stacked_elements = Stack::new()

        .push(Container::new(rec_canvas))

        .push(Container::new(trapez_2))

        .push(Container::new(trapez_1))

        .push(Container::new(Text::new("Select Theme").size((app.dimensions.0 / 25) as f32).align_x(Center)
            .align_y(Start)
            .width(Length::Fill))
                  .height(Length::Fill)
        )

        .push(Container::new(buttons)
            .align_x(Center)
            .align_y(Center)
            .width(Length::Fill)
            .height(Length::Fill)
        )

        .push(Button::new(Text::new("Back").size(25.0).align_x(Center)
            .align_y(Center)).style(|_, status| button_style(&Theme::Light, status, &app.tema, Color::BLACK))
            .on_press(Message::NavigateToMenu)
            .width(Length::from(100))
            .height(Length::from(50)));

    stacked_elements.into()
}