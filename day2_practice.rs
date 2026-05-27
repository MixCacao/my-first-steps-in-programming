fn main() {
    let enemy_name = "Goblin";
    let mut enemy_health = 50;
    let sword_damage = 15;

    println!("A wild {} appears with {} HP!", enemy_name, enemy_health);

    // First attack
    println!("You slash the {} with your sword for {} damage!", enemy_name, sword_damage);
    enemy_health = enemy_health - sword_damage;
    println!("{} health is now: {}", enemy_name, enemy_health);

    // Second attack
    println!("You hit the {} again for {} damage!", enemy_name, sword_damage);
    enemy_health = enemy_health - sword_damage;
    println!("{} health is now: {}", enemy_name, enemy_health);
}
