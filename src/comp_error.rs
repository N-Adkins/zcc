#[derive(Debug, Default, Clone, Copy)]
pub enum ErrorCode {
    #[default]
    None = 0,

    UnterminatedCharConstant = 1,
    UnterminatedStringLiteral = 2,
    UnterminatedHeaderName = 3,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::None => write!(f, "No error"),
            Self::UnterminatedCharConstant => {
                write!(f, "Failed to find end of a character constant")
            }
            Self::UnterminatedStringLiteral => write!(f, "Failed to find end of string literal"),
            Self::UnterminatedHeaderName => write!(f, "Failed to find end of header name"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CompError {
    pub code: ErrorCode,
    pub message: Option<String>,
    pub src: Option<(String, usize)>,      // actual line, line num
    pub highlight: Option<(usize, usize)>, // range
    pub highlight_message: Option<String>,
}

impl std::fmt::Display for CompError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Compilation Error [E{:04}]: {}",
            self.code as i32, self.code
        )?;

        // We need to leave a buffer on the left side
        // of the message for the line number. This will
        // calculate the size of that.
        let mut space_count = 0;
        if let Some((_, ref line_num)) = self.src {
            space_count = line_num.to_string().chars().count();
        }

        if let Some(ref message) = self.message {
            writeln!(f, "{:buffer$} | {}", "", message, buffer = space_count)?;
        }
        if let Some((ref src, ref line_num)) = self.src {
            writeln!(f, "{:buffer$} |", "", buffer = space_count)?;
            writeln!(
                f,
                "{} | {}",
                line_num,
                src.split('\n').nth(line_num - 1).unwrap()
            )?;
            if let Some((ref low, ref high)) = self.highlight {
                let highlight = high - low;
                writeln!(
                    f,
                    "{:buffer$} | {:low_buffer$}{:^>highlight_chars$}",
                    "",
                    "",
                    "",
                    buffer = space_count,
                    low_buffer = low,
                    highlight_chars = highlight
                )?;
                if let Some(ref message) = self.highlight_message {
                    writeln!(
                        f,
                        "{:buffer$} | {:low_buffer$}{}",
                        "",
                        "",
                        message,
                        buffer = space_count,
                        low_buffer = low
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct CompErrorBuilder {
    error: CompError,
}

impl CompErrorBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn code(mut self, code: ErrorCode) -> Self {
        self.error.code = code;
        self
    }

    pub fn message(mut self, msg: String) -> Self {
        self.error.message = Some(msg);
        self
    }

    pub fn source(mut self, src: String, line: usize) -> Self {
        self.error.src = Some((src, line));
        self
    }

    pub fn highlight(mut self, begin: usize, end: usize) -> Self {
        self.error.highlight = Some((begin, end));
        self
    }

    pub fn highlight_message(mut self, msg: String) -> Self {
        self.error.highlight_message = Some(msg);
        self
    }

    pub fn build(&self) -> Box<CompError> {
        Box::new(self.error.clone())
    }
}

pub type CompResult<T> = Result<T, Box<CompError>>;
