#[cfg(test)]
use std::{thread, time::Duration};

use fast_log::{
    consts::LogSize,
    error::LogError,
    plugin::{file_split::RollingType, packer::LogPacker},
    Logger,
};

#[derive(Clone)]
pub struct Log<'a> {
    pub file_path: &'a str,
    pub temp_size: LogSize,
    pub chan_len: Option<usize>,
    pub roll_type: RollingType,
    pub packer: LogPacker,
}
impl Default for Log<'_> {
    fn default() -> Self {
        Self {
            file_path: "logs/out.log",
            temp_size: LogSize::MB(100),
            chan_len: Some(100000),
            roll_type: RollingType::All,
            packer: LogPacker {},
        }
    }
}

impl Log<'_> {
    pub fn init(&self) -> Result<&Logger, LogError> {
        fast_log::init(
            fast_log::Config::new()
                .console()
                .chan_len(self.chan_len)
                .file_split(
                    self.file_path,
                    self.temp_size,
                    self.roll_type,
                    self.packer.clone(),
                ),
        )
    }
}

#[test]
fn test_log() {
    Log::default().init().unwrap();
    log::info!("test log...");
    thread::sleep(Duration::from_secs(1));
}
