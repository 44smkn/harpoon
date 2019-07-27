fn main() {
    println!("Hello, harpoon!");

    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);
}
