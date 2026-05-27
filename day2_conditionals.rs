fn main() {
    let player_name = "Hero";
    let mut player_health = 40;
    let monster_damage = 50;

    println!("{} takes {} damage from a monster!", player_name, monster_damage);
    
    // Player takes damage (40 - 50 = -10, but let's set health to 0 to keep it simple)
    player_health = 0;

    // Here the computer makes a decision:
    if player_health <= 0 {
        println!("☠️ {} has died! GAME OVER.", player_name);
    } else {
        println!("❤️ {} survived! Current health: {}", player_name, player_health);
    }
}
