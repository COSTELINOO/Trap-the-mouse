use rand::Rng;
use std::collections::VecDeque;
use std::io;

fn initializare_tabla() ->Vec<Vec<i32>> {
    vec![vec![ 0; 11]; 11]
}
fn adaugare_obstacole(tabla: &mut Vec<Vec<i32>>,nr_obstacole:i32)
{
    let mut rng = rand::thread_rng();
    let mut cnt=1;
    let mut random;
    while cnt<=nr_obstacole
    {
       random=rng.gen_range(1..=121);

        tabla[random/11 ][random % 11]=1;

        cnt+=1;
    }
}

fn bfs(tabla: &Vec<Vec<i32>>, start: (usize, usize)) -> (Vec<Vec<i32>>, Vec<Vec<Option<(usize, usize)>>>, Vec<(usize, usize)>) {
    let n = tabla.len();
    let m = tabla[0].len();
    let mut distanta = vec![vec![-1; m]; n];
    let mut parent = vec![vec![None; m]; n];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distanta[start.0][start.1] = 0;


    let dir_pare = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)];
    let dir_impare = vec![(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (0, 1)];

    while let Some((x, y)) = queue.pop_front() {
        let directions = if x % 2 == 0 { &dir_pare } else { &dir_impare };

        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < n && ny < m && tabla[nx][ny] == 0 && distanta[nx][ny] == -1 {
                distanta[nx][ny] = distanta[x][y] + 1;
                parent[nx][ny] = Some((x, y));
                queue.push_back((nx, ny));
            }
        }
    }


    let mut margini = Vec::new();
    for i in 0..11 {
        if i != 0 && i != 10 {
            margini.push((i, 0));
            margini.push((i, 10));
        }
    }

    for j in 0..11 {
        if j != 0 && j != 10 {
            margini.push((0, j));
            margini.push((10, j));
        }
    }


    let mut distanta_minima = i32::MAX;
    for &dest in margini.iter() {
        if distanta[dest.0][dest.1] != -1 {
            distanta_minima = distanta_minima.min(distanta[dest.0][dest.1]);
        }
    }

    let mut destinatii_minime = Vec::new();
    for &dest in margini.iter() {
        if distanta[dest.0][dest.1] == distanta_minima {
            destinatii_minime.push(dest);
        }
    }

    (distanta, parent, destinatii_minime)
}


fn reconstruire_drum(parent: &Vec<Vec<Option<(usize, usize)>>>, dest: (usize, usize)) -> Vec<(usize, usize)> {
    let mut drum = Vec::new();
    let mut curent = Some(dest);

    while let Some((x, y)) = curent {
        drum.push((x, y));
        curent = parent[x][y];
    }

    drum.reverse();
    drum
}

fn afiseaza_matricea(matrice: &Vec<Vec<i32>>) {
    for row in matrice.iter() {
        for &val in row.iter() {
            print!("{} ", val);
        }
        println!();

    }
    println!();
}
fn main() {
    let mut tabla = vec![vec![0; 11]; 11];
    adaugare_obstacole(&mut tabla, 18);
    tabla[5][5] = -1;

    afiseaza_matricea(&tabla);

    let mut start = (5, 5);
    let mut nr_mutari = 0;

    loop {


        println!("\nIntroduceti coordonatele pentru a adauga un obstacol (x y): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if coords.len() == 2 {
            let (x, y) = (coords[0], coords[1]);
            if x < 11 && y < 11 && tabla[x][y] == 0 {
                tabla[x][y] = 1;
            }
        }

        nr_mutari += 1;


        println!("\nMai doriti sa continuati? (da/nu): ");
        let mut continuare = String::new();
        io::stdin().read_line(&mut continuare).unwrap();
        if continuare.trim() == "nu" {
            break;
        }


        if (start.0 == 0 || start.0 == 10 || start.1 == 0 || start.1 == 10) && tabla[start.0][start.1] == -1 {
            println!("GAME OVER");
            break;
        }

        println!("\nMutarea #{}", nr_mutari);

        let (distanta, parent, destinatii_minime) = bfs(&tabla, start);


        let mut minim = i32::MAX;
        let mut drum_minim = Vec::new();

        for &dest in destinatii_minime.iter() {
            let drum = reconstruire_drum(&parent, dest);
            if drum.len() < minim as usize {
                minim = drum.len() as i32;
                drum_minim = drum;
            }
        }


        if !drum_minim.is_empty() {
            println!("\nCel mai scurt drum de la ({}, {}) la margine (distanta minimă = {}):", start.0, start.1, minim);
            for (x, y) in drum_minim.iter() {
                print!("({},{}) ", x, y);
            }
            println!();
        }


        if !drum_minim.is_empty() {
            let old_start = start;
            start = drum_minim[1];


            tabla[old_start.0][old_start.1] = 0;
            tabla[start.0][start.1] = -1;        
        }
    }
    afiseaza_matricea(&tabla);
}