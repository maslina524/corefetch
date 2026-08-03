pub mod colors;    // 14) Colors        : Display the terminal's 16-color palette
pub mod locale;    // 37) Locale        : Print system locale name
pub mod os;        // 47) OS            : Print the OS or Linux distribution name and version
pub mod processes; // 53) Processes     : Print number of running processes
pub mod version;   // 68) Version       : Print the Fastfetch version and build information
pub mod weather;   // 71) Weather       : Print weather information

pub use colors::Colors;
pub use locale::Locale;
pub use os::Os;
pub use processes::Processes;
pub use version::Version;
pub use weather::Weather;