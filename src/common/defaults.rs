pub const CFG_FILE_PATH: &str = "config.json";
pub const DB_FILE_PATH: &str = "broker.db";

// default setting key's section
pub const DATA_FLOW_RECONNECT_DELAY_KEY: &str = "data_flow_reconnect_delay";
pub const PACKET_SIZE_KEY: &str               = "packet_size";
pub const MAX_COUNT_DATA_ROWS_KEY: &str       = "max_count_data_rows";
pub const CLEAR_DATA_DELAY_KEY: &str          = "clear_data_delay";
pub const BROKER_CONFIGURATION_KEY: &str      = "broker_configuration";
pub const DESCRIPTION_KEY: &str               = "description";
pub const MODIFIED_TICKS_KEY: &str            = "modified_ticks";
pub const DATA_SENDING_DELAY_KEY: &str        = "data_sending_delay";
pub const VIDEO_SEGMENTS_EXPIRATION_KEY: &str = "video_segments_expiration";
pub const SETTING_VALUES: [(&str, &str); 9] = [
    (BROKER_CONFIGURATION_KEY, "{}"),
    (DATA_FLOW_RECONNECT_DELAY_KEY, "10000"),
    (DATA_SENDING_DELAY_KEY, "1000"),
    (MODIFIED_TICKS_KEY, "0"),
    (DESCRIPTION_KEY, "Embedded broker"),
    (MAX_COUNT_DATA_ROWS_KEY, "1000000"),
    (CLEAR_DATA_DELAY_KEY, "3600"),
    (PACKET_SIZE_KEY, "1000"),
    (VIDEO_SEGMENTS_EXPIRATION_KEY, "72"),
];