use std::io::Write;

use std::io::{stdin, stdout};
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn ft_get_input() -> Option<String> {
    let mut stdout: termion::raw::RawTerminal<std::io::Stdout> = stdout().into_raw_mode().unwrap();
    let stdin: std::io::Stdin = stdin();

    let mut input: String = String::new();
    let mut cursor_pos: usize = 0;

    write!(stdout, "\r> ").unwrap();
    stdout.flush().unwrap();

    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('\n') => {
                write!(stdout, "\n").unwrap();
                stdout.flush().unwrap();
                break;
            }
            Key::Char(c) => {
                input.insert(cursor_pos, c);
                cursor_pos += 1;
            }
            Key::Backspace => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    input.remove(cursor_pos);
                }
            }
            Key::Left => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                }
            }
            Key::Right => {
                if cursor_pos < input.len() {
                    cursor_pos += 1;
                }
            }
            _ => {}
        }
        write!(
            stdout,
            "\r> {}{}",
            input,
            clear::UntilNewline
        ).unwrap();
        if cursor_pos < input.len() {
            write!(
                stdout,
                "{}",
                cursor::Left((input.len() - cursor_pos) as u16)
            )
            .unwrap();
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "\r{}", cursor::Show).unwrap();
    stdout.flush().unwrap();

    Some(input)
}