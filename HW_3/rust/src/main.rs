extern crate custom_error;
use custom_error::custom_error;

custom_error!{MyError
    General{n:i32} = "Syntax error at character positon {n}."
}

fn str_to_vec(input: &str) -> Vec<char>
{
    /*
    this function converts input string
    into vector of characters
    */
    let mut v : Vec<char> = Vec::new();
    for i  in 0..input.len() as usize
        {
            v.push(input.chars().nth(i).unwrap());
        }
    return v;
}

struct SimpleParser
{
    input_str : Vec<char>,
    char_pos: i32
}


impl SimpleParser
{
    /*
    this is an implementation for a struct used as  parser for the following EBNF grammar
     <S> ::= { a } <X > | b <X >
     <X > ::= c | d

     * curly braces {} are for reptition
     * The input will be string

    */

    // constructor
    fn new(input:&str) -> Self{
        SimpleParser{
            input_str: str_to_vec(input),
            char_pos: -1
        }

    }
    fn fun_s(&mut self) -> Result< (), MyError>
    {
        /*
        this is the funtion that is used to check whether the input string
        followed the grammar or not. if input is not valid, then we raise
        raise custom error
        */
        if (self.input_str.len() == 0) //if we have empty string
            {
                Err(MyError::General{n:self.char_pos+1})?;
            }
        for i in 0..self.input_str.len()
        {
            let c = self.get_next_char();
            if !['a','b','c','d'].contains(&c)
            {
                Err(MyError::General{n:self.char_pos})?;
            }

            if c == 'a'
            {
                // 'a' can't be the last character
                if !self.more_char_available()
                {
                    Err(MyError::General{n:self.char_pos})?;
                }

                // after 'a', we must have either 'a','c', or 'd'
                if ! ['a', 'd', 'c'].contains(&self.peek_next_char())
                {
                    self.char_pos += 1;
                    Err(MyError::General{n:self.char_pos})?;
                }
            }

            else if c == 'b'
            {
                // b can't be the last character
                if !self.more_char_available()
                {
                    Err(MyError::General{n:self.char_pos})?;
                }
                // after 'b' we must have either 'c' or 'd'
                if ! ['d', 'c'].contains(&self.peek_next_char())
                {
                    self.char_pos += 1;
                    Err(MyError::General{n:self.char_pos})?;
                }
            }
            // throw error if we have 'cc', or 'cd'
            else if c == 'c' &&  self.more_char_available()
            {
                if self.fun_x()
                {
                    self.char_pos += 1;
                    Err(MyError::General{n:self.char_pos})?;
                }
            }
            // throw error if we have 'dc', or 'dd'
            else if c == 'd' &&  self.more_char_available()
            {
                if self.fun_x()
                {
                    self.char_pos += 1;
                    Err(MyError::General{n:self.char_pos})?;
                }
            }
        }
        // == end of for loop ===
        
        // since it passed above loop
        // then it must be valid input
        Ok(())

    }
    fn fun_x(&self) -> bool
    {
        return ( ['d', 'c'].contains(&self.peek_next_char()) )
    }
    fn get_next_char(&mut self) -> char
    {
        self.char_pos += 1;
        let i:usize = (self.char_pos) as usize; 
        return (self.input_str[i])
    }

    fn peek_next_char(&self) -> char
    {
        let i:usize = (self.char_pos+1) as usize; 
        return (self.input_str[i])
    }

    fn more_char_available(&self) -> bool
    {
        return self.char_pos < ( ( self.input_str.len() -1) as i32)
    }

}




fn main() {
    // array of test cases
    for test in ["a", "aa", "b",  "bb", "c", "cc", "d", "dd", "bc", "abc", "acac",  "acd", 
        "aaad", "cvc", "2yz", "acx" , ""]
    { 
        let mut sp = SimpleParser::new(test);
        match sp.fun_s()
        {
            Err(e) => println!("{}: {}",test,  e),
            _ => println!("{}: Input is valid", test)

        }
    }

 
} 
