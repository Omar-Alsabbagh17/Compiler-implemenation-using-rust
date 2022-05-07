def matching_parenthesis(input_str):
    ''''
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

    return -> boolean :   True if parantheses matches, false otherwise
    '''

    stack = []  # Stack datastructure LIFO
    for char in input_str: # traverse each char
        if char in ["(", "{", "["]:
            stack.append(char)
        elif char in [")", "}", "]"]:
            if not stack:  #if stack empty
                return False
            top = stack.pop()
            if top == "(":
                if char != ")":
                    return False
            if top == "{":
                if char != "}":
                    return False
            if top == "[":
                if char != "]":
                    return False
    
    #end of for loop
    if stack:
        #missing closing parantheses
        return False
    
    return True  #every paranthese matched
        


if __name__ == "__main__":
    input_str = "{(abddd)}"
    output = matching_parenthesis(input_str)
    if output:
        print("True")
    else:
        print("False")