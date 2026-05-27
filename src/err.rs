use std::fmt::Display;

#[derive(Debug)]
pub enum AllocErr {
    ArchitectureMismatch = 0,
    InsufficientPermissions = 1,
    BlockWriteFailure = 2,
    ProcessReolveError = 3,
}

impl Display for AllocErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocErr::ArchitectureMismatch => write!(
                f,
                "Architecture confict while trying to capture process list."
            ),
            AllocErr::InsufficientPermissions => {
                write!(f, "Cannot perform read/write with provided permissions.")
            }
            AllocErr::BlockWriteFailure => {
                write!(
                    f,
                    "Failed to commit memory at the provided block. (NULLPTR BASE ADDRESS)"
                )
            }
            AllocErr::ProcessReolveError => {
                write!(f, "Unable to resolve process ID.")
            }
        }
    }
}
