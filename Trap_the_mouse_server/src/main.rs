use rand_chacha::ChaChaRng;
use rand::{Rng, SeedableRng};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Notify, mpsc};
mod drum;
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

#[derive(Clone)]
struct Room {
    pin: String,
    clients: Vec<Arc<Mutex<TcpStream>>>, // Fiecare client este protejat de un Mutex
    hunter_first: bool,
    notify: Arc<Notify>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9090").await?;
    let mut rooms = (HashMap::new()); // Camerele sunt doar într-un HashMap

    println!("Server started on 127.0.0.1:9090");

    loop {
        let (socket, _) = listener.accept().await?;
        // let mut rooms = Arc::clone(& mut rooms);


        let socket = Arc::new(Mutex::new(socket));
        let mut buffer = [0; 1024];


        let n = socket.lock().await.read(&mut buffer).await?;

        if n == 0 {
            println!("Client disconnected.");
        }

        let input = String::from_utf8_lossy(&buffer[..n]).to_string();
        let mut commands = input.split_whitespace();
        let command = commands.next();

        match command {
            Some("create") => {
                if let (Some(pin), Some(role)) = (commands.next(), commands.next()) {
                    println!("Create command received. Pin: {}, Role: {}", pin, role);
                    let hunter_first = role.to_uppercase() == "HUNTER";

                    // Creăm o cameră nouă
                    let mut room = Room {
                        pin: pin.to_string(),
                        clients: vec![socket.clone()],
                        hunter_first,
                        notify: Arc::new(Notify::new()),
                    };

                    // Adăugăm camera în hash map
                    if rooms.contains_key(pin) {
                        println!("Room with pin {} already exists.", pin);
                        let mut locked_socket = socket.lock().await;
                        locked_socket.write_all(b"Room already exists\n").await.unwrap();
                    }
                    else{


                        tokio::spawn(async move {
                            print_socket(socket.clone(), "READY\n").await;
                        });

                        rooms.insert(pin.to_string(), room.clone());
                        println!("Room created with pin: {}", pin);
                        println!("Created room with pin: {}", rooms.get(pin.clone()).unwrap().clients.len());
                    }



                    // Trimitem mesajul clientului că a fost creat
                    //   let mut locked_socket = socket.lock().await;
                    // locked_socket.write_all(b"HUNTER\n").await.unwrap();
                    // locked_socket.write_all(b"CONTINUA\n").await.unwrap();
                } else {
                    println!("Invalid create command format: {}", input);
                }
            }

            Some("join") => {
                println!("Join command received.");
                if let Some(pin) = commands.next() {
                    println!("Join command received. Pin: {}", pin);
                    if let Some(mut room) = rooms.get_mut(pin) {
                        room.clients.push(socket.clone());

                        println!(
                            "Client joined room: {}. Total clients: {}",
                            pin,
                            room.clients.len()
                        );

                        if room.clients.len() == 2 {
                            println!("Both clients joined. Assigning roles.");
                            tokio::spawn(async move {
                                print_socket(socket.clone(), "READY\n").await;
                            });


                            let role_message_1 = if room.hunter_first { "HUNTER\n" } else { "MOUSE\n" };
                            let role_message_2 = if room.hunter_first { "MOUSE\n" } else { "HUNTER\n" };

                            for(index, client) in room.clients.iter_mut().enumerate() {
                                let mut locked_client = client.lock().await;
                                let role_message = if index == 0 { role_message_1 } else { role_message_2 };
                                locked_client.write_all(role_message.as_bytes()).await.unwrap();



                            }
                            let pin_clone = pin.clone(); // Clonează pin-ul pentru a-l folosi în task
                            let room_clone = room.clone(); // Clonează referința la camera curentă

                            tokio::spawn(async move {
                                start_game(room_clone).await ;
                            });

                            // Ștergerea camerei din hash map după ce jocul s-a terminat
                            if let Some(_) = rooms.remove(pin_clone) {
                                println!("Room-ul cu pin-ul {} a fost șters.", pin_clone);
                            } else {
                                println!("Room-ul cu pin-ul {} nu a fost găsit pentru ștergere.", pin_clone);
                            }


                            if rooms.remove(pin_clone).is_some() {
                                println!("Room-ul cu pin-ul {} a fost șters.", pin_clone);
                            } else {
                                println!("Room-ul cu pin-ul {} nu a fost găsit pentru ștergere.", pin_clone);
                            }

                        } else {
                            println!("Waiting for another client to join room: {}", pin);
                        }
                    } else {
                        println!("Room with pin {} not found.", pin);
                        let mut locked_socket = socket.lock().await;
                        locked_socket.write_all(b"Room not found\n").await.unwrap();
                    }
                } else {
                    println!("Invalid join command format: {}", input);
                }
            }

            Some("computer") => {

                println!("Starting computer mode");

                // Începem modul computerizat, respectiv jocul împotriva botului
                let client = Arc::clone(&socket);
                tokio::spawn(start_computer_mode(client));



            }

            _ => {
                println!("Unknown command: {}", input);
            }
        }}}







async fn print_to_room(room: Room, message: &str) {

    let aux=room.clone();
    for client in aux.clients.clone(){

        let mut locked_client = client.lock().await;

        locked_client.write_all(message.as_bytes()).await.unwrap();

    }
}
async fn start_game(room: Room) {
    println!("Starting game in room: {}", room.pin);


    let mut matrix = [[0i8; 11]; 11];
    let mut turn = if room.hunter_first { 0 } else { 1 };
    println!("Starting game in room: {}", room.hunter_first);
    let first=turn;

    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    let random = rng.gen_range(30..=60);

    drum::adaugare_obstacole(&mut matrix, random);
    matrix[5][5] = -1;
    let mut start = (5, 5);

    // Trimiterea stării matricei către toți clienții

    let mesaj=["CONTINUA\n","DECONECTAT\n","HUNTER WIN\n","MOUSE WIN\n"];
    let mut cnt=0;
    loop {


        let matrix_state = format!("{:?}\n", matrix);
        print_to_room(room.clone(),mesaj[cnt]).await;
        print_to_room(room.clone(),matrix_state.as_str()).await;

        if cnt>=1
        {
            return;
        }

        if turn==first
        {
            print_to_room(room.clone(),"HUNTER\n").await;
        }
        else {
            print_to_room(room.clone(),"MOUSE\n").await;
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
        if n == 0 {
            cnt=1;

        }


        if player_move == "iesire" {
            cnt=1;


        }
        else{
            println!("Clientul {} a făcut mișcarea: {}", turn + 1, player_move);


            let mut number=player_move.parse::<usize>().unwrap();

            println!("Number: {}", number);
            if turn==first {
                println!("Number hunter: {}", number);
                let x = (number - 1) / 11;
                let y = (number - 1) % 11;
                matrix[x][y] = 1;

                let (distanta, parent, destinatii_minime) = drum::bfs(&matrix, start);
                let mut drum_minim = Vec::new();

                for &dest in destinatii_minime.iter() {
                    let drum = drum::reconstruire_drum(&parent, dest);
                    if drum.len() < drum_minim.len() || drum_minim.is_empty() {
                        drum_minim = drum;
                    }
                }

                if drum_minim.is_empty() {
                    cnt=2;
                }
            }
            else {
                println!("Number Mouse: {}", number);
                let x = (number - 1) / 11;
                let y = (number - 1) % 11;
                println!("{:?}",start);
                println!("{} {}",x,y);
                matrix[start.0][start.1] = 0;
                matrix[x][y] = -1;
                println!("{}",matrix[x][y]);
                start.0=x;
                start.1=y;

                println!("{:?}",start);

                if (start.0 == 0 || start.0 == 10 || start.1 == 0 || start.1 == 10)
                    && matrix[start.0][start.1] == -1
                {
                    cnt=3;
                }


            }


            turn = (turn + 1) % 2;
        }}
}

async fn print_socket(client: Arc<Mutex<TcpStream>>,message: &str)
{
    let mut locked_client = client.lock().await;
    locked_client.write_all(message.as_bytes()).await.unwrap();
}
async fn start_computer_mode(client: Arc<Mutex<TcpStream>>) {
    let mut matrix = [[0; 11]; 11];
    let mut rng = rand_chacha::ChaChaRng::from_entropy();
    let random = rng.gen_range(30..=60);
    drum::adaugare_obstacole(&mut matrix, random);
    matrix[5][5] = -1;
    let mut start = (5, 5);
    let mesaj=["CONTINUA\n","HUNTER WIN\n","MOUSE WIN\n"];
    let mut cnt=0;
    let mut matrix_state = format!("{:?}\n", matrix);

    loop {
        print_socket(client.clone(),mesaj[cnt]).await;
        matrix_state = format!("{:?}\n", matrix);
        print_socket(client.clone(),matrix_state.as_str()).await;
        let mut buffer = [0; 1024];
        if cnt>0
        {
            return;
        }

        let n = {
            let mut locked_client = client.lock().await;
            locked_client.read(&mut buffer).await.unwrap()
        };

        match n {
            0 => {
                println!("Client deconectat.");
               return
            }
            _=> {
                let input = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                if input == "iesire" {
                    let mut locked_client = client.lock().await;
                    locked_client.write_all(b"EXIT\n").await.unwrap();
                    return
                }else {
                    let number = input.parse::<usize>().unwrap();
                    println!("Number: {}", number);
                    let x = (number - 1) / 11;
                    let y = (number - 1) % 11;
                    println!("x: {}   y: {}", x,y);
                    matrix[x][y] = 1;
println!("{:?}",matrix);
                    if (start.0 == 0 || start.0 == 10 || start.1 == 0 || start.1 == 10)
                        && matrix[start.0][start.1] == -1
                    {
                       cnt=2;
                    }
else{
                    let (distanta, parent, destinatii_minime) = drum::bfs(&matrix, start);
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
                    } else {
                        if cnt!=2
                        {
                            cnt=1;
                        }

                    }
}

                }
            }
        }
    }
}
