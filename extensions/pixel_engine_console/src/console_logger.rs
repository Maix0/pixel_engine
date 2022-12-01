pub struct ConsoleLogger<const BUFFER_SIZE: usize, PASSTHROUGH: log::Log = SinkLogger> {
    passthrough: Box<PASSTHROUGH>,
    max_level: log::LevelFilter,
    inner_buffer: std::sync::Arc<std::sync::RwLock<heapless::Deque<String, BUFFER_SIZE>>>,
}

pub struct SinkLogger;

impl log::Log for SinkLogger {
    fn log(&self, _record: &log::Record) {}
    fn flush(&self) {}
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        false
    }
}
