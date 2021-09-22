use std::fmt;

pub enum BuilderError {
    BuildError(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuilderError::BuildError(str) => {
                write!(f, "Error occurred executing build(): {}", str)
            }
        }
    }
}

impl fmt::Debug for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let errstr = match self {
            BuilderError::BuildError(_) => "BuildError".to_string(),
        };
        write!(f, "{}: {}, line {}", errstr, file!(), line!())
    }
}
