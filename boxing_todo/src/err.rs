use std::fmt;
use std::error::Error; // Keep this import

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

// Implement the Display trait for ParseErr
impl fmt::Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErr::Empty => write!(f, "Fail to parse todo"),
            ParseErr::Malformed(_) => write!(f, "Fail to parse todo"),
        }
    }
}

// Implement the Error trait for ParseErr
impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseErr::Malformed(err) => Some(&**err),
            ParseErr::Empty => None,
        }
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

// Implement the Display trait for ReadErr
impl fmt::Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

// Implement the Error trait for ReadErr
impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.child_err)
    }
}
