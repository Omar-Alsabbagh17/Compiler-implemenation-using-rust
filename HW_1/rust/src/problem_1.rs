
fn matching_parentheses(input: String) -> bool
{
    /*
    This function accepts a string and checks whether it has matching
    parantheses or not. That's it, it checks whether each opening paranthese
    it's own closing parantheses in correct order.
    The function ignores letters, numbers, and any other character
     and only checks the following parantheses:
     {}, (), []

    example:
    ([])     : true
    ([)      : false, since "["  is missing it's closing parntheses "]"
    ()ab{}123:  true

    parameters:
    input -> String:      the string that you want to test
                          to see wehther it has matching parantheses or not

    return -> boolean :   True if parantheses matches, false otherwise
    */
    let mut stk: Vec<char> = vec![]; // our stack LIFO
    let mut top : char;
    for n in input.chars()
    {
        //if we see opening parantheses 
        if n == '('  || n == '[' || n  == '{'
        {
            stk.push(n);
            continue;
        }
        // if we see closing parantheses
        if n == ')'  || n == ']' || n  == '}'
        {
            if stk.len() ==  0 //if  empty
            {
                return false;
            }
            top = stk.pop().unwrap();
            // if there is a mismatch, then return false 
            if top == '('  &&  n != ')' {return false;}
            if top == '['  &&  n != ']' {return false;}
            if top == '{'  &&  n != '}' {return false;}

        }
    }
    if stk.len() !=  0  //if not empty
    {
        // openining parentheses is missing closing parantheses, so return false
        return false
    }
    return true
}
//=================================================
pub fn run()
{
    /*
    run this function in the main.rs by using below code:

    mod problem_1; 
    problem_1::run();

    */
    let str = "(((((".to_string(); //to_string converts the variable from &str to String
    let output = matching_parentheses(str);
    println!("{}", output);

}
