use iced::keyboard::key::Named::Print;
use crate::{do_async_work, MyApp};
use crate::model::Role;
use crate::view::view_single_player;

pub fn parse_message(app:& mut MyApp, mesaj: String)
{
    let mut aux=String::from("");
    if mesaj.starts_with("CONTINUA"){

         aux= mesaj.replace("CONTINUA", "");
        app.matrix=parse_matrix(aux.as_str());
    }
    else if mesaj.starts_with("DECONECTAT") {
        println!("DECONECTATttttttttttttttttttttttttttttttttttttt");

        app.winner= Option::from("DECONECTAT".to_string());

    }
    else if mesaj.starts_with("READY")
    {
        app.ready_room=true;
        println!("READYyyyyyyyyyyyyyyyy");
    }
    else if mesaj.starts_with("MOUSE"){
        aux= mesaj.replace("MOUSE", "");
        app.winner=Option::from("MOUSE".to_string());
        app.matrix=parse_matrix(aux.as_str());
    }
    else if mesaj.starts_with("HUNTER") {
         aux= mesaj.replace("HUNTER", "");
        app.winner= Option::from("HUNTER".to_string());
        println!("{:?}",Some(&app.winner));
        app.matrix=parse_matrix(aux.as_str());
    }
    else if mesaj.starts_with("ROL MOUSE") {
        app.model.role=Option::from(Role::Mouse);
        println!("ROL MOUSE");
    }
    else  if mesaj.starts_with("ROL HUNTER") {
        app.model.role=Option::from(Role::Hunter);
        println!("ROL HUNTER");
    }
    else if mesaj.starts_with("RUNDA HUNTER") {
        app.model.current_move=Option::from(Role::Hunter);
        aux= mesaj.replace("RUNDA HUNTER", "");
        app.matrix=parse_matrix(aux.as_str());
        println!("RUNDA HUNTER");
    }
    else if mesaj.starts_with("RUNDA MOUSE") {
        app.model.current_move=Option::from(Role::Mouse);
        aux= mesaj.replace("RUNDA MOUSE", "");
        app.matrix=parse_matrix(aux.as_str());
        println!("RUNDA MOUSE");
    }




}



pub fn parse_matrix(input: &str) -> [[i8; 11]; 11] {
    let mut matrix = [[0i8; 11]; 11];
    let rows: Vec<&str> = input
        .trim_matches(|c| c == '[' || c == ']') // Eliminăm parantezele exterioare
        .split("], [") // Împărțim string-ul în rânduri
        .collect();

    for (i, row) in rows.iter().enumerate() {
        let values: Vec<i8> = row
            .split(", ")
            .filter_map(|num| num.parse::<i8>().ok()) // Convertim fiecare valoare în i8
            .collect();

        for (j, &value) in values.iter().enumerate() {
            matrix[i][j] = value;
        }
    }
    matrix
}


pub fn print_matrix(matrix: [[i8; 11]; 11]) {
    let mut i = 0;
    while i < 11 {
        let mut j = 0;
        if i % 2 == 1 {
            print!(" ");
        }
        while j < 11 {
            print!("{} ", matrix[i][j]);
            j += 1;
        }
        println!();
        i += 1;
    }
}

