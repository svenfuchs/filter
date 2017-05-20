// use std::env;
use std::io::{self};
use std::str;
// use std::process::Command;

const MASK: &'static str = "[secure]";
// const MIN_LEN: usize = 1;

trait Reader {
    fn read(&mut self) -> Option<String>;
}

struct Stdin<'a, R: 'a> where R: io::Read {
    input: &'a mut R
}

impl<'a, R: io::Read> Reader for Stdin<'a, R> {
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

struct Filter<'a, R: 'a> where R: Reader {
    input: &'a mut R,
    string: String,
    buffer: String
}

impl<'a, R: Reader> Reader for Filter<'a, R> where R: Reader {
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

impl<'a, R: 'a> Filter<'a, R> where R: Reader {
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

struct Runner<'a, R: 'a, W: 'a> where R: Reader, W: io::Write {
    input:  &'a mut R,
    output: &'a mut W
}

impl<'a, R: Reader, W: io::Write> Runner<'a, R, W> where R: Reader, W: io::Write {
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

// fn unescape(cmd: &String) -> String {
//     let out = Command::new("echo").arg(cmd).output().unwrap().stdout;
//     return str::from_utf8(&out).unwrap().trim_right().to_string()
// }
//
// fn var(key: &String) -> String {
//     env::var(key).unwrap_or("".to_string())
// }
//
// fn strs() -> Vec<String> {
//     let mut keys: Vec<String> = env::args().collect();
//     keys.remove(0);
//
//     let mut strs: Vec<String> = keys.iter().map(|s| var(s)).collect();
//     let mut escd: Vec<String> = strs.iter().map(|s| unescape(s)).collect();
//
//     strs.append(&mut escd);
//     strs.retain(|s| s.len() >= MIN_LEN);
//     strs.sort();
//     strs.dedup();
//     strs.sort_by_key(|s| s.len());
//     strs.reverse();
//
//     return strs;
// }

// fn filter(stdin: io::Read) -> Reader {
//     let stdin = Stdin { input: stdin };
//     return strs().iter().fold(stdin, |input, arg| {
//         Filter { input: input, string: arg.to_string(), buffer: "".to_string() }
//     });
// }

// fn runner<'a>(input: io::Read, output: io::Write) -> Runner<'a> {
//     return Runner { input: filter(input), output: output };
// }

// fn filters(input: &Reader, strs: Vec<String>) -> &Filter<Reader> {
//     // for s in strs() {
//     //     input = Filter { input: &mut input, string: s.to_string(), buffer: "".to_string() };
//     // }
//     // return input
// }

fn main() {
    let mut input  = Stdin { input: &mut io::stdin() };
    let mut filter = Filter { input: &mut input, string: "1".to_string(), buffer: "".to_string() };
    let mut runner = Runner { input: &mut filter, output: &mut io::stdout() };
    runner.run();

    // for s in strs() {
    //     input = Filter { input: &mut input, string: s.to_string(), buffer: "".to_string() };
    // }

    // let filter = strs().iter().fold(stdin, |input, s| {
    //     Filter { input: &mut input, string: s.to_string(), buffer: "".to_string() }
    // });

    // let runner = Runner { input: &mut input, output: &mut io::stdout() };
    // runner.run();

    println!("\nDone.")
}

#[cfg(test)]
mod tests {
    // http://stackoverflow.com/questions/28370126/how-can-i-test-stdin-and-stdout
    use Stdin;
    use Filter;
    use Runner;
    use std::io::Cursor;

    #[test]
    fn test() {
        let mut input  = Stdin  { input: &mut Cursor::new(&b"foo bar"[..]) };
        let mut filter = Filter { input: &mut input, string: "foo".to_string(), buffer: "".to_string() };
        let mut output = Cursor::new(Vec::new());
        let mut runner = Runner { input: &mut filter, output: &mut output };

        runner.run();

        let out = &runner.output;
        let s   = String::from_utf8(out.into_inner()).expect("Not UTF-8");

        assert_eq!("Who goes there?", s);
    }
}
