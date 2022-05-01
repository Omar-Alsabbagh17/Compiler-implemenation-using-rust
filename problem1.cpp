#include <iostream>
#include <stack>

using namespace std;

bool matching_parenthesis(string input_str)
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
    input_str -> String:      the string that you want to test
                          to see wehther it has matching parantheses or not

    return -> boolean :   1 if parantheses matches, 0 otherwise
    */


    stack <char> stk; //stack is datastructure that is LIFO 
    char top;
    for (int i = 0; i < (int)input_str.length(); i++) //traverse each char
    {
        if (input_str[i] == '(' || input_str[i] == '[' || input_str[i] == '{')
                stk.push(input_str[i]);
        else if (input_str[i] == ')' || input_str[i] == ']' || input_str[i] == '}')
        {
            if (stk.empty()) return false; //closing parantheses must be preceded by opening parantheses
            top = stk.top();
            stk.pop();
            //if there is mismatch, then return false
            if (top == '(' && (input_str[i] != ')'))  return false;
            if (top == '[' && (input_str[i] != ']'))  return false;
            if (top == '{' && (input_str[i] != '}'))  return false;

        }
    }
    if (!stk.empty())
    {
        // stack is not empty
        //this means we are missing closing parantheses
        return false;
    }
    return true;
}

int main()
{
    string input_str = "{[]}44[]";
    bool output = matching_parenthesis(input_str);
    if (output) cout<<"True"  <<endl;
    else        cout<<"False" <<endl;
    return 0;
}
