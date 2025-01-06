use crate::to_server;
use crate::parser::*;
use std::sync::Arc;
use iced::Task;
use crate::messages::{Message};

use crate::model::{Page, Role};

use crate::forme::{set_tema};
use crate::{do_async_work, MyApp};
use tokio::sync::Mutex;
use tokio::sync::mpsc as tokio_mpsc;

pub fn update(message: Message, app: &mut MyApp) -> Task<Message> {
    match message {
        Message::EMERALD => set_tema(1, &mut app.tema),

        Message::CANDY => set_tema(2, &mut app.tema),

        Message::AQUA => set_tema(3, &mut app.tema),

        Message::DEFAULT => set_tema(4, &mut app.tema),

        Message::COFFEE => set_tema(5, &mut app.tema),

        Message::PRINCESS => set_tema(6, &mut app.tema),

        Message::NavigateToMenu => {
            app.default();

            app.model.go_to_menu();
        }

        Message::NavigateToSinglePlayer => {
            let (tx, rx) = tokio_mpsc::channel(32);
            let (server_tx, server_rx) = tokio_mpsc::channel(32);

            tokio::spawn(async move {
                if let Err(_e) = to_server::tcp_handler(rx, server_tx).await {

                }
            });

            let receiver = Arc::new(Mutex::new(server_rx));

            let receiver_for_task = Arc::clone(&receiver);

            app.sender = Some(tx);
            app.receiver = Some(receiver);

            if let Some(sender) = &app.sender {
                let _ = sender.try_send("computer".to_string());
            }

            app.default();


            app.model.go_to_single_player();

            let command = Task::perform(async move {
                let res = do_async_work(receiver_for_task).await;
                Message::Received(res)
            }, |msg| msg);

            return command;
        }

        Message::Received(mesaj) => {
            parse_message(app, mesaj);

            if app.ready_room == true
            {
                app.model.go_to_game_board();
            }

            if let Some(receiver) = &app.receiver {
                let receiver_for_task = Arc::clone(receiver);
                let command = Task::perform(
                    async move {
                        let res = do_async_work(receiver_for_task).await;
                        Message::Received(res)
                    },
                    |msg| msg,
                );
                return command;
            }
        }

        Message::NavigateToTwoPlayers => app.model.go_to_two_players(),

        Message::NavigateToJoin => app.model.go_to_join(),

        Message::NavigateToCreate => app.model.go_to_create(),

        Message::NavigateToGameBoard => {
            let (tx, rx) = tokio_mpsc::channel(32);
            let (server_tx, server_rx) = tokio_mpsc::channel(32);

            tokio::spawn(async move {
                if let Err(e) = to_server::tcp_handler(rx, server_tx).await {
                    eprintln!("Eroare în thread-ul TCP: {}", e);
                }
            });

            let receiver = Arc::new(Mutex::new(server_rx));
            let receiver_for_task = Arc::clone(&receiver);

            app.sender = Some(tx);
            app.receiver = Some(receiver);

            if app.model.current_page == Page::Create
            {
                let mut aux = "create ".to_string();
                if let Some(pin) = &app.model.pin {
                    aux.push_str(&pin);
                }
                if let Some(role) = &app.model.role {
                    match role {
                        Role::Hunter => aux.push_str(" HUNTER"),
                        Role::Mouse => aux.push_str(" MOUSE"),
                    }
                }

                if let Some(sender) = &app.sender {
                    let _ = sender.try_send(aux);
                }
            } else if app.model.current_page == Page::Join
            {
                let mut aux = "join ".to_string();

                if let Some(pin) = &app.model.pin {
                    aux.push_str(&pin);
                }

                if let Some(sender) = &app.sender {
                    let _ = sender.try_send(aux);
                }
            }

            if app.ready_room == true
            {
                app.model.go_to_game_board();
            }

            let command = Task::perform(async move {
                let res = do_async_work(receiver_for_task).await;
                Message::Received(res)
            }, |msg| msg);
            return command;
        }

        Message::NavigateToOption => app.model.go_to_option(),

        Message::Exit => {
            if let Some(sender) = &app.sender {
                let _ = sender.try_send("EXITT".to_string());
            }

            std::process::exit(0);
        }

        Message::SetRole(role) => app.model.set_role(role),

        Message::SetPin(pin) => app.model.set_pin(pin),

        Message::Resize(size) => {
            *&mut app.dimensions = (size.width as u32, size.height as u32);
        }

        Message::ButtonPressed(numar) => {
            if let Some(role) = &app.model.role {
                if let Some(ceva) = &app.model.current_move
                {
                    if numar == 2222
                    {
                        if let Some(sender) = &app.sender {
                            let _ = sender.try_send("iesire".to_string());
                        }

                        app.default();

                        app.model.go_to_menu();

                        app.winner = None;
                    } else if role == ceva && app.matrix[((numar - 1) / 11) as usize][((numar - 1) % 11) as usize] == 0 {
                        if let Some(sender) = &app.sender {
                            let _ = sender.try_send(numar.to_string());
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Task::none()
}

