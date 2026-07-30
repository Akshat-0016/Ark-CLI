use assistant_core::{ask_engine, index::builder::IndexBuilder, memory::Memory};

#[tokio::main]
async fn main() {
    println!("Building search index...");
    let folder = "../info-dump";

    let artifacts = IndexBuilder::new().build_from_folder(folder);

    let index = artifacts.index;
    let documents = artifacts.documents;

    println!("Index ready.");
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

        let reply = ask_engine(input.to_string(), &index, &documents, &mut memory).await;

        println!("Ark: {}", reply);
    }
}
