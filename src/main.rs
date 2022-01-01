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
    println!("Parsed text.");
    let tokens = new_tokens;
    

    let mut p_tokens = Vec::new();
    for token in tokens{
        let t = &token.text;
        p_tokens.push(Token{tl: token.clone(), token: match t.as_str(){
            "\n" => TokenT::NewLine,
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
    println!("Tokenized text");
    let tokens = p_tokens;
    
    //INTERPRET
    run(tokens);
}




//*********Functions***********

fn run(tokens:Vec<Token>) -> bool{
    println!("hi1?");
    for i in 0..tokens.len(){
        let token = &tokens[i];
        if token.token == TokenT::Name && token.tl.text == "main"{
            if let Some(vals) = evaluate_chip(tokens, i as u32){
                for val in vals{
                    println!("{:#?}", val);
                }
            }
            break;
        }
    }


    true
}

fn evaluate_chip(tokens:Vec<Token>, start_i:u32) -> Option<Vec<Bit>>{
    let mut ret = Vec::new();
    let mut vars: Vec<Var> = Vec::new();
    let mut endI:u32 = tokens.len() as u32 -1;
    for i in start_i..tokens.len() as u32{
        if tokens[i as usize].token == TokenT::CloseL{
            endI = i;
            break;
        }
        return None;
    }
    let token = &tokens[(start_i+1) as usize];
    println!("starting function token: {}", token.tl.text);

    //to be run for inputs outputs and code
    fn run_group(token: &Token, tokens:&Vec<Token>, vars:&mut Vec<Var>, start_i:u32, tc:&mut u32)->bool{ // tc is token counter, modified for user
        match token.token{
            TokenT::OpenI =>{
                let mut i = (start_i+1) as usize;
                while tokens[i].token != TokenT::CloseI && i < tokens.len(){
                    let (varo, j) = get_var(&tokens, i, VarType::Input);
                    i = j;
                    match varo{
                        Some(var) =>{
                            vars.push(var);
                        }
                        None =>{
                            return false
                        }
                    }
                }
            }
            TokenT::OpenO =>{
                let mut i = (start_i+1) as usize;
                while tokens[i].token != TokenT::CloseI && i < tokens.len(){
                    let (varo, j) = get_var(&tokens, i, VarType::Output);
                    i = j;
                    match varo{
                        Some(var) =>{
                            vars.push(var);
                        }
                        None =>{
                            return false
                        }
                    }
                }
    
            }
            TokenT::OpenL =>{
    
            }
            _=>{
                    println!("Error - Unexpected token at line {}, char {}", token.tl.line, token.tl.character);
                    return false;
                }
        }
        true
    }
    let mut tc = start_i+1;
    if !run_group(&token, &tokens, &mut vars, tc, &mut tc){ // first run (preferably input but maybe output)
        return None;
    }
    for var in vars{
        println!("var: {}, type: {:#?}", var.name, var.t);
    }
    println!("hi?");

    Some(ret)
}

fn get_var(tokens:&Vec<Token>, start_i:usize, ty:VarType) -> (Option<Var>, usize){
    //later implement range variables
    match tokens[start_i].token {
        TokenT::Name =>
            (Some(Var{
                v:Bit::zero(),
                name: tokens[start_i].tl.text.to_string(),
                t:ty
            }), start_i+1),
        _ => {
            println!("Error, bad input variable naming convention: Line {}, char {}", tokens[start_i].tl.line, tokens[start_i].tl.character);
            (None, start_i+1)
        }
    }
}
        


//**********Structures*********

struct Var{
    v:Bit,
    name:String,
    t: VarType,
}
#[derive(Debug)]
enum VarType{
    Reg,
    Input,
    Output,
}
#[derive(Debug,Copy, Clone)]
struct Bit{
    b:bool
}
impl Bit{
    fn zero()->Bit{
        Bit{
            b:false
        }
    }
    fn one()->Bit{
        Bit{
            b:true
        }
    }
    fn from_b(val:bool) ->Bit{
        Bit{
            b:val
        }
    }
    fn from_str(val:String)->Bit{
        if val == "0" {Bit::from_b(false)} else {Bit::from_b(true)}
    }
    fn from_num(val:u32)->Bit{
        if val == 0 {Bit::from_b(false)} else {Bit::from_b(true)}
    }
}

#[derive(Clone)]
struct TextLoc{
    text: String,
    line: u32,
    character: u32,
}
struct Token{
    tl: TextLoc,
    token: TokenT,
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenT{
    NewLine,
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
