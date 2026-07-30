#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intent {
    Search,
    Bash,
    Memory,
    Task,
    Help,
    Unknown,
}
