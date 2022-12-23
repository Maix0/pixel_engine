pub fn install(
    buffer_size: usize,
    pass_through: (&'static dyn log::Log, log::LevelFilter),
    line_size: usize,
) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(ConsoleLogger::new(
        buffer_size,
        line_size,
        pass_through,
    )))
    .map(|()| HAS_CONSOLE_LOGGER.store(true, std::sync::atomic::Ordering::SeqCst))
}
pub(crate) static HAS_CONSOLE_LOGGER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[derive(Clone)]
pub struct ConsoleLogger {
    pass_through: &'static dyn log::Log,
    line_size: usize,
    pub(crate) inner_buffer:
        std::sync::Arc<std::sync::RwLock<std::collections::VecDeque<LeveledMessage>>>,
}

impl std::fmt::Debug for ConsoleLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConsoleLogger")
            .field("line_size", &self.line_size)
            .field("inner_buffer", &self.inner_buffer)
            .field("pass_through", &"&'static dyn log::Log")
            .finish()
    }
}

impl ConsoleLogger {
    pub fn new(
        buffer_size: usize,
        line_size: usize,
        pass_through: (&'static dyn log::Log, log::LevelFilter),
    ) -> ConsoleLogger {
        log::set_max_level(std::cmp::max(pass_through.1, CONSOLE_LOG_LEVEL));
        Self {
            pass_through: pass_through.0,
            line_size,
            inner_buffer: std::sync::Arc::from(std::sync::RwLock::from(
                std::collections::VecDeque::with_capacity(buffer_size),
            )),
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct LeveledMessage {
    pub level: log::Level,
    pub message: String,
}

#[cfg(feature = "off")]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Off;
#[cfg(all(feature = "error", not(feature = "off")))]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Error;
#[cfg(all(feature = "warn", not(feature = "off")))]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Warn;
#[cfg(all(feature = "info", not(feature = "off")))]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;
#[cfg(all(feature = "debug", not(feature = "off")))]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;
#[cfg(all(feature = "trace", not(feature = "off")))]
const CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

impl log::Log for ConsoleLogger {
    fn log(&self, record: &log::Record) {
        if record.metadata().target() == "console" && record.level() <= CONSOLE_LOG_LEVEL {
            let message = record.args().to_string();
            let message_wrapped = textwrap::wrap(&message, self.line_size)
                .into_iter()
                .map(std::borrow::Cow::into_owned);
            let mut write_lock = self.inner_buffer.write().unwrap();
            for m in message_wrapped {
                if write_lock.len() + 1 > write_lock.capacity() {
                    write_lock.pop_front();
                }
                write_lock.push_front(LeveledMessage {
                    level: record.level(),
                    message: m,
                });
            }
        } else {
            self.pass_through.log(record);
        }
    }

    fn flush(&self) {}
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() >= CONSOLE_LOG_LEVEL || self.pass_through.enabled(metadata)
    }
}
