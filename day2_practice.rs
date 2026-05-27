fn main() {
    let player_name = "Player 1";
    let mut player_health = 100; 
    
    println!("Welcome, {}!", player_name);
    println!("Your current health is: {}", player_health);
    
    // O jogador tomou um ataque! Vamos mudar o valor da vida:
    player_health = 75;
    
    println!("Oh no! You got hit! New health: {}", player_health);
}
