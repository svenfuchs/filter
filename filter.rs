use std::env;
use std::io::{self, Read, Write};
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

struct Runner {
    input:  Box<(Reader)>,
    output: Box<io::Write>
}

impl Runner {
    fn run(&mut self) {
        while let Some(chars) = self.input.read() {
            self.write(chars);
        }
    }

    fn write(&mut self, chars: String) {
        self.output.write(&chars.into_bytes()).unwrap();
        self.output.flush().unwrap();
    }
}

fn unescape(cmd: &String) -> String {
    let out = Command::new("echo").arg(cmd).output().unwrap().stdout;
    return str::from_utf8(&out).unwrap().trim_right().to_string()
}

fn var(key: &String) -> String {
    env::var(key).unwrap_or("".to_string())
}

fn strs() -> Vec<String> {
    let mut keys: Vec<String> = env::args().collect();
    keys.remove(0);

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

fn filter(stdin: Box<io::Read>) -> Box<Reader> {
    let stdin: Box<Reader> = Box::new(Stdin { input: stdin });
    return strs().iter().fold(stdin, |input, arg| {
        Box::new(Filter { input: input, string: arg.to_string(), buffer: "".to_string() })
    });
}

fn runner(input: Box<io::Read>, output: Box<io::Write>) -> Runner {
    return Runner { input: filter(input), output: output };
}

fn main() {
    let input  = Box::new(io::stdin());
    let output = Box::new(io::stdout());

    runner(input, output).run();

    println!("\n\nDone.")
}

#[cfg(test)]
mod test {
    // http://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
    use std::io::{self, Cursor, Read, Write, Seek, SeekFrom};
    use runner;

    #[test]
    fn test_foo() {
        let mut input  = Box::new(Cursor::new(&b"one two"[..]));
        let mut cursor = Cursor::new(Vec::new());
        let mut bx     = Box::new(cursor);
        let mut runner = runner(input, bx);

        runner.run();

        let mut string = "".to_string();
        // let mut cursor = unsafe { &*Box::into_raw(bx) };
        // let mut cursor = Box::borrow(bx);
        Cursor::seek(&mut cursor, SeekFrom::Start(0));
        Cursor::read_to_string(&mut cursor, &mut string);

        println!("{}", string);
    }
}
