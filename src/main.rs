fn main() {
    println!("Hello, harpoon!");

    let args = std::env::args().collect::<Vec<String>>();

    let (stdout, stderr) = match std::process::Command::new("docker")
        .arg("inspect")
        .arg(&args[1])
        .output()
    {
        Ok(output) => (output.stdout, output.stderr),
        Err(e) => {
            println!("failed. cause is {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    );
}
