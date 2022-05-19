#[derive(Debug)]
enum TokenType 
{
    CONSTANT, VARIALBE, OPERATOR, SPECIAL
    // let a = TokenType::CONSTANT;
    // print!("{:?}",a);
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
            if input.chars().nth(i).unwrap() == ' '
            {
                // remove whitespaces
                continue;
            }
            v.push(input.chars().nth(i).unwrap());
        }
    return v;
}

struct Token
{
    input_str: Vec<char>,
    text: Vec<char>,
    token_type: Vec<TokenType>
}
impl Token
{
    /*
    This class is used to extract the token types in the provided string input
    the Token type can be one of the followings:
    CONSTANT, VARIABLE, OPERATOR, SPECIAL
    • Constants: 0 and 1
    • Variables: a, b, c, d
    • Special: := and ;
    • Anything else is an operator

    * The input program is assumed to be syntatically correct.
    • The input program is represented as a string.
    
    EXAMPLE:
    inuput =>  a:=1+b
    a is variable
    := is special
    + is operator
    1 is constant
    */

    // constructor
    fn new(input:&str) -> Self
    {

        Token{
            input_str: str_to_vec(input),
            text: Vec::new(),
            token_type: Vec::new()
        }
    }
    fn scan(&mut self)
    {
        let mut previous : bool = false;
        for c in &self.input_str
        {
            if *c == '0' || *c == '1'
            {
                // case we have constant
                self.text.push(*c);
                self.token_type.push(TokenType::CONSTANT);
            }

            else if  ['a','b','c','d'].contains(c)
            {
                // case we have variable
                self.text.push(*c);
                self.token_type.push(TokenType::VARIALBE);
            }

            //case we have special
            else if *c == ';'{
            self.text.push(*c);
            self.token_type.push(TokenType::SPECIAL);
            }
            else if *c == ':'
            {
                previous = true;
                continue;
            }
            else if *c == '=' && previous
            {
                self.text.push(':');
                self.token_type.push(TokenType::SPECIAL);
            }
            
            //everything else
            else
            {
                self.text.push(*c);
                self.token_type.push(TokenType::OPERATOR);
            }
        }
    }

    fn result(& self)
    {
        for i in 0..self.text.len()
        {
            if self.text[i] == ':'
                {  println!("Token {} = :=", i) }
            else { println!("Token {} = {}", i, self.text[i]) }
            println!("Token type: {:?}\n", self.token_type[i]);
        }
    }



}
fn main()
{
    let test_cases = ["a:= 0 + 1", "a:= a + b * 1", "c:= (d * 1) + 0 + c;"];
    for test in test_cases{
        println!("Testing: {}", test);
        let mut obj = Token::new(test);
        obj.scan();
        obj.result();
        println!("\n\n");
    }
    
}
