use sqlx::FromRow;

#[derive(Debug, PartialEq, Clone, FromRow)]
pub struct Record {
    pub id: u32,
    pub src_id: String,
    pub data: Vec<u8>,
    pub sent: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct Source {
    pub src_id: String,
    pub cfg: Option<String>,
    pub active: bool,
}
