use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_str = &args[1];
    // error checking
    if !file_str.contains(".qm"){
        println!("not a qm file: {}", file_str);
        return;
    }
    let path = Path::new(file_str);
    if !path.exists(){
        println!("File {} does not exist", file_str);
        return;
    }

    //get file contents
    let path = path.canonicalize().expect("Unable to create absolute path from file");
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let qm = contents;

    //*********parser***********

    let mut result = Vec::new();
    let mut last = 0;
    let mut line = 0;
    for (index, matched) in qm.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '_')) {
        if last != index {
            result.push(TextLoc{text: qm[last..index].to_string(), line: line, character: index as u32});
        }
        if matched != " " {
            result.push(TextLoc{text: matched.to_string(), line: line, character: index as u32});
        }
        if matched == "\n"{
            line+=1;
        }
        last = index + matched.len();
    }
    if last < qm.len() {
        result.push(TextLoc{text: qm[last..].to_string(), line: line, character: last as u32});
    }

    let tokens = result;
    
    //get rid of comments from token list

    let mut new_tokens = Vec::new();
    let mut include_flag = true;
    for tok in &tokens{
        let token = &tok.text;
        if token == ";"{
            include_flag = false;
            continue;
        }
        if token == "\n" && !include_flag {
            include_flag = true;
            continue;
        }
        if !include_flag{
            continue;
        }
        new_tokens.push(tok);
    }

    let tokens = new_tokens;
    

    let mut p_tokens = Vec::new();
    for token in tokens{
        let t = &token.text;
        p_tokens.push(Token{text: token.clone(), token: match t.as_str(){
            "(" => TokenT::OpenI,
            ")" => TokenT::CloseI,
            "[" => TokenT::OpenO,
            "]" => TokenT::CloseO,
            "{" => TokenT::OpenL,
            "}" => TokenT::CloseL,
            "&" => TokenT::And,
            "|" => TokenT::Or,
            "^" => TokenT::Xor,
            "!" => TokenT::Not,
            "-" => TokenT::Set,
            "*" => TokenT::Clock,
            "/" => TokenT::Rising,
            "\\" => TokenT::Falling,
            "." => TokenT::Cdot,
            "," => TokenT::Comma,
            _ => {
                if t.parse::<u64>().is_ok() {TokenT::Num} else {TokenT::Name}
            }
        }});
    }
    //finished tokenization


}
#[derive(Clone )]
struct TextLoc{
    text: String,
    line: u32,
    character: u32,
}
struct Token{
    text: TextLoc,
    token: TokenT,
}
#[derive(Debug, Clone, Copy)]
enum TokenT{
    Name,
    OpenI,
    CloseI,
    OpenO,
    CloseO,
    OpenL,
    CloseL,
    And,
    Or,
    Xor,
    Not,
    Set,
    Clock,
    Rising,
    Falling,
    Cdot, // chip marking dot
    Comma,
    Num,
}
