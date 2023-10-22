use crate::primitive::PrimitiveCollection;
use crate::primitive::Primitive;
use crate::primitive::PrimitiveType;
// use std::collections::HashMap;


const  KEYWORD_ARRAY: [&str; 12] = ["unsigned", "char", "short", "int", "long", "float", "double",
"while", "if", "return", "void", "main"];
// static  keywords_hashmap = HashMap::new();

// Token types
#[derive( Clone)]
pub enum TokenType {
    IntConstant, 
    FloatConstant,
    Keyword, // unsigned, char, short, int, long, float, double, while, if, return, main, void
    Operator, // (,), {,}, =,==,<,>,<=,>=,!=,+,-,*,/,;
    Identifier,
    Invalid,
    Space,
    NewLine,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntConstant => write!(f, "IntConstant"),
            Self::FloatConstant => write!(f, "FloatConstant"),
            Self::Keyword => write!(f, "Keyword"),
            Self::Operator => write!(f, "Operator"),
            Self::Identifier => write!(f, "Identifier"),
            Self::Invalid => write!(f, "Invalid"),
            Self::Space => write!(f, "Space"),
            Self::NewLine => write!(f, "NewLine"),
        }
    }
}
#[derive( Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub line_num: i32,
    pub char_pos: i32,
}

impl Token {
    fn new(text_input: String, token_type_input: TokenType, line_num_input: i32, char_pos_input: i32) -> Self{
        Self {
            text: text_input,
            token_type: token_type_input,
            line_num: line_num_input,
            char_pos: char_pos_input,
        }
    }

    pub fn print(&self){
        println!("Token '{}' is type {} on line {} at position {}", self.text, self.token_type, self.line_num, self.char_pos);
    }
    pub fn error(&self) -> String{
        return format!("Error at Line {} position {}", self.line_num.to_string() , self.char_pos)
    }
    // test me
    pub fn is_constant(&self) -> bool {
        match self.token_type {
            TokenType::IntConstant | TokenType::FloatConstant => true,
            _ => false
        }
    }
}

pub struct Scanner {
    all_tokens: Vec<Token>,
    primitve_collection: PrimitiveCollection,
    line_num: i32,
    char_pos: i32,
    token_idx: i32,
}

impl Scanner {

    pub fn new(prim_coll: PrimitiveCollection) -> Self{
        Self {
            all_tokens: Vec::new(),
            primitve_collection: prim_coll,
            line_num: -1,
            char_pos: -1,
            token_idx: -1,
        }
    }

    pub fn get_next_token(&mut self) -> &Token{
        // 
        self.token_idx += 1;
        &self.all_tokens[self.token_idx as usize]
    }

    pub fn more_tokens_available(&self) -> bool{
        //
        self.token_idx + 1 < (self.all_tokens.len() as i32)
    }

    pub fn get_all_tokens(&self) -> Vec<Token> {
        return self.all_tokens.to_vec();
    }
    
    pub fn tokenize_prims(&mut self){
        self.line_num += 1;
        self.char_pos += 1;
        while self.primitve_collection.more_available(){
            let p = self.primitve_collection.get_next_prim();
            // p.print_prim();
            match p.prim_type {
                PrimitiveType::Alpha | PrimitiveType::UnderScore => {
                    self.identifier();
                    // self.all_tokens.push(self.identifier);
                },
                PrimitiveType::Operator => {
                    self.operator();
                },
                PrimitiveType::Digit | PrimitiveType::Negative => {
                    self.constant();
                },
                PrimitiveType::NewLine => {
                    self.line_num += 1;
                    self.char_pos = 0;
                    continue;
                },
                PrimitiveType::Space => {
                    self.space();
                    continue;
                },
                PrimitiveType::Eof => {
                    continue;
                },
                _ => {
                    self.invalid();
                    continue;
                }
            }
            // if p.prim_type == PrimitiveType::Alpha || p.prim_type == PrimitiveType::Special {
            //     self.identifier();
            // }
        }
        // self.print_tokens();
    }

    pub fn print_tokens(&self){
        for tok in &self.all_tokens  {
            tok.print();
        }
    }

    fn identifier(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
        // Create an empty string 
        let mut new_tok_text = String::new();
        // Grab the inital primitive
        let mut pr = self.primitve_collection.get_curr_prim();
        // First rule -> special (_) or alpha
        new_tok_text.push(pr.symbol);
        self.char_pos += 1;
        // Optional second rule (Alpha,Special,Digit)
        loop {
            match self.primitve_collection.peek_next_prim().prim_type {
                PrimitiveType::Alpha | PrimitiveType::UnderScore | PrimitiveType::Digit => {
                    pr = self.primitve_collection.get_next_prim();
                    new_tok_text.push(pr.symbol);
                    self.char_pos += 1;
                    continue;
                },
                _ => {
                    break;
                }
            }
        }
        if is_keyword(&new_tok_text){   
            self.all_tokens.push(Token::new(new_tok_text, TokenType::Keyword, curr_line_num, curr_char_pos));
        }
        else {
            self.all_tokens.push(Token::new(new_tok_text, TokenType::Identifier, curr_line_num, curr_char_pos));
        }
        
    }

    fn operator(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
         // Create an empty string 
         let mut new_tok_text = String::new();
         // Grab the inital primitive
         let mut pr = self.primitve_collection.get_curr_prim();
         // First rule -> Operator
         new_tok_text.push(pr.symbol);
         self.char_pos += 1;
         // Optional second rule 
         match self.primitve_collection.peek_next_prim().symbol {
            '=' => {
                pr = self.primitve_collection.get_next_prim();
                new_tok_text.push(pr.symbol);
                self.char_pos += 1;
            }
            _ => {
               ();
            }
         }
         self.all_tokens.push(Token::new(new_tok_text, TokenType::Operator, curr_line_num, curr_char_pos));
    }

    fn constant(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
        // Create an empty string 
        let mut new_tok_text = String::new();
        // Grab the inital primitive
        let mut pr = self.primitve_collection.get_curr_prim();
        // First rule -> Operator
        new_tok_text.push(pr.symbol);
        // Optional second rule 
        loop {
            match self.primitve_collection.peek_next_prim().prim_type {
                PrimitiveType::Digit | PrimitiveType::Dot => {
                    pr = self.primitve_collection.get_next_prim();
                    new_tok_text.push(pr.symbol);
                    self.char_pos += 1;
                    continue;
                },
                _ => {
                    break;
                }
            }
        }
        self.all_tokens.push(Token::new(new_tok_text, TokenType::IntConstant, curr_line_num, curr_char_pos));
    }

    // pub fn print(&self){
    //     &self.primitve_collection.print_prims();
    // }

    fn invalid(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
        // Create an empty string 
        let mut new_tok_text = String::new();
        // Grab the  primitive
        let mut pr = self.primitve_collection.get_curr_prim();
        // First rule -> Operator
        new_tok_text.push(pr.symbol);
        // Add invalid token 
        self.all_tokens.push(Token::new(new_tok_text, TokenType::Invalid, curr_line_num, curr_char_pos));
        // Increment that charcater position
        self.char_pos += 1;
    }

    fn newline(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
        // Create an empty string 
        let mut new_tok_text = String::new();
        // Grab the  primitive
        let mut pr = self.primitve_collection.get_curr_prim();
        // First rule -> Operator
        new_tok_text.push(pr.symbol);
        // Add new line token 
        self.all_tokens.push(Token::new(new_tok_text, TokenType::NewLine, curr_line_num, curr_char_pos));
        // Increment that charcater position
        self.char_pos += 1;
    }

    fn space(&mut self){
        // Grab the initial line and character positions
        let curr_char_pos = self.char_pos;
        let curr_line_num = self.line_num;
        // Create an empty string 
        let mut new_tok_text = String::new();
        // Grab the  primitive
        let mut pr = self.primitve_collection.get_curr_prim();
        // First rule -> Operator
        new_tok_text.push(pr.symbol);
        // Add space token 
        self.all_tokens.push(Token::new(new_tok_text, TokenType::Space, curr_line_num, curr_char_pos));
        // Increment that charcater position
        self.char_pos += 1;
    }

    

}





// The keywords of language X are: unsigned, char, short, int, long, float, double, while, if,
// return, void, and main.

fn is_keyword(input: &String) -> bool {

    if KEYWORD_ARRAY.iter().any(|&elem|  elem == input) {
        return true;
    }
    return false;
}
