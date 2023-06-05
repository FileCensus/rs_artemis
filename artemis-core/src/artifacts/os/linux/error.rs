use std::fmt;

#[derive(Debug)]
pub(crate) enum LinuxArtifactError {
    Output,
    FilterOutput,
    BadToml,
    Serialize,
    Format,
    Cleanup,
    File,
    Process,
}

impl std::error::Error for LinuxArtifactError {}

impl fmt::Display for LinuxArtifactError {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinuxArtifactError::Output => write!(f, "Failed to output data"),
            LinuxArtifactError::FilterOutput => write!(f, "Failed to filter macos data"),
            LinuxArtifactError::BadToml => write!(f, "Artemis failed to parse TOML data"),
            LinuxArtifactError::Serialize => write!(f, "Artemis failed serialize artifact data"),
            LinuxArtifactError::Format => write!(f, "Unknown formatter provided"),
            LinuxArtifactError::Cleanup => write!(f, "Could not delete output data safely"),
            LinuxArtifactError::Process => write!(f, "Failed to parse Processes"),
            LinuxArtifactError::File => write!(f, "Failed to parse Files"),
        }
    }
}
