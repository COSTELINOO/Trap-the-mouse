use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;
use std::error::Error;
use tokio::sync::mpsc as tokio_mpsc;

pub async fn tcp_handler(mut rx: tokio_mpsc::Receiver<String>, server_tx: tokio_mpsc::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {

    match rx.recv().await {
        Some(command) => {
            if command.starts_with("create") || command.starts_with("join") || command == "computer" {
                let stream = TcpStream::connect("127.0.0.1:9090")?;

                let mut reader = BufReader::new(stream.try_clone()?);

                let mut writer = stream;

                writeln!(writer, "{}", command)?;

                writer.flush()?;

                if command == "computer" {
                    computer_mode(&mut reader, &mut writer, &mut rx, &server_tx).await?;
                } else if command.starts_with("create") || command.starts_with("join") {
                    create_or_join_mode(&mut reader, &mut rx, &mut writer, &server_tx).await?;
                }
            } else if command == "EXITT" {
                exit(0);
            }
        }
        None => {
        }
    }

    Ok(())
}

async fn computer_mode(reader: &mut BufReader<TcpStream>, writer: &mut TcpStream, rx: &mut tokio_mpsc::Receiver<String>, server_tx: &tokio_mpsc::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>>
{

    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "CONTINUA".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;

                        if let Some(move_input) = rx.recv().await {
                            writeln!(writer, "{}", move_input)?;
                            writer.flush()?;
                        }
                    }
                }

                "HUNTER WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }

                    return Ok(());
                }
                "MOUSE WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }

                    return Ok(());
                }
                _ => return Ok(())
            }
        } else {

            return Ok(());
        }
    }
}

async fn create_or_join_mode(reader: &mut BufReader<TcpStream>, rx: &mut tokio_mpsc::Receiver<String>, writer: &mut TcpStream, server_tx: &tokio_mpsc::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {

    if let Some(response) = read_from_socket(reader).await?
    {
        match response.as_str() {
            "READY" =>
                {
                    let aux: String = "READY".to_string();
                    server_tx.send(aux).await?;
                }

            _ => {}
        }
    }

    if let Some(response) = read_from_socket(reader).await? {
        match response.as_str() {
            "READY" =>
                {
                    let aux: String = "READY".to_string();
                    server_tx.send(aux).await?;
                }

            "MOUSE" => {
                let aux: String = "ROL MOUSE".to_string();
                server_tx.send(aux).await?;

                mouse_mode(reader, writer, rx, server_tx).await?;

                return Ok(());
            }
            "HUNTER" => {
                let aux: String = "ROL HUNTER".to_string();
                server_tx.send(aux).await?;

                hunter_mode(reader, writer, rx, server_tx).await?;

                return Ok(());
            }

            "EXIT" => {

                return Ok(());
            }
            _ => {

                return Ok(());
            }
        }
    }

    Ok(())
}

async fn hunter_mode(reader: &mut BufReader<TcpStream>, writer: &mut TcpStream, rx: &mut tokio_mpsc::Receiver<String>, server_tx: &tokio_mpsc::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>>
{
    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "RUNDA MOUSE".to_string();

                        let ceva = read_from_socket(reader).await?;

                        if ceva.unwrap().trim() == "HUNTER" {
                            aux = "RUNDA HUNTER".to_string();
                            aux.push_str(&matrix);
                            server_tx.send(aux.clone()).await?;


                            if let Some(move_input) = rx.recv().await {
                                if move_input == "iesire"
                                {
                                    return Ok(());
                                }

                                writeln!(writer, "{}", move_input)?;
                                writer.flush()?;
                            }
                        } else {
                            aux.push_str(&matrix);
                            server_tx.send(aux.clone()).await?;
                        }
                    }
                }

                "HUNTER WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }

                    return Ok(());
                }

                "MOUSE WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }
                    return Ok(());
                }

                "DECONECTAT" => {
                    let aux: String = "DECONECTAT".to_string();
                    server_tx.send(aux).await?;

                    return Ok(());
                }

                _ => {
                    let aux: String = "DECONECTAT".to_string();
                    server_tx.send(aux).await?;

                    return Ok(());
                }
            }
        } else {
            let aux: String = "DECONECTAT".to_string();
            server_tx.send(aux).await?;

            return Ok(());
        }
    }
}

async fn mouse_mode(reader: &mut BufReader<TcpStream>, writer: &mut TcpStream, rx: &mut tokio_mpsc::Receiver<String>, server_tx: &tokio_mpsc::Sender<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        if let Some(status) = read_from_socket(reader).await? {
            match status.as_str() {
                "CONTINUA" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "RUNDA HUNTER".to_string();

                        let ceva = read_from_socket(reader).await?;

                        if ceva.unwrap().trim() == "MOUSE" {
                            aux = "RUNDA MOUSE".to_string();
                            aux.push_str(&matrix);
                            server_tx.send(aux).await?;

                            if let Some(move_input) = rx.recv().await {
                                if move_input == "iesire"
                                {
                                    return Ok(());
                                }

                                writeln!(writer, "{}", move_input)?;
                                writer.flush()?;
                            }
                        } else {
                            aux.push_str(&matrix);
                            server_tx.send(aux).await?;
                        }
                    }
                }
                "HUNTER WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "HUNTER".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }

                    return Ok(());
                }
                "MOUSE WIN" => {
                    if let Some(matrix) = read_from_socket(reader).await? {
                        let mut aux: String = "MOUSE".to_string();
                        aux.push_str(&matrix);
                        server_tx.send(aux).await?;
                    }

                    return Ok(());
                }
                "DECONECTAT" => {
                    let aux: String = "DECONECTAT".to_string();
                    server_tx.send(aux).await?;

                    return Ok(());
                }

                _ => {
                    let aux: String = "DECONECTAT".to_string();
                    server_tx.send(aux).await?;
                }
            }
        } else {
            let aux: String = "DECONECTAT".to_string();
            server_tx.send(aux).await?;
            return Ok(());
        }
    }
}

async fn read_from_socket(reader: &mut BufReader<TcpStream>) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let mut buffer = String::new();

    buffer.clear();

    let n = reader.read_line(&mut buffer)?;

    if n > 0 {
        return Ok(Some(buffer.trim().to_string()));
    } else {
        if reader.get_ref().peer_addr().is_err() {
            return Ok(None);
        }
    }

    Ok(None)
}

