// This is our custom function! It handles the healing logic.
fn drink_potion(current_health: i32, potion_strength: i32) -> i32 {
    let new_health = current_health + potion_strength;
    println!("🧪 Gulp... You drank a potion and recovered {} HP!", potion_strength);
    return new_health;
}

fn main() {
    let player_name = "Knight";
    let mut knight_health = 45;
    let potion_heal_amount = 30;

    println!("{} currently has {} HP.", player_name, knight_health);

    // Instead of doing math here, we use (or "call") our function!
    knight_health = drink_potion(knight_health, potion_heal_amount);

    println!("{} now has {} HP!", player_name, knight_health);
}
