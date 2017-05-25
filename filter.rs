use std::env;
use std::io::{self, Read};
use std::str;
use std::process::Command;

const MASK: &'static str = "[secure]";
const MIN_LEN: usize = 1;

trait Reader {
    fn read(&mut self) -> Option<String>;
}

struct Stdin {
    input: Box<std::io::Read>
}

impl Reader for Stdin {
    fn read(&mut self) -> Option<String> {
        let mut buffer = [0];
        let result = self.input.read(&mut buffer);

        if result.unwrap() == 0 {
            return None;
        } else {
            return Some(str::from_utf8(&buffer).unwrap().to_string());
        }
    }
}

struct Filter {
    input: Box<(Reader)>,
    string: String,
    buffer: String
}

impl Reader for Filter {
    fn read(&mut self) -> Option<String> {
        match self.input.read() {
            Some(c) => {
                self.buffer.push_str(&c);
                return Some(self.eval())
            },
            None => return None
        }
    }
}

impl Filter {
    fn eval(&mut self) -> String {
        if self.string == self.buffer {
            return self.mask();
        } else if !self.string.starts_with(&self.buffer) {
            return self.flush();
        } else {
            return "".to_string();
        }
    }

    fn mask(&mut self) -> String {
        self.buffer.clear();
        return MASK.to_string();
    }

    fn flush(&mut self) -> String {
        let str = self.buffer.clone();
        self.buffer.clear();
        return str
    }
}

fn unescape(cmd: &String) -> String {
    let out = Command::new("echo").arg(cmd).output().unwrap().stdout;
    return str::from_utf8(&out).unwrap().trim_right().to_string()
}

fn var(key: &String) -> String {
    env::var(key).unwrap_or("".to_string())
}

fn vars(keys: Vec<String>) -> Vec<String> {
    let mut strs: Vec<String> = keys.iter().map(|s| var(s)).collect();
    let mut escd: Vec<String> = strs.iter().map(|s| unescape(s)).collect();

    strs.append(&mut escd);
    strs.retain(|s| s.len() >= MIN_LEN);
    strs.sort();
    strs.dedup();
    strs.sort_by_key(|s| s.len());
    strs.reverse();

    return strs;
}

fn filter(stdin: Box<io::Read>, args: Vec<String>) -> Box<Reader> {
    let stdin: Box<Reader> = Box::new(Stdin { input: stdin });
    return vars(args).iter().fold(stdin, |input, arg| {
        Box::new(Filter { input: input, string: arg.to_string(), buffer: "".to_string() })
    });
}

fn run<'a>(input: Box<io::Read>, output: &'a mut io::Write, args: Vec<String>) {
    let mut filter = filter(input, args);
    while let Some(chars) = filter.read() {
        output.write(&chars.into_bytes()).unwrap();
        output.flush().unwrap();
    }
}

fn args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    return args;
}

fn main() {
    run(Box::new(io::stdin()), &mut io::stdout(), args());
}

#[cfg(test)]
mod test {
    use run;
    use std::io::Cursor;
    use std::env;

    #[test]
    fn test_foo() {
        env::set_var("a", "two");
        let args = vec!["a".to_string()];

        // http://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
        let input = Box::new(Cursor::new(&b"one two three"[..]));
        let mut output = Cursor::new(Vec::new());

        run(input, &mut output, args);

        let stdout = String::from_utf8(output.into_inner()).expect("Not UTF-8");
        assert_eq!("one [secure] three", stdout);
    }
}
