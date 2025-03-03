pub struct Record {
    sensor_id: String,
    data: Vec<u8>,
}

pub struct Sensor {
    sensor_id: String,
    enabled: bool,
    cfg: Option<String>,
}

pub struct Settings {
    key: String,
    value: String
}