#[derive(Debug, Clone)]
pub enum EntryMessage {
    Ok,
    Abort,

    InputChanged(String)
}
