#![warn(unused_variables)]
mod drum;
use rand::{Rng, SeedableRng};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone)]
struct Room {
    pin: String,
    clients: Vec<Arc<Mutex<TcpStream>>>,
    hunter_first: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9090").await?;

    let mut rooms = HashMap::new();

    println!("Pornire server cu adresa: 127.0.0.1:9090");

    let mut buffer = [0; 1024];

    loop {
        let (socket, addr) = listener.accept().await?;

        let socket = Arc::new(Mutex::new(socket));

        let n = socket.lock().await.read(&mut buffer).await?;

        if n == 0 {
            println!("Client deconectat.");
        }

        let input = String::from_utf8_lossy(&buffer[..n]).to_string();
        println!("Comanda primita de la clientul \"{}\": {}", addr, input);

        let mut commands = input.split_whitespace();

        let command = commands.next();

        match command {
            Some("create") => {
                if let (Some(pin), Some(role)) = (commands.next(), commands.next()) {
                    println!("Incercam sa cream camera cu pin-ul: \"{}\"  si rolul: \"{}\"", pin, role);

                    let hunter_first = role.to_uppercase() == "HUNTER";

                    let room = Room {
                        pin: pin.to_string(),
                        clients: vec![socket.clone()],
                        hunter_first,
                    };

                    if rooms.contains_key(pin) {
                        println!("Nu se poate crea; Camera cu pinul: \"{}\" exista.", pin);

                        print_socket(socket.clone(), "Room already exists\n").await;
                    } else {
                        print_socket(socket.clone(), "READY\n").await;

                        rooms.insert(pin.to_string(), room.clone());

                        println!("Camera creata, pin: {}", pin);
                    }
                } else {
                    println!("Invalid create command format: {}", input);
                }
            }

            Some("join") => {
                if let Some(pin) = commands.next() {
                    println!("Incecam sa dam join la camera cu pinul: {}", pin);

                    if let Some(room) = rooms.get_mut(pin) {
                        room.clients.push(socket.clone());


                        if room.clients.len() == 2 {
                            println!("S-a dat join cu succes. 2/2 jucatori conectati");

                            print_socket(socket.clone(), "READY\n").await;

                            println!("Se atribuie rolurile");

                            let role_message_1 = if room.hunter_first { "HUNTER\n" } else { "MOUSE\n" };

                            let role_message_2 = if room.hunter_first { "MOUSE\n" } else { "HUNTER\n" };

                            for (index, client) in room.clients.iter_mut().enumerate() {
                                let role_message = if index == 0 { role_message_1 } else { role_message_2 };

                                print_socket(client.clone(), role_message).await;
                            }

                            let pin_clone = pin;

                            let room_clone = room.clone();

                            tokio::spawn(async move {
                                start_game(room_clone).await;
                            });

                            if rooms.remove(pin_clone).is_some() {
                                println!("Room-ul cu pin-ul {} a fost scos din funnctiune pentru interaciuni.", pin_clone);
                            } else {
                                println!("Room-ul cu pin-ul {} nu a fost găsit pentru ștergere.", pin_clone);
                            }


                            if rooms.remove(pin_clone).is_some() {
                                println!("Room-ul cu pin-ul {} a fost șters.", pin_clone);
                            } else {
                                println!("Room-ul cu pin-ul {} nu a fost găsit pentru ștergere.", pin_clone);
                            }
                        } else {
                            println!("Clienti insuficienti: {}", pin);
                        }
                    } else {
                        println!("Camera cu pinul {} nu a fost gasita.", pin);

                        print_socket(socket.clone(), "Camera nu a fost gasita\n").await;
                    }
                } else {
                    println!("Format invalid: {}", input);
                }
            }

            Some("computer") => {
                let client = Arc::clone(&socket);
                tokio::spawn(start_computer_mode(client));
            }

            _ => {
                println!("Comanda nerecunoscuta: {}", input);
            }
        }
    }
}


async fn print_to_room(room: Room, message: &str) {
    let aux = room.clone();

    for client in aux.clients.clone() {
        print_socket(client.clone(), message).await;
    }
}


async fn start_game(room: Room) {
    println!("Incepe jocul in modul multi player, in camera cu pinul: {}", room.pin);

    let mut turn = if room.hunter_first { 0 } else { 1 };

    let first = turn;

    let mut rng = rand_chacha::ChaChaRng::from_entropy();

    let random = rng.gen_range(30..=60);

    let mut start = (5, 5);

    let mut matrix = [[0i8; 11]; 11];

    drum::adaugare_obstacole(&mut matrix, random);

    matrix[5][5] = -1;

    let mesaj = ["CONTINUA\n", "DECONECTAT\n", "HUNTER WIN\n", "MOUSE WIN\n"];

    let mut cnt = 0;

    loop {
        let matrix_state = format!("{:?}\n", matrix);

        print_to_room(room.clone(), mesaj[cnt]).await;

        print_to_room(room.clone(), matrix_state.as_str()).await;

        if cnt >= 1
        {
            return;
        }

        if turn == first
        {
            print_to_room(room.clone(), "HUNTER\n").await;
        } else {
            print_to_room(room.clone(), "MOUSE\n").await;
        }


        let current_client = &room.clients[turn];

        let mut move_buffer = [0; 1024];

        let mut locked_client = current_client.lock().await;


        let n = match locked_client.read(&mut move_buffer).await {
            Ok(n) => n,
            Err(_) => {
                println!("Eroare la citirea de la client.");
                return;
            }
        };

        let player_move = String::from_utf8_lossy(&move_buffer[..n]).trim().to_string();

        if n == 0 || player_move == "iesire"
        {
            println!("Client deconectat");
            cnt = 1;
        } else {
            println!("Jucatorul {} a făcut mișcarea: {}", turn + 1, player_move);

            let number = player_move.parse::<usize>().unwrap_or(61);

            if turn == first {
                let x = (number - 1) / 11;
                let y = (number - 1) % 11;

                println!("Mutare hunter: {}x{}", x, y);

                matrix[x][y] = 1;

                let (_distanta, parent, destinatii_minime) = drum::bfs(&matrix, start);

                let mut drum_minim = Vec::new();

                for &dest in destinatii_minime.iter() {
                    let drum = drum::reconstruire_drum(&parent, dest);

                    if drum.len() < drum_minim.len() || drum_minim.is_empty() {
                        drum_minim = drum;
                    }
                }

                if drum_minim.is_empty() {
                    cnt = 2;
                }
            } else {
                let x = (number - 1) / 11;
                let y = (number - 1) % 11;

                println!("Mutare hunter: {}x{}", x, y);

                matrix[start.0][start.1] = 0;

                matrix[x][y] = -1;

                start.0 = x;

                start.1 = y;

                if (start.0 == 0 || start.0 == 10 || start.1 == 0 || start.1 == 10) && matrix[start.0][start.1] == -1
                {
                    cnt = 3;
                }
            }

            turn = (turn + 1) % 2;
        }
    }
}

async fn print_socket(client: Arc<Mutex<TcpStream>>, message: &str)
{
    let mut locked_client = client.lock().await;


    if let Err(e) = locked_client.write_all(message.as_bytes()).await {
        eprintln!("Eroare latrimiterea mesajului: \"{}\" catre client", e);
    }

    ;
}

async fn start_computer_mode(client: Arc<Mutex<TcpStream>>) {
    println!("Incepe jocul in modul single player(vs computer)");

    let mut rng = rand_chacha::ChaChaRng::from_entropy();

    let random = rng.gen_range(30..=60);

    let mut matrix = [[0; 11]; 11];

    drum::adaugare_obstacole(&mut matrix, random);

    matrix[5][5] = -1;

    let mut start = (5, 5);

    let mesaj = ["CONTINUA\n", "HUNTER WIN\n", "MOUSE WIN\n"];

    let mut cnt = 0;

    let mut matrix_state;

    loop {
        print_socket(client.clone(), mesaj[cnt]).await;

        matrix_state = format!("{:?}\n", matrix);
        print_socket(client.clone(), matrix_state.as_str()).await;

        let mut buffer = [0; 1024];

        if cnt > 0
        {
            return;
        }

        let mut locked_client = client.lock().await;
        let n = match locked_client.read(&mut buffer).await {
            Ok(n) => n,
            Err(_) => {
                println!("Eroare la citirea de la client.");
                return;
            }
        };

        let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

        match n {
            0 => {
                println!("Client deconectat.");
                return;
            }
            _ => {
                if input == "iesire" {
                    print_socket(client.clone(), "EXIT").await;
                    return;
                } else {
                    let number = input.parse::<usize>().unwrap();

                    let x = (number - 1) / 11;
                    let y = (number - 1) % 11;

                    println!("Mutare client: {}x{}", x, y);

                    matrix[x][y] = 1;

                    if (start.0 == 0 || start.0 == 10 || start.1 == 0 || start.1 == 10) && matrix[start.0][start.1] == -1
                    {
                        cnt = 2;
                    } else {
                        let (_distanta, parent, destinatii_minime) = drum::bfs(&matrix, start);

                        let mut drum_minim = Vec::new();

                        for &dest in destinatii_minime.iter() {
                            let drum = drum::reconstruire_drum(&parent, dest);

                            if drum.len() < drum_minim.len() || drum_minim.is_empty() {
                                drum_minim = drum;
                            }
                        }

                        if !drum_minim.is_empty() {
                            let old_start = start;

                            start = drum_minim[1];

                            matrix[old_start.0][old_start.1] = 0;

                            matrix[start.0][start.1] = -1;
                        } else if cnt != 2
                        {
                            cnt = 1;
                        }
                    }
                }
            }
        }
    }
}
