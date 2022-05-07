use std::cmp;

fn compare(str1: String, str2: String) -> String
{
    /*
    This is internal function.
    This funciton accepts two string and returns
    the longest common prefix
    */

    let n1 = str1.len();
    let n2 = str2.len();
    let min_len = cmp::min(n1, n2);
    let mut prefix = String::from("");
    for i in 0..min_len
    {
        if str1.chars().nth(i).unwrap()  != str2.chars().nth(i).unwrap()
        {
            break;
        }
        prefix.push(str1.chars().nth(i).unwrap()); //add current char into prefix
    }
    return prefix;
}


fn LongestCommonPrefix<'a>(arr: &'a Vec<String >) -> String
{
    /*
    This function accepts a vector of Strings and returns the
    longest common prefix between those strings.

    Example:
    given vector of strings ["apple", "app", "aple", "appl"], the longest common  
    prefix is "ap".

    parameters:
    arr : vector of strings

    Return -> string: the longest common prefix between the vector of strings
    */
    let n = arr.len();
    let mut common = arr[0].to_owned();
    for i in 1..n
    {
        common = compare(common, arr[i].to_owned());
        if common == "" {break;}  //no prefix is found
    }
    return common;
}

//=================================================
pub fn run2()
{
    /*
    run this function in the main.rs by using below code:

    mod problem_2; 
    problem_2::run2();
    */
    let   arr: Vec<_> = vec!["apple", "app", "aplp", "appleor"].into_iter().map(String::from).collect();
    let out =  LongestCommonPrefix(&arr);

    if out == "" { println!("There is no common prefix");}
    else {println! ("The longest common prefix is: {}", out) }


}
