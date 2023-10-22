
#![allow(warnings)]

mod primitive;
mod cstream;
mod tokenizer;
mod parser;
use std::env;

fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    // Create the cstream struct
    let mut cstream = cstream::build_cstream();
    // Create the primitive struct
    // The primitive struct is responsible for converting the 
    // cstream struct into an array or primiitives (letters, numbers, etc)
    let mut prim = primitive::PrimitiveCollection::new();
    // Pass the cstream struct to the primitive struct in order to generate
    // all of the primitives
    prim.create_prims(cstream);
    // Pass the primitive struct to the tokenizer
    let mut tokenizer = tokenizer::Scanner::new(prim);
    // Concert the primitives to tokens
    tokenizer.tokenize_prims();
    // Get a copy of the vector of tokens
    let all_tokens = tokenizer.get_all_tokens();
    

    let mut parser = parser::Parser::new(all_tokens);
    // parser.strip_space();
    //parser.test_tokens();
    let successful = parser.parse();
    if successful
    {
        // TO DO: ->  Output XHTML File
    }
    else
    {
        println!("Couldn't output XHTML File, since parse was not successful")
    }
    // println!("Printign all tokens");
    // for tok in all_tokens {
    //     tok.print();
    // }
    // // Gets the original token directly from the tokenizer struct
    // while tokenizer.more_tokens_available(){
    //     let tok = tokenizer.get_next_token();
    //     tok.print();
    // }
}


