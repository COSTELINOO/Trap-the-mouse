use rand::Rng;

fn initializare_tabla() -> [[u8; 13]; 13] {
    [[0; 13]; 13]
}

fn adaugare_obstacole(tabla: &mut [[u8; 13]; 13],nr_obstacole:i32)
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

fn liste_de_adiacenta(liste: &mut Vec<Vec<usize>>, tabla: [[u8; 13]; 13]) {
    let mut cnt_noduri = 1;
    let mut linie = 0;

    while cnt_noduri <= 121 {


        if cnt_noduri % 11 != 1 {
            let left_x = (cnt_noduri -2) / 11;
            let left_y = (cnt_noduri - 2) % 11;
            if tabla[left_x][left_y] == 0 {
                liste[cnt_noduri].push(cnt_noduri - 1);
            }
        }

        if cnt_noduri % 11 != 0 {
            let right_x = (cnt_noduri) / 11;
            let right_y = (cnt_noduri) % 11;
            if tabla[right_x][right_y] == 0 {
                liste[cnt_noduri].push(cnt_noduri + 1);
            }
        }

        if linie % 2 == 0 {
            if cnt_noduri > 11 {
                if cnt_noduri % 11 != 1 {
                    let up_left_x = (cnt_noduri - 12) / 11;
                    let up_left_y = (cnt_noduri - 12) % 11;
                    if tabla[up_left_x][up_left_y] == 0 {
                        liste[cnt_noduri].push(cnt_noduri - 12);
                    }
                }
                let up_right_x = (cnt_noduri - 11) / 11;
                let up_right_y = (cnt_noduri - 11) % 11;
                if tabla[up_right_x][up_right_y] == 0 {
                    liste[cnt_noduri].push(cnt_noduri - 11);
                }
            }

            if cnt_noduri <= 110 {
                if cnt_noduri % 11 != 1 {
                    let down_left_x = (cnt_noduri + 10) / 11;
                    let down_left_y = (cnt_noduri + 10) % 11;
                    if tabla[down_left_x][down_left_y] == 0 {
                        liste[cnt_noduri].push(cnt_noduri + 10);
                    }
                }
                let down_right_x = (cnt_noduri + 11) / 11;
                let down_right_y = (cnt_noduri + 11) % 11;
                if tabla[down_right_x][down_right_y] == 0 {
                    liste[cnt_noduri].push(cnt_noduri + 11);
                }
            }
        } else {
            if cnt_noduri > 11 {
                if cnt_noduri % 11 != 0 {
                    let up_right_x = (cnt_noduri - 11) / 11;
                    let up_right_y = (cnt_noduri - 11) % 11;
                    if tabla[up_right_x][up_right_y] == 0 {
                        liste[cnt_noduri].push(cnt_noduri - 10);
                    }
                }
                let up_left_x = (cnt_noduri - 12) / 11;
                let up_left_y = (cnt_noduri - 12) % 11;
                if tabla[up_left_x][up_left_y] == 0 {
                    liste[cnt_noduri].push(cnt_noduri - 11);
                }
            }

            if cnt_noduri <= 110 {
                if cnt_noduri % 11 != 0 {
                    let down_right_x = (cnt_noduri + 11) / 11;
                    let down_right_y = (cnt_noduri + 11) % 11;
                    if tabla[down_right_x][down_right_y] == 0 {
                        liste[cnt_noduri].push(cnt_noduri + 12);
                    }
                }
                let down_left_x = (cnt_noduri + 10) / 11;
                let down_left_y = (cnt_noduri + 10) % 11;
                if tabla[down_left_x][down_left_y] == 0 {
                    liste[cnt_noduri].push(cnt_noduri + 11);
                }
            }
        }

        cnt_noduri += 1;
        if cnt_noduri % 11 == 1 {
            linie += 1;
        }
    }
}

fn main()  {
    let mut tabla;

    tabla=initializare_tabla();
adaugare_obstacole(& mut tabla,10);
    while tabla[4][5]==1&&tabla[4][6]==1&&tabla[5][4]==1&&tabla[5][6]==1&&tabla[6][5]==1&&tabla[6][6]==1
    {
        tabla=initializare_tabla();
        adaugare_obstacole(& mut tabla,10);
    }
    let mut liste: Vec<Vec<usize>> = vec![vec![]; 123];

    liste_de_adiacenta(& mut liste,tabla);
    let mut i=1;
    let mut j;
    while i<=121
    {
        j=0;
        print!("{}: ",i);
        while j<liste[i].len()
        {
            print!("{} ",liste[i][j]);
            j=j+1;
        }
        println!();
        i=i+1;
    }
    println!();
    i=0;


    while i<11
    {
        j=0;
        while j<11
        {
            print!("{} ",tabla[i][j]);
            j=j+1;
        }
        println!();
        i+=1;
    }


}

