// [INCORRECT] ตัวอย่าง Trait ที่ละเมิดกฎ Object Safety:
// pub trait UnsafeObjectTrait {
//     fn generic_method<U>(&self, data: U); // ผิดกฎ: มี Generic Parameter
//     fn clone_self(&self) -> Self;         // ผิดกฎ: คืนค่าเป็น Self
// }
// let obj: Box<dyn UnsafeObjectTrait>;     // ERROR: cannot be made into an object

// [CORRECT] Trait ที่ผ่านเกณฑ์ Object Safety:
pub trait SafeLogger {
    fn log(&self, message: &str);
    fn level(&self) -> &'static str;
}

pub struct ConsoleLogger;

impl SafeLogger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[CONSOLE - {}] {}", self.level(), message);
    }

    fn level(&self) -> &'static str {
        "INFO"
    }
}

pub struct FileLogger {
    pub file_name: String,
}

impl SafeLogger for FileLogger {
    fn log(&self, message: &str) {
        println!("[FILE: {} - {}] {}", self.file_name, self.level(), message);
    }

    fn level(&self) -> &'static str {
        "DEBUG"
    }
}

fn main() {
    let loggers: Vec<Box<dyn SafeLogger>> = vec![
        Box::new(ConsoleLogger),
        Box::new(FileLogger {
            file_name: "app.log".to_string(),
        }),
    ];

    for logger in &loggers {
        logger.log("System initialized successfully.");
    }
}
