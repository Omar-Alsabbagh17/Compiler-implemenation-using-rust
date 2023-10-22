
extern crate custom_error;
use custom_error::custom_error;

custom_error!{pub MyError
     //General{line_num:i32, char_pos:i32} = "Error at Line {line_num}, character {char_pos}"
    General{}= "Error!!! Parse was not successful"
}
use crate::primitive::PrimitiveCollection;
use crate::primitive::Primitive;
use crate::primitive::PrimitiveType;
use crate::tokenizer::Token;
use crate::tokenizer::TokenType;

pub struct Parser
{
    all_tokens: Vec<Token>,
    index : usize,
    successful: bool
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
    Parser
    {
        all_tokens: tokens,
        index: 0,
        successful: false
    }
    }
    pub fn parse(&mut self) -> bool
    {
        self.strip_space();
        //self.test_tokens();
        match self.run()
        {
            Err(e) => {
                println!("{}", e);
                return false;
            },
            _ => {
                println!("Input program is syntactacilly correct.");
                self.successful = true;
                return self.successful;
            }
        }
    }
    pub fn strip_space(&mut self)
    {
        let mut i  = 0 as usize;
         while i < self.all_tokens.len()
        {
            if let TokenType::Space =  self.all_tokens[i].token_type
            {
                self.all_tokens.remove(i);
                continue;
            }
            // remove where end-of-line is considered operator
            if let TokenType::Operator =  self.all_tokens[i].token_type
            {
                if self.all_tokens[i].text.trim().is_empty()
                {
                    self.all_tokens.remove(i);
                    continue;
                }
            }
            i += 1;
        }
    }
    pub fn parse_successful(&mut self) -> bool
    {
        return self.successful;
    }
    pub fn test_tokens(&mut self)
    {
        for token in &self.all_tokens
        {
            println!("text: {}", token.text);
            println!("type: {}\n", token.token_type);
        }       
    }

    fn run(&mut self) -> Result<(), MyError> {
        let mut idx: usize = 0;
        while true {
            match self.Declaration(idx) {
                Some(x) => { idx = x; },
                _ => { break; }
            }
        }
        //println!("Token before main: {}", self.all_tokens[idx].text);
        match self.MainDeclaration(idx) {
            Some(x) => { idx = x; },
            _ => {
                //print!("Error at MainDeclartion");
                let a = self.all_tokens[self.index].line_num;
                let b = self.all_tokens[self.index].char_pos;
                //return Err(MyError::General{line_num:a, char_pos:b})?;
                return Err(MyError::General{});
            }
        }
        // println!("{}" ,idx.to_string());
        // println!("{}", self.all_tokens.len());
        // println!("Token before FunctionDefinition: {}", self.all_tokens[idx].text);
        while true {
            match self.FunctionDefinition(idx) {
                Some(x) => { idx = x; },
                _ => { break; }
            }
        }
        //println!("idx: {}", idx.to_string());
        // println!("{}", self.all_tokens.len());
        if idx != self.all_tokens.len()
        {
            let a = self.all_tokens[self.index].line_num;
            let b = self.all_tokens[self.index].char_pos;
             //return Err(MyError::General{line_num:a, char_pos:b})?;
             return Err(MyError::General{});
        }
        return Ok(());
    }

    fn Declaration(&mut self, mut idx: usize) -> Option<usize> {
        match self.DeclarationType(idx) {
            Some(x) => {
                idx = x;
                match self.VariableDeclaration(idx) {
                    Some(x) => {
                        //println!("Enter var");
                        return Some(x);
                    },
                    _ => {}
                }
                match self.FunctionDeclaration(x) {
                    Some(x) => {
                        //println!("Enter function");
                        return Some(x);
                    },
                    _ => { self.index = idx; }
                }

                self.index = idx; return None;
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn MainDeclaration(&mut self, mut idx: usize) -> Option<usize> {
        
        if let "void" = self.all_tokens[idx].text.as_str(){
            idx += 1;
            if let "main" = self.all_tokens[idx].text.as_str(){
                idx += 1;
                if let "(" = self.all_tokens[idx].text.as_str(){
                    idx += 1;
                    if let ")" = self.all_tokens[idx].text.as_str(){
                        idx += 1;
                        match self.Block(idx) {
                            Some(x) => {
                                return Some(x);
                            },
                            _ => {
                                //println!("didn't enter Block\n"); 
                                self.index = idx; return None; }
                        }
                    } 
                    else {self.index = idx; return None;}

                } 
                else {self.index = idx; return None;}

            } else {self.index = idx; return None;}

        } 
        else {self.index = idx; return None;}
    }

    fn FunctionDefinition(&mut self, mut idx: usize) -> Option<usize> {
        match self.DeclarationType(idx) {
            Some(x) => {
                match self.ParameterBlock(x) {
                    Some(y) => {
                        match self.Block(y) {
                            Some(z) => {
                                return Some(z);
                            },
                            _ => { self.index = idx; return None; }
                        }
                    },
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn DeclarationType(&mut self, mut idx: usize) -> Option<usize> {
        match self.DataType(idx) {
            Some(x) => {
                idx = x;
                match self.all_tokens.get(idx) {
                    Some(token) => {
                        if let TokenType::Identifier = token.token_type
                        {
                            return Some(idx+1);
                        } 
                        else 
                        {
                            self.index = idx; return None;
                        }
                    },
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn VariableDeclaration(&mut self, mut idx: usize) -> Option<usize> {
       
        if let "="  = self.all_tokens[idx].text.as_str(){
            idx += 1;
            match self.Constant(idx) {
                Some(x) => {
                    idx = x;
                },
                _ => { self.index = idx; return None; }
            }
        }
        if let ";" =  self.all_tokens[idx].text.as_str() {
            return Some(idx+1);
        } else {
            self.index = idx; return None;
        }
    self.index = idx; None
    }

    fn FunctionDeclaration(&mut self, mut idx: usize) -> Option<usize> {
        match self.ParameterBlock(idx) {
            Some(x) => {
                idx = x;
                if let ";"  = self.all_tokens[idx].text.as_str(){
                    return Some(idx+1);
                } 
                else {
                    self.index = idx; return None;
                }
                    
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn Block(&mut self, mut idx: usize) -> Option<usize> {
        if let "{" = self.all_tokens[idx].text.as_str(){
            idx += 1;
            while true {
                match self.Declaration(idx) { 
                    Some(x) => { idx = x; }, 
                    _ => {break; } 
                }
            }
            while true {
                match self.Statement(idx) {
                    Some(x) => {idx = x; },
                    _ => { break; }
                }
            }
            while true {
                match self.FunctionDefinition(idx) {
                    Some(x) => {idx = x; },
                    _ => { break; }
                }
            }
            if idx >= self.all_tokens.len()
            {
                self.index = idx; return None;
            }
            if let "}" = self.all_tokens[idx].text.as_str(){
                return Some(idx+1);
            } 
            else {
                self.index = idx; return None;
            }

        } else {
            self.index = idx; return None;
        }

        self.index = idx; None
    }

    fn ParameterBlock(&mut self, mut idx: usize) -> Option<usize> {
        if let "(" = self.all_tokens[idx].text.as_str(){
            match self.Parameter(idx+1) { 
                Some(x) => {
                    idx = x;
                    while true {
                        if let "," = self.all_tokens[idx].text.as_str(){
                            idx += 1;
                            match self.Parameter(idx) {
                                Some(x) => { idx = x; },
                                _ => { self.index = idx; return None; }
                            }
                        } 
                        else { break; }

                    }
                },
                _ => { self.index = idx; } 
            }
            if let ")" = self.all_tokens[idx].text.as_str(){
                return Some(idx+1);
            } else {
                self.index = idx; return None; 
            }

        } 
        else {
            self.index = idx; return None;
        }

        self.index = idx; None
    }

    fn DataType(&mut self, idx: usize) -> Option<usize> {
        match self.IntegerType(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        match self.FloatType(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        self.index = idx; None
    }

    fn Constant(&mut self, mut idx: usize) -> Option<usize> {
        if self.all_tokens[idx].is_constant() {
            return Some(idx+1);
        } else {
            self.index = idx; return None;
        }
    }

    fn FACTOR(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) { 
            Some(token) => {
                if let "(" = token.text.as_str() {
                    idx += 1;
                    match self.Expression(idx) {
                        Some(x) => {
                            idx = x;
                            match self.all_tokens.get(idx) {
                                Some(token) => {
                                    if let ")" = token.text.as_str() {
                                        return Some(idx+1);
                                    }
                                },
                                _ => { self.index = idx; return None; } 
                            }
                        },
                        _ => { self.index = idx; return None; } 
                    }
                }
            },
            _ => { self.index = idx; return None; } 
        }
        
        match self.Constant(idx) { 
            Some(x) => {
                return Some(x);
            },
            _ => { self.index = idx; }
        }
        match self.all_tokens.get(idx) { 
            Some(token) => {
                if  let TokenType::Identifier = token.token_type {
                    idx += 1;
                    match self.all_tokens.get(idx) {
                        Some(token) => {
                            if let "(" = token.text.as_str() {
                                idx += 1;
                                match self.Expression(idx) {
                                    Some(x) => {
                                        idx = x;
                                        while true {
                                            match self.all_tokens.get(idx) {
                                                Some(token) => {
                                                    if let "," = token.text.as_str() {
                                                        idx += 1;
                                                        match self.Expression(idx) {
                                                            Some(x) => { idx = x; },
                                                            _ => { self.index = idx; return None; }
                                                        }
                                                    } else { break; }
                                                },
                                                _ => { self.index = idx; return None; }
                                            }
                                        }
                                        match self.all_tokens.get(idx) {
                                            Some(token) => {
                                                if let ")"= token.text.as_str() {
                                                    idx += 1;
                                                } else {
                                                    self.index = idx; return None;
                                                }
                                            },
                                            _ => { self.index = idx; return None; }
                                        }
                                    },
                                    _ => { self.index = idx; return None; }
                                }
                            }
                        }, 
                        _ => { self.index = idx; return None; }
                    }
                    return Some(idx);
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }


    fn IntegerType(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
        Some(token) => {if let "unsigned" = token.text.as_str(){idx += 1;}},
        _ => { self.index = idx; return None; }
        }
        match self.all_tokens.get(idx) {
            Some(token) => {
                match token.text.as_str() {
                "int" | "short" | "long" | "char" => {
                    return Some(idx+1);
                },
                _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }
    fn Statement(&mut self, mut idx: usize) -> Option<usize> {
        match self.Assignment(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        match self.WhileLoop(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        match self.IfStatement(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        match self.ReturnStatement(idx) {
            Some(x) => { return Some(x); },
            _ => { self.index = idx; }
        }
        match self.Expression(idx) {
            Some(x) => {
                idx = x;
                if let ";" = self.all_tokens[idx].text.as_str(){
                        return Some(idx+1);
                }
      
            },
            _ => { self.index = idx; }
        }
        self.index = idx; None
    }

    fn Assignment(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                if let TokenType::Identifier = token.token_type {
                    idx += 1;
                    match self.all_tokens.get(idx) {
                    Some(token) => {
                        if token.text == "=" {
                            idx += 1;
                            while true {
                            match self.all_tokens.get(idx) {
                                Some(token) => {
                                    if  let TokenType::Identifier = token.token_type {
                                        match self.all_tokens.get(idx + 1) {
                                            Some(token) => {
                                                if token.text == "=" {
                                                    idx += 2;
                                                } else {
                                                    break;
                                                }
                                            },
                                            _ => { self.index = idx; return None; }
                                        }
                                    } else {
                                        break;
                                    }
                                },
                                _ => { self.index = idx; return None; }
                            }
                            }
                                match self.Expression(idx) {
                                    Some(x) => { idx = x; },
                                    _ => { self.index = idx; return None; }
                                }
                                match self.all_tokens.get(idx) {
                                    Some(token) => {
                                        if token.text == ";" {
                                            return Some(idx+1);
                                        } else {
                                            self.index = idx; return None;
                                        }
                                    },
                                    _ => { self.index = idx; return None; }
                                }
                            } else {
                                self.index = idx; return None;
                            }
                        },
                        _ => { self.index = idx; return None; }
                    }
                } else {
                    self.index = idx; return None;
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }
    fn Parameter(&mut self, mut idx: usize) -> Option<usize> {
        match self.DataType(idx) {
            Some(x) => {
                idx = x;
                match self.all_tokens.get(idx) {
                    Some(token) => {
                        if let TokenType::Identifier = token.token_type {
                            return Some(idx+1);
                        } else {
                            self.index = idx; return None;
                        }
                    },
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn WhileLoop(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                if let "while" = token.text.as_str() {
                    idx += 1;
                    match self.all_tokens.get(idx) {
                        Some(token) => {
                            if  let "(" = token.text.as_str() {
                            idx += 1;
                            match self.Expression(idx) {
                                Some(x) => {
                                    idx = x;
                                    match self.all_tokens.get(x) {
                                    Some(token) => {
                                    if let ")" = token.text.as_str(){
                                        idx += 1;
                                        match self.Block(idx) {
                                            Some(x) => {
                                                return Some(x);
                                            },
                                            _ => { self.index = idx; return None; }
                                                }
                                        } else {
                                            self.index = idx; return None;
                                        }
                                    },
                                    _ => { self.index = idx; return None; }
                                }
                            },
                            _ => { self.index = idx; return None; }
                        }
                    } else {
                        self.index = idx; return None;
                    }
                        },
                        _ => { self.index = idx; return None; }
                    }
                } else {
                    self.index = idx; return None;
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn FloatType(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                match token.text.as_str() {
                    "float" | "double" => {
                        return Some(idx+1);
                    },
                    _ => {  }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn IfStatement(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                if let "if" = token.text.as_str() {
                    idx += 1;
                    match self.all_tokens.get(idx) {
                    Some(token) => {
                        if token.text == "(" {
                            idx += 1;
                            match self.Expression(idx) {
                                Some(x) => {
                                    idx = x;
                                match self.all_tokens.get(idx) {
                                Some(token) => {
                                    if let ")" = token.text.as_str() {
                                        idx += 1;
                                        match self.Block(idx) {
                                            Some(x) => {
                                                return Some(x);
                                            },
                                            _ => { self.index = idx; return None; }
                                        }
                                    } else {
                                        self.index = idx; return None;
                                    }
                                        },
                                        _ => { self.index = idx; return None; }
                                    }
                                    },
                                    _ => { self.index = idx; return None; }
                                }
                            } else {
                                self.index = idx; return None;
                            }
                        },
                        _ => { self.index = idx; return None; }
                    }
                } else {
                    self.index = idx; return None;
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }
    fn Term(&mut self, mut idx: usize) -> Option<usize> {
        match self.FACTOR(idx) {
            Some(x) => {
                idx = x;
                while true {
                  match self.MultOperator(idx) {
                    Some(x) => {
                        idx = x;
                        match self.FACTOR(idx) {
                            Some(x) => { idx = x; },
                            _ => { self.index = idx; return None; }
                        }
                    },
                    _ => { break; }
                }
                }
                return Some(idx);
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

 
    fn ReturnStatement(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                if let "return" = token.text.as_str() {
                    idx += 1;
                    match self.Expression(idx) {
                        Some(x) => {
                            idx = x;
                            match self.all_tokens.get(idx) {
                                Some(token) => {
                                    if token.text == ";" {
                                        return Some(idx+1);
                                    } else {
                                        self.index = idx; return None;
                                    }
                                },
                                _ => { self.index = idx; return None; }
                            }
                        },
                        _ => { self.index = idx; return None; }
                    }
                } else {
                    self.index = idx; return None;
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }
    fn RelationOperator(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                match token.text.as_str() {
                    "<=" | "<" | ">" | ">=" | "==" | "!=" => { return Some(idx+1);},
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn Expression(&mut self, mut idx: usize) -> Option<usize> {
        match self.SimpleExpression(idx) {
            Some(x) => {
                idx = x;
                match self.RelationOperator(idx) {
                    Some(x) => {
                        idx = x;
                        match self.SimpleExpression(idx) {
                            Some(x) => { return Some(x); },
                            _ => { self.index = idx; return None; }
                        }
                    },
                    _ => { self.index = idx; }
                }
                return Some(idx);
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

    fn SimpleExpression(&mut self, mut idx: usize) -> Option<usize> {
        match self.Term(idx) {
            Some(x) => {
                idx = x;
                while true {
                    match self.AddOperator(idx) {
                        Some(x) => {
                            idx = x;
                            match self.Term(idx) {
                                Some(x) => { idx = x; },
                                _ => { self.index = idx; return None; }
                            }
                        },
                        _ => { break; }
                    }
                }
                return Some(idx);
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; None
    }

  
    fn AddOperator(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                match token.text.as_str()  {
                    "+" | "-" => {return Some(idx+1);},
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; 
        None
    }

    fn MultOperator(&mut self, mut idx: usize) -> Option<usize> {
        match self.all_tokens.get(idx) {
            Some(token) => {
                match token.text.as_str() {
                    "*" | "/" => {return Some(idx+1);},
                    _ => { self.index = idx; return None; }
                }
            },
            _ => { self.index = idx; return None; }
        }
        self.index = idx; 
        None
    }

}

