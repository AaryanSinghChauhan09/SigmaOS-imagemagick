//! Command-line options for imagemagick.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Convert mode
    Convert(String, String),
    /// Resize mode
    Resize(u32, u32),
    /// Batch mode
    Batch(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Input file
    pub input: String,
    /// Output file
    pub output: String,
    /// Quality
    pub quality: u8,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            input: String::new(),
            output: String::new(),
            quality: 90,
            verbose: false,
            quiet: false,
        }
    }
}
