use std::io;

fn main() {
    //intro();
    //game_field();
    game_loop();
}
fn intro() {

    let mut field = ["1","2","3","4","5","6","7","8","9"];

    println!("Willkommen im Tic Tac Toe Spiel in Rust!!!\n\nGebe deinen Spielernamen 1 ein!");
    let mut player1 = String::new();
    let mut player2 = String::new();

        io::stdin()
            .read_line(&mut player1)
            .expect("Konnte Namen nicht einlesen werden!");
        print!("Dein Spielername ist {}\n Gebe deinen Spielernamen 2 ein!", player1);
    
        io::stdin()
        .read_line(&mut player2)
        .expect("Konnte Namen nicht einlesen werden!");
    print!("Dein Spielername ist {}", player2);
    print!("Es spielt {player1} gegen {player2}");

}
fn game_loop(){

    println!("X ist am Zug!\nWähle ein Feld!");

    let mut field_choice = String::new();

    io::stdin()
            .read_line(&mut field_choice)
            .expect("Fehler beim einlesen des Feldes!");

    match field_choice.trim().parse::<i32>() {
        Ok(value) => {
            if (1..=9).contains(&value)  {
                
            } else {
                println!("Das Feld existiert nicht")
            }
        }
        Err(_) => {
            println!("Falsche Eingabe!")
        }
    }
}
fn game_field(field: str[]){

    println!("-------------\n| {} | {} | {} |\n-------------\n| {} | {} | {} |\n-------------\n| {} | {} | {} |\n-------------\n",
     field[0], field[1],field[2],field[3],field[4],field[5],field[6],field[7],field[8]);
}