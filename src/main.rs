fn main() {
    println!("Hello, harpoon!");

    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    let (stdout, stderr) = match std::process::Command::new("docker")
        .arg("inspect")
        .arg("1435d5fbf3c6")
        .output()
    {
        Ok(output) => (output.stdout, output.stderr),
        Err(e) => {
            println!("failed. cause is {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "stdout: {:?}\nstderr: {}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    );
}
