use std::env;
use std::fs;
use std::fmt;
use std::str::from_utf8;

struct EventCodeLine {
    int_fields: Vec<i32>,
    str_fields: Vec<String>,
    indent: u8,
}

struct CommonEvent {
    name: String,
    memo: String,
    code: Vec<EventCodeLine>,
}

impl fmt::Display for EventCodeLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for _ in 0..self.indent {
            s += "    ";
        }
        
        s += "(";
        for &e in &self.int_fields {
            s += &e.to_string();
            s += ",";
        }
        s += ")(";

        for e in &self.str_fields {
            s += "\"";
            s += &e;
            s += "\"";
            s += ",";
        }
        s += ")";
        write!(f, "{}", s)
    }
}

impl fmt::Display for CommonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut code_string = String::new();
        for l in &self.code {
            code_string += "    ";
            code_string += &l.to_string();
            code_string += "\n";
        }
        write!(f, "{}\n{}\n{}", self.memo, self.name, code_string)
    }
}

fn parse_int(byte_iter: &mut std::slice::Iter<u8>) -> i32 {
    let mut val: [u8; 4] = [0; 4];
    for i in 0..4 {
        val[i] =
            *byte_iter.next()
            .expect("int should have four bytes");
    }
    i32::from_le_bytes(val)
}

fn parse_str(byte_iter: &mut std::slice::Iter<u8>) -> String {
    let len = parse_int(byte_iter);
    let mut s: Vec<u8> = vec![];
    for _ in 0..len-1 {
        s.push(
            *byte_iter.next()
            .expect("number of bytes in string should match declared length")
        );
    }
    byte_iter.next(); // skip the null terminator

    from_utf8(&s)
        .expect("string should be valid utf8")
        .replace("\r", "") // some comment lines mysteriously seem to have carriage returns at the end?
}

fn parse_line(byte_iter: &mut std::slice::Iter<u8>) -> EventCodeLine {
    let num_ints = *byte_iter.next().unwrap();
    let mut int_fields = vec![];
    for _ in 0..num_ints {
        int_fields.push(parse_int(byte_iter));
    }
    
    let indent = *byte_iter.next().unwrap();

    let num_strs = *byte_iter.next().unwrap();
    let mut str_fields = vec![];
    for _ in 0..num_strs {
        str_fields.push(parse_str(byte_iter));
    }
    byte_iter.next(); // null pad

    EventCodeLine {
        int_fields,
        str_fields,
        indent
    }
}

fn parse_cev(byte_iter: &mut std::slice::Iter<u8>) -> CommonEvent {
    for _ in 0..16 {
        byte_iter.next();
    }
    let name = parse_str(byte_iter);
    let line_count = parse_int(byte_iter);
    let mut code = vec![];
    for _ in 0..line_count {
        code.push(parse_line(byte_iter));
    }

    parse_str(byte_iter); // mystery string

    let memo = parse_str(byte_iter);

    // unused stuff
    byte_iter.next();
    for _ in 0..parse_int(byte_iter) {
        parse_str(byte_iter);
    }
    for _ in 0..parse_int(byte_iter) {
        byte_iter.next();
    }
    for _ in 0..parse_int(byte_iter) {
        for _ in 0..parse_int(byte_iter) {
            parse_str(byte_iter);
        }
    }
    for _ in 0..parse_int(byte_iter) {
        for _ in 0..parse_int(byte_iter) {
            parse_int(byte_iter);
        }
    }

    for _ in 0..parse_int(byte_iter) {
        parse_int(byte_iter);
    }

    byte_iter.next();
    parse_int(byte_iter);
    for _ in 0..100 {
        parse_str(byte_iter);
    }
    byte_iter.next();
    parse_str(byte_iter); // mystery string
    byte_iter.next();

    parse_str(byte_iter);
    parse_int(byte_iter);
    byte_iter.next();

    CommonEvent {
        name,
        memo,
        code
    }
}

fn textconv(file_path: &String) -> String {
    let contents = fs::read(file_path)
        .expect("failed to read file");

    let mut byte_iter = contents.iter();

    // ignore header info
    if file_path.contains("CommonEvent.dat") {
        for _ in 0..11 {
            byte_iter.next();
        }
    } else if file_path.contains(".common") {
        for _ in 0..4 {
            byte_iter.next();
        }
    } else {
        panic!("could not detect file type (expected file path to contain either \"CommonEvent.dat\" or \".common\"");
    }

    let num_cmns = parse_int(&mut byte_iter);
    
    let mut s = String::new();
    for _ in 0..num_cmns {
        s += &parse_cev(&mut byte_iter).to_string();
        s += "\n\n";
    }

    s
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("no file supplied");
        return;
    }
    let file_path = &args[1];

    let output = textconv(&file_path);
    print!("{output}");
}
