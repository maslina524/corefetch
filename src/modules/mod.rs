pub mod locale;    // 37) Locale        : Print system locale name
pub mod processes; // 53) Processes     : Print number of running processes
pub mod version;   // 68) Version       : Print the Fastfetch version and build information

pub use locale::Locale;
pub use processes::Processes;
pub use version::Version;