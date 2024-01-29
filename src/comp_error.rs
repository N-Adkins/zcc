#[derive(Debug, Default, Clone, Copy)]
pub enum ErrorCode {
    #[default]
    None = 0,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ErrorCode::None => write!(f, "No Error"),
        }
    }
}

#[derive(Debug, Default)]
pub struct CompError {
    pub code: ErrorCode,
    pub message: Option<&'static str>,
    pub src: Option<(String, usize)>,      // actual line, line num
    pub highlight: Option<(usize, usize)>, // range
    pub highlight_message: Option<&'static str>,
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

pub type CompResult<T> = Result<T, Box<CompError>>;
