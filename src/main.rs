use ferris_says::say;
use std::io::{stdout, stderr, BufWriter};
use std::env;
use std::process::Command;


fn main() {
    // Where we store the output of the command output
    let mut echo = Command::new("echo");

    // Pass though each argument to the command
    for arg in env::args().skip(1) {
        // get rid of automatic escaping of characters
        echo.arg(unescape(arg));
    }

    //println!("{:?}", echo.get_args());
    // Run the command
    let output = echo.output().expect("failed to execute!");

    // Command output
    if output.status.success() {
        let width = output.stdout.len();
        let stdout = stdout();

        let mut writer = BufWriter::new(stdout.lock());
        say(&output.stdout, width, &mut writer).unwrap();
    } else {
        let width = output.stderr.len();
        let stderr = stderr();

        let mut writer = BufWriter::new(stderr.lock());
        say(&output.stderr, width, &mut writer).unwrap();
    }
}

fn unescape(arg: String) -> String {

    let mut chars = arg.chars();
    let mut res = String::with_capacity(arg.len());

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                None => res.push('\\'),
                Some(c2) => {
                    match c2 {
                        'a' => res.push('\u{07}'),
                        'b' => res.push('\u{08}'),
                        'v' => res.push('\u{0B}'),
                        'f' => res.push('\u{0C}'),
                        'n' => res.push('\n'),
                        'r' => res.push('\r'),
                        't' => res.push('\t'),
                        'e' | 'E' => res.push('\u{1B}'),
                        '\\' => res.push('\\'),
                        '\'' => res.push('\''),
                        '"' => res.push('"'),
                        '$' => res.push('$'),
                        '`' => res.push('`'),
                        ' ' => res.push(' '),
                        _ => {
                            res.push('\\');
                            res.push(c2);
                        }
                    }
                }
            }
        } else {
            res.push(c)
        }
    }

    return res
}