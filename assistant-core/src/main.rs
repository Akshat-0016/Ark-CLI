use assistant_core::ask_engine;
use assistant_core::memory::Memory;

#[tokio::main]
async fn main() {
    println!("Ark CLI — type something:");
    let mut memory = Memory::new();

    loop {
        let mut input = String::new();
        let bytes = std::io::stdin().read_line(&mut input).unwrap();

        if bytes == 0 {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let reply = ask_engine(input.to_string(), &mut memory).await;
        println!("Ark: {}", reply);
    }
}
