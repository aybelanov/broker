#[derive(Debug)]
pub struct Record {
    id: u32,
    src: String,
    data: Vec<u8>,
    sent: bool
}

#[derive(Debug)]
pub struct Source {
    sensor_id: String,
    cfg: Option<String>,
    on: bool,
}

#[derive(Debug)]
pub struct Setting {
    key: String,
    value: String
}