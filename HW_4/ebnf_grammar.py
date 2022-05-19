from enum import Enum

class TokenType(Enum):
    CONSTANT = 0
    OPERATOR = 1
    VARIABLE = 2
    SPECIAL  = 3

class Token:
    """"
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
    """
    def __init__(self, input) -> None:
        self.input = input.replace(" ", "") #remove whitespaces
        self.text = []
        self.token_type = [] 

    def scan(self):
        previous = False
        for c in self.input:
            # case we have constant
            if c == '0' or c == '1':
                self.token_type.append(TokenType.CONSTANT)
                self.text.append(c)

            #case we have variable
            elif c in ['a','b','c','d']:
                self.token_type.append(TokenType.VARIABLE)
                self.text.append(c)
            
            #case we have special
            elif c  == ';':
                self.token_type.append(TokenType.SPECIAL)
                self.text.append(c)
            elif  c == ':':
                previous = True
                continue
            elif c == '=' and previous:
                self.token_type.append(TokenType.SPECIAL)
                self.text.append(':=')
                previous = False

            #everything else is operator    
            else:
                self.token_type.append(TokenType.OPERATOR)
                self.text.append(c)

    def result(self):
        for i in range(0, len(self.text)):
            print("Token ",i," = ", self.text[i])
            print("Token type: ", self.token_type[i].name)
            print()


if __name__ == "__main__":
    test_cases = ["a:= 0 + 1", "a:= a + b * 1", "c:= (d * 1) + 0 + c;"]
    for test in test_cases:
        print("Testing: ", test)
        obj  = Token(test)
        obj.scan()
        obj.result()
        print("\n\n")
