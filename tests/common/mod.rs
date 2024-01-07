use logfather::Logger;

const TEST_LOG: &str = "tests/it_log.txt";

pub fn setup() -> Logger {
    // some setup code, like creating required files/directories, starting
    // servers, etc.

    let mut logger = Logger::new();
    logger.file(true);
    logger.path(&TEST_LOG);

    logger
}