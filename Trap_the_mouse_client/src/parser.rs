use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::{MyApp};
use crate::model::Role;
use tokio::sync::mpsc;

pub fn parse_message(app: &mut MyApp, mesaj: String)
{
    let aux;

    if mesaj.starts_with("CONTINUA") {
        aux = mesaj.replace("CONTINUA", "");

        app.matrix = parse_matrix(aux.as_str());
    } else if mesaj.starts_with("DECONECTAT") {
        app.winner = Option::from("DECONECTAT".to_string());
    } else if mesaj.starts_with("READY")
    {
        app.ready_room = true;
    } else if mesaj.starts_with("MOUSE") {
        aux = mesaj.replace("MOUSE", "");

        app.winner = Option::from("MOUSE".to_string());

        app.matrix = parse_matrix(aux.as_str());
    } else if mesaj.starts_with("HUNTER") {
        aux = mesaj.replace("HUNTER", "");

        app.winner = Option::from("HUNTER".to_string());

        app.matrix = parse_matrix(aux.as_str());
    } else if mesaj.starts_with("ROL MOUSE") {
        app.model.role = Option::from(Role::Mouse);
    } else if mesaj.starts_with("ROL HUNTER") {
        app.model.role = Option::from(Role::Hunter);
    } else if mesaj.starts_with("RUNDA HUNTER") {
        app.model.current_move = Option::from(Role::Hunter);

        aux = mesaj.replace("RUNDA HUNTER", "");

        app.matrix = parse_matrix(aux.as_str());
    } else if mesaj.starts_with("RUNDA MOUSE") {
        app.model.current_move = Option::from(Role::Mouse);

        aux = mesaj.replace("RUNDA MOUSE", "");

        app.matrix = parse_matrix(aux.as_str());
    }
}


pub fn parse_matrix(input: &str) -> [[i8; 11]; 11] {
    let mut matrix = [[0i8; 11]; 11];

    let rows: Vec<&str> = input
        .trim_matches(|c| c == '[' || c == ']')
        .split("], [")
        .collect();

    for (i, row) in rows.iter().enumerate() {
        let values: Vec<i8> = row
            .split(", ")
            .filter_map(|num| num.parse::<i8>().ok())
            .collect();

        for (j, &value) in values.iter().enumerate() {
            matrix[i][j] = value;
        }
    }

    matrix
}

pub async fn do_async_work(rx: Arc<Mutex<mpsc::Receiver<String>>>) -> String {
    let mut rx = rx.lock().await;

    rx.recv().await.unwrap_or_else(|| "".to_string())
}