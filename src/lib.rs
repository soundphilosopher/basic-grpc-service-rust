pub mod sdk {
    pub mod basic {
        pub mod v1 {
            include!("sdk/basic.v1.rs");
        }
        pub mod service {
            pub mod v1 {
                include!("sdk/basic.service.v1.rs");
            }
        }
    }
    pub mod io {
        pub mod cloudevents {
            pub mod v1 {
                include!("sdk/io.cloudevents.v1.rs");
            }
        }
    }
}
pub mod talk;
pub mod utils;

pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("sdk/descriptor.bin");

#[macro_export]
macro_rules! info {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("[{}] {}", "o".blue().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! success {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("[{}] {}", "✓".green().bold(), std::format_args!($($arg)*));
  })
}

#[macro_export]
macro_rules! error {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("[{}] {}", "!".red().bold(), std::format_args!($($arg)*));
    std::process::exit(1);
  })
}

#[macro_export]
macro_rules! warning {
  ($($arg:tt)*) => ({
    use colored::Colorize;
    println!("[{}] {}", "!".yellow().bold(), std::format_args!($($arg)*));
  })
}
