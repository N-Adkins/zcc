use comp_error::{CompError, ErrorCode};

mod comp_error;
mod lexer;

fn main() {
    let test_src = String::from("Test test testing testing!!");
    let error = CompError {
        code: ErrorCode::None,
        message: Some("Test message!".into()),
        line: Some((test_src, 1)),
        highlight: Some((5, 9)),
        highlight_message: Some("Test highlight".into()),
    };

    print!("{}", error);
}
