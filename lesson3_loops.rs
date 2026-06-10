fn main() {
    let player_name = "Ranger";
    let max_arrows = 5;

    println!("🎯 {} activates Multi-Shot skill!", player_name);

    // This loop will count from 1 to 5 automatically
    for current_arrow in 1..=max_arrows {
        println!("🏹 Fired arrow number {}!", current_arrow);
    }

    println!("✨ Skill finished! All arrows fired.");
}
