use std::collections::HashMap;
use std::fs;
use std::fmt;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParsingError {
    message: String,
    line: u32,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ParsingError {
    pub fn new(message: String, line: u32) -> ParsingError {
        ParsingError{
            message: message,
            line: line,
        }
    }
}

struct Keyword {
    name: &'static str,
    process: fn(&str, &mut Context) -> Result<String, ParsingError>,
}

pub struct Context {
    macros: HashMap<String, String>,
    line_num: u32,

    // Compiling regex is slow. Only do it once per context
    word_re: Regex,
    block_comment_re: Regex,
    eol_comment_re: Regex,
}

impl Context {
    pub fn new() -> Context {
        Context{
            macros: HashMap::new(),
            line_num: 0,

            word_re: Regex::new(r"\b\w+\b").unwrap(),
            block_comment_re: Regex::new(r"(?s)/\*.*\*/").unwrap(),
            eol_comment_re: Regex::new(r"//.*$").unwrap(),
        }
    }
}

const KEYWORDS: &[Keyword] = &[
    Keyword {
        name: "#define",
        process: process_define,
    },
    Keyword {
        name: "#include",
        process: process_include,
    },
];

fn process_define(value: &str, ctx: &mut Context) -> Result<String, ParsingError> {
    let mut parts = value.split_whitespace();
    let key = match parts.next() {
        Some(k) => k,
        None => return Err(ParsingError::new(
            String::from("#define has no name"),
            ctx.line_num)
        ),
    };

    let val = match parts.next() {
        Some(v) => v,
        None => return Err(ParsingError::new(
            format!("#define {} has no value", key),
            ctx.line_num)
        ),

    };

    ctx.macros.insert(key.to_string(), val.trim().to_string());
    Ok(String::new())
}

fn process_include(value: &str, ctx: &mut Context) -> Result<String, ParsingError> {
    let chars: &[_] = &['"', '<', '>'];
    let filename = value.trim_matches(chars);
    let tmp_line = ctx.line_num;
    ctx.line_num = 0;
    let out = process_file(filename, ctx);
    ctx.line_num = tmp_line;

    out
}

fn evaluate_macros(line: &str, ctx: &Context) -> String {
    let mut last = 0;
    let mut out = String::new();
    for m in ctx.word_re.find_iter(line) {
        // Push everything leading up the the matched word
        out.push_str(&line[last..m.start()]);

        // Push matched word or replacement if one was found.
        let word = &line[m.start()..m.end()];

        // Recursively try to replace word. This allows macro definitions like the
        // following:
        // #define A 1
        // #define B (A + 2)
        let out_word = match ctx.macros.get(word) {
            Some(x) => evaluate_macros(x, ctx),
            None => word.to_string()
        };
        out.push_str(out_word.as_str());
        last = m.end();
    }

    // Push everything remaining in line after the last matched word.
    out.push_str(&line[last..]);

    return out;
}

fn process_line(line: &str, ctx: &mut Context) -> Result<String, ParsingError> {
    // Trim whitespace
    let trimmed = line.trim();

    if trimmed.starts_with("#") {
        // For lines with keywords, remove comments
        let replaced = ctx.eol_comment_re.replace_all(trimmed, "");

        let mut parts = replaced.splitn(2, ' ');
        let keyword_name = match parts.next() {
            Some(v) => v,
            None => return Err(ParsingError::new(
                String::from("Found no directive"),
                ctx.line_num)),
        };

        let value = match parts.next() {
            Some(v) => v,
            None => return Err(ParsingError::new(
                String::from("Unknown value for directive"),
                ctx.line_num
            )),
        };

        let kw = match KEYWORDS
            .iter()
            .find(|keyword| keyword.name == keyword_name) {
                Some(k) => k,
                None => return Err(ParsingError::new(
                    format!("Unknown keyword: {}", keyword_name),
                    ctx.line_num)
                ),
            };

        return (kw.process)(value, ctx);
    } else {
        // For normal lines, do macro replacements. This finds all whole words
        // in the line and then checks the macro dictionary for a replacement.
        Ok(evaluate_macros(line, ctx) + "\n")
    }
}

pub fn process_str(s: &str, ctx: &mut Context) -> Result<String, ParsingError> {
    // Remove all block comments, we won't need those where we're going.
    // TODO: This messes up the line numbers for error reporting. Sad day.
    let no_comments = ctx.block_comment_re.replace_all(&s, "");

    let mut out = String::new();
    for line in no_comments.lines() {
        ctx.line_num += 1;
        match process_line(line, ctx) {
            Ok(l) => out.push_str(&l),
            Err(e) => return Err(e),
        }
    }

    Ok(out)
}

pub fn process_file(filename: &str, ctx: &mut Context) -> Result<String, ParsingError> {
    match fs::read_to_string(filename) {
        Ok(s) => return process_str(&s, ctx),
        Err(e) => return Err(ParsingError::new(
            format!("Error opening file {}: {}", filename, e), 0)
        ),
    };
}
