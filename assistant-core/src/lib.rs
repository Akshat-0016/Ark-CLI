pub mod config;
pub mod memory;
pub mod router;
pub mod skills;
pub mod cache;
pub mod info_dump;
pub mod pdf;
pub mod file_index;
pub mod file_router;

pub async fn ask_engine(prompt: String, memory: &mut memory::Memory) -> String {
    router::route(&prompt, memory).await
}
