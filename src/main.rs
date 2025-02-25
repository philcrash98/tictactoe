use std::io;

struct Player {
    name: String,
    icon: String,
    first_player: bool,
    highscore: u32,
}

fn main() {

    let player = player_setup();

    let mut a = ["X", "X", "X", " ", " ", " ", " ", " ", " "];
    let a = gamefield(a);
    gameloop(&player, a);

    for s in a.iter() {
        println!("{}", s);
    }
}

fn gamefield(a: [&str; 9]) -> [&str; 9]{

    

    println!("\n-------------");
    println!("| {:} | {:} | {:} |", a[0], a[1], a[2]);
    println!("-------------");
    println!("| {:} | {:} | {:} |", a[3], a[4], a[5]);
    println!("-------------");
    println!("| {:} | {:} | {:} |", a[6], a[7], a[8]);
    println!("-------------");
    a
}
fn player_setup() -> Player{
    println!("Willkommen im TicTacToe!!!\n\n");

    loop {
        println!("Wähle (E)inzelspieler oder (M)ehrspieler?");

        let mut mode = String::new();

        io::stdin().read_line(&mut mode)
            .expect("Konnte Modus nicht erkennen!!!!");

        let mode = mode.trim();

        let valid_mode = ['E', 'e', 'M', 'm'];

        if mode.chars().all(|c| valid_mode.contains(&c)){
            break;
        }
        else {
            println!("Falsche Eingabe! versuche es nochmal!!!")
        
        }
    }
    
        println!("Gebe deinen Namen ein");

        let mut player_name = String::new();
        

        io::stdin().read_line(&mut player_name)
            .expect("Konnte Icon nicht erkennen!!!!");
        let player_name = player_name.trim();

        let valid_icon = ['X', 'x', 'O', 'o', '0'];

        println!("Hallo {:}!!!", player_name);

        let mut player_icon = String::new();
    loop {
        println!("Wähle dein Zeichen (X) oder (O)");

        
        

        io::stdin().read_line(&mut player_icon)
            .expect("Konnte Icon nicht erkennen!!!!");
        let player_icon = player_icon.trim();

        let valid_icon = ['X', 'x', 'O', 'o', '0'];

        if player_icon.chars().all(|c| valid_icon.contains(&c)){
            break;
        }
        else {
            println!("Falsche Eingabe! versuche es nochmal!!!")
        
        }
    }

    Player {
        name: player_name.to_string(),
        icon: player_icon.to_string(),
        first_player: true,
        highscore: 0,
    }
}


fn gameloop(player: &Player, a: [&str; 9]){
    println!("Spieler {:} ist am Zug", player.name);
    println!("Wähle dein Feld!!");
    let mut playmove = String::new();
    
    loop {
        io::stdin().read_line(&mut playmove)
            .expect("Konnte Feld nicht erkennen!!!!");
        let playmove = playmove.trim();

        let valid_fields = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];


        if playmove.chars().all(|c| valid_fields.contains(&c)) && a[playmove.parse::<usize>().unwrap() -1] != " "{
            break;
        }
        else {
            println!("Falsche Eingabe! versuche es nochmal!!!")
        
        }
        wincondition(a);


    }
fn wincondition(arr: [&str; 9]){

    let mut game = ["X", "X", "O", " ", " ", " ", " ", " ", " "];
    let mut win = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8], 
        [0, 4, 8], [2, 4, 6]
        ];

        for combo in win.iter() {
            let [a, b, c] = *combo;

            if game[a] != " " && arr[a] == game[b] && game[a] == game[c] {
                println!("Gewonnen");
            }
        }
    

        



}

//Player 1 am Zug
//gameloop
//wincondition
}


