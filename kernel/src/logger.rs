use conquer_once::spin::OnceCell;
use core::fmt::Write;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;
pub(crate) static LOGGER: OnceCell<SerialLogger> = OnceCell::uninit();

pub struct SerialLogger {
    port: Mutex<SerialPort>,
}

impl SerialLogger {
    fn new(port: SerialPort) -> Self {
        SerialLogger {
            port: Mutex::new(port),
        }
    }
}

impl log::Log for SerialLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut serial = self.port.lock();
            writeln!(
                serial,
                "[{:5} {}] {}",
                record.level(),
                record.target(),
                record.args()
            )
            .expect("Logging to serial")
        }
    }
}

pub(crate) fn init_logger() {
    let mut serial = unsafe { SerialPort::new(SERIAL_IO_PORT) };
    serial.init();
    let logger = LOGGER.get_or_init(move || SerialLogger::new(serial));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Logger initialised");
}
