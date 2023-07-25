fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..=100 {
        let response: String = ureq::get("http://127.0.0.1:8000/get")
            .call()?
            .into_string()?;
        println!("{i} {response}");
    }

    Ok(())
}
