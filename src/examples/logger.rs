use std::fmt::Arguments;

trait Logger {
    fn print(&mut self, value: &Arguments<'_>);
}

struct BasicLogger;

impl Logger for BasicLogger {
    fn print(&mut self, value: &Arguments<'_>) {
        println!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestLogger(Vec<String>);

    impl Logger for TestLogger {
        fn print(&mut self, value: &Arguments<'_>) {
            self.0.push(value.to_string());
        }
    }

    fn log_hello(logger: &mut dyn Logger) {
        logger.print(&format_args!("{}", "hello"));
    }

    fn log_goodbye(logger: &mut dyn Logger) {
        logger.print(&format_args!("{}", "goodbye"));
    }

    #[test]
    fn logs() {
        let mut logger = TestLogger::default();
        log_hello(&mut logger);
        assert_eq!(logger.0[0], "hello");
        log_goodbye(&mut logger);
        assert_eq!(logger.0[1], "goodbye");
    }
}