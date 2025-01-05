use rand::Rng;
use std::collections::VecDeque;
use std::io;

fn initializare_tabla() ->Vec<Vec<i32>> {
    vec![vec![ 0; 11]; 11]
}
pub fn adaugare_obstacole(tabla:&mut [[i8; 11]; 11],nr_obstacole:i32)
{
    let mut rng = rand::thread_rng();
    let mut cnt=1;
    let mut random;
    while cnt<=nr_obstacole
    {
        random=rng.gen_range(1..=121);

        tabla[(random-1)/11][(random-1) % 11]=1;

        cnt+=1;
    }
}

pub fn bfs(tabla: &[[i8; 11]; 11], start: (usize, usize)) -> (Vec<Vec<i32>>, Vec<Vec<Option<(usize, usize)>>>, Vec<(usize, usize)>) {
    let n = tabla.len();
    let m = tabla[0].len();
    let mut distanta = vec![vec![-1; m]; n];
    let mut parent = vec![vec![None; m]; n];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    distanta[start.0][start.1] = 0;


    let dir_pare = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, -1), (1, 0)];
    let dir_impare = vec![(-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0), (1, 1)];

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


pub fn reconstruire_drum(parent: &Vec<Vec<Option<(usize, usize)>>>, dest: (usize, usize)) -> Vec<(usize, usize)> {
    let mut drum = Vec::new();
    let mut curent = Some(dest);

    while let Some((x, y)) = curent {
        drum.push((x, y));
        curent = parent[x][y];
    }

    drum.reverse();
    drum
}
