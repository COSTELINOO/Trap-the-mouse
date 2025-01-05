use crate::parser::*;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;
use std::error::Error;
use iced::wgpu::naga::Statement::Return;
use tokio::sync::mpsc as tokio_mpsc;

pub async fn tcp_handler(
    mut rx: tokio_mpsc::Receiver<String>,  // Canal pentru comenzi trimise de la thread-ul principal
    server_tx: tokio_mpsc::Sender<String>  // Canal pentru a trimite răspunsuri înapoi la thread-ul principal
) -> Result<(), Box<dyn Error + Send + Sync>> {
println!("Starting TCP server");

        // Așteaptă o comandă din canalul rx
        match rx.recv().await {
            Some(command) => {
                if command.starts_with("create") || command.starts_with("join") || command == "computer" {
                    let stream = TcpStream::connect("127.0.0.1:9090")?;
                    let mut reader = BufReader::new(stream.try_clone()?);
                    let mut writer = stream;

                    writeln!(writer, "{}", command)?;
                    writer.flush()?;

                    // Logica pentru fiecare comandă trimisă
                    if command == "computer" {
                        computer_mode(&mut reader, &mut writer, &mut rx, &server_tx).await?;
                    } else if command.starts_with("create") || command.starts_with("join") {
                        println!("Creating server aaaa :: {}",command);


                        create_or_join_mode(&command, &mut reader, & mut rx,&mut writer, &server_tx).await?;


                    }
                } else if command == "EXITT" {
                    println!("Ieșire din aplicație...");
                    exit(0);
                } else {
                    println!("Comandă necunoscută.");
                }
            },
            None => {
                println!("Canalul pentru comenzi a fost închis.");
                //break;
            }
        }


    Ok(())
}

async fn computer_mode(
    reader: &mut BufReader<TcpStream>,
    writer: &mut TcpStream,
     rx: & mut tokio_mpsc::Receiver<String>,
    server_tx: &tokio_mpsc::Sender<String>
) -> Result<(), Box<dyn Error + Send + Sync>>
{

    println!("Conectat ca 'computer'. Aștept confirmarea serverului...");

    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);
                        println!();

                        let mut aux:String="CONTINUA".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;


                        // Citire mișcare din canal
                        if let Some(move_input) = rx.recv().await {

                            writeln!(writer, "{}", move_input)?;
                            writer.flush()?;

                            // Trimite mesaj înapoi către thread-ul principal
                           // server_tx.send(move_input).await?;
                            println!("{}", move_input);

                        }
                    }
                }
                "HUNTER WIN" => {
                    println!("HUNTER WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let mut aux:String="HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }
                "MOUSE WIN" => {
                    println!("MOUSE WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);

                        let mut aux:String="MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                        print_matrix(matrice);
                    }
                    return Ok(());
                }
                _ => return Ok(())
            }
        } else {
            println!("Eroare la primirea statusului.");
            return Ok(());
        }
    }
}

async fn create_or_join_mode(
    command: &str,
    reader: &mut BufReader<TcpStream>,
    rx: & mut tokio_mpsc::Receiver<String>,
    writer: &mut TcpStream,
    server_tx: &tokio_mpsc::Sender<String>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("Aștept alocarea rolului...");

    if let Some(response) = read_from_socket(reader).await?
    {
        match response.as_str() {
            "READY" =>
                {
                    let mut aux: String = "READY".to_string();
                    server_tx.send(aux).await?;
                },
           _ =>{}
        }
    }

    if let Some(response) = read_from_socket(reader).await? {
        match response.as_str() {
            "READY"=>
                {
                    let mut aux:String="READY".to_string();
                    server_tx.send(aux).await?;
                }
            "MOUSE" => {
                let mut aux:String="ROL MOUSE".to_string();
                server_tx.send(aux).await?;
                mouse_mode(reader, writer,rx, server_tx).await?;
                return Ok(());
            }
            "HUNTER" => {
                let mut aux:String="ROL HUNTER".to_string();
                server_tx.send(aux).await?;

                hunter_mode(reader, writer,rx, server_tx).await?;
                return Ok(());
            }
            "EXIT" => {

                println!("Serverul a trimis 'EXIT'. Revenire la meniul principal.");
                return Ok(());
            }
            _ => {
                println!("Răspuns necunoscut: {}", response);
               return Ok(());
            }
        }

    }

    Ok(())
}

async fn hunter_mode(
    reader: &mut BufReader<TcpStream>,
    writer: &mut TcpStream,
    rx: & mut tokio_mpsc::Receiver<String>,
    server_tx: &tokio_mpsc::Sender<String>
) -> Result<(), Box<dyn Error + Send + Sync>>
{


    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let ceva = read_from_socket(reader).await?;
                        let mut aux:String="RUNDA MOUSE".to_string();

                        if ceva.unwrap().trim() == "HUNTER" {

                            aux="RUNDA HUNTER".to_string();
                            aux.push_str(&matrix);
                            server_tx.send(aux.clone()).await?;
                            // Citire mișcare din canal
                            if let Some(move_input) = rx.recv().await {
                                if move_input=="iesire"
                                {
                                    return Ok(());
                                }
                                writeln!(writer, "{}", move_input)?;
                                writer.flush()?;

                                // Trimite mesaj înapoi către thread-ul principal
                                // server_tx.send(move_input).await?;
                                println!("{}", move_input);

                            }

                        }else {
                            aux.push_str(&matrix);
                            server_tx.send(aux.clone()).await?;
                        }

                    }
                }
                "HUNTER WIN" => {
                    println!("HUNTER WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let mut aux:String="HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }
                "MOUSE WIN" => {
                    println!("MOUSE WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let mut aux:String="MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }
                "DECONECTAT" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                     ;

                        let mut aux:String="DECONECTAT".to_string();

                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }

                _ => {

                    let mut aux:String="DECONECTAT".to_string();

                    server_tx.send(aux).await?;
                    return Ok(())}
            }
        } else {


            let mut aux:String="DECONECTAT".to_string();

            server_tx.send(aux).await?;

            return Ok(());
        }
    }
}

async fn mouse_mode(
    reader: &mut BufReader<TcpStream>,
    writer: &mut TcpStream,
    rx: & mut tokio_mpsc::Receiver<String>,
    server_tx: &tokio_mpsc::Sender<String>
) -> Result<(), Box<dyn Error + Send + Sync>> {

    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    println!("Așteptați mesajul de la server...");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let ceva = read_from_socket(reader).await?;
                        let mut aux:String="RUNDA HUNTER".to_string();


                        if ceva.unwrap().trim() == "MOUSE" {

                            aux="RUNDA MOUSE".to_string();
                            aux.push_str(&matrix);
                            server_tx.send(aux).await?;
                            // Citire mișcare din canal
                            if let Some(move_input) = rx.recv().await {
                                if move_input=="iesire"
                                {
                                    return Ok(());
                                }
                                writeln!(writer, "{}", move_input)?;
                                writer.flush()?;

                                // Trimite mesaj înapoi către thread-ul principal
                                // server_tx.send(move_input).await?;
                                println!("{}", move_input);

                            }
                        }else {
                            aux.push_str(&matrix);
                            server_tx.send(aux).await?;
                        }

                    }
                }
                "HUNTER WIN" => {
                    println!("HUNTER WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let mut aux:String="HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }
                "MOUSE WIN" => {
                    println!("MOUSE WIN");
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let matrice: [[i8; 11]; 11] = parse_matrix(&matrix);
                        print_matrix(matrice);

                        let mut aux:String="MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }
                "DECONECTAT" => {
                    if let Some(matrix) = read_from_socket(reader).await? {


                        let mut aux:String="DECONECTAT".to_string();

                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }

                _ => {println!("Mesaj necunoscut de la server: {}", status);
                    let mut aux:String="DECONECTAT".to_string();

                    server_tx.send(aux).await?;}
            }
        } else {
            let mut aux:String="DECONECTAT".to_string();

            server_tx.send(aux).await?;
            return Ok(());
        }
    }
}

async fn read_from_socket(reader: &mut BufReader<TcpStream>) ->Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let mut buffer = String::new();

   // loop {
        buffer.clear();
        let n = reader.read_line(&mut buffer)?;

        if n > 0 {
            return Ok(Some(buffer.trim().to_string()));
        } else {
            if reader.get_ref().peer_addr().is_err() {
                return Ok(None); // Conexiunea s-a închis
            }
        }
    Ok(None)
  // }
}

