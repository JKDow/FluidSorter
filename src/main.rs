
fn main() {
    println!("How many vials are there?");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num_vials: usize = buf.trim().parse().unwrap();
    buf.clear();

    println!("What is the capacity of the vials?");
    std::io::stdin().read_line(&mut buf).unwrap();
    let capacity: usize = buf.trim().parse().unwrap();
    buf.clear();

    println!("Please enter the vial string");
    std::io::stdin().read_line(&mut buf).unwrap();

    let game = fluid_sort::game::Game::new(&buf, num_vials, capacity).unwrap();
    game.print_state();
}
