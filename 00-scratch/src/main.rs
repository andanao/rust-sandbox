#[allow(unused)]
#[allow(dead_code)]
use std::error::Error;

#[allow(unused)]
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Hostname: {:?}",
        gethostname::gethostname().into_string().unwrap().chars().next().unwrap()
    );

    Ok(())
}
