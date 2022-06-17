use crate::progress::{ProgressMessage};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Progress(ProgressMessage)
}
