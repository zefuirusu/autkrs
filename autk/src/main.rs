use autk::cli::cmd::run_autk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to AUTK! current version 4.0.");
    run_autk();
    Ok(())
}
