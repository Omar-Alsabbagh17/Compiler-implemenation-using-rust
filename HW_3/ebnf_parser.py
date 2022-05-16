import string


class SimpleParser:
    """
    this is a class for a simple  parser for the following EBNF grammar
     <S> ::= { a } <X > | b <X >
     <X > ::= c | d

     * curly braces {} are for reptition
     * The input will be string

    """
    def __init__(self, str) -> None:
        self.input_str = str  #the input string which we will check 
        self.char_pos = -1  #position of current character (-1 means we have not read yet!)
    
    def fun_s(self):
        """
        this is the funtion that is used to check whether the input string
        followed the grammar or not. if input not valid, then we raise
        exception and catch it
        """
        try:
            if len(self.input_str) == 0: #if we have empty string
                self.char_pos += 1
                raise Exception()
            for i in range(0, len(self.input_str)):
                c = self.__get_next_char()
                if c not in ['a', 'b', 'c', 'd' ]:
                    raise Exception() 
                if c == 'a':
                    # 'a' can't be the last character
                    if not(self.__more_char_available()):
                        raise Exception()
                    # after 'a' we must either have 'a','c', or 'd'
                    if self.__peek_next_char()  not in ['a', 'c','d']:
                        self.char_pos += 1
                        raise Exception()
                    
                elif c == 'b':
                    # 'b' can't be the last character
                    if not(self.__more_char_available()):
                        raise Exception()
                    # after 'b' we must either have 'c', or 'd'
                    if self.__peek_next_char() not in ['c','d']:
                        self.char_pos += 1
                        raise Exception()
                        
                # throw error if we have 'cc', or 'cd'        
                elif c == 'c' and self.__more_char_available():
                    if self.__fun_x():
                        self.char_pos += 1
                        raise Exception()
                
                # throw error if we have 'dc', or 'dd' 
                elif c == 'd'  and self.__more_char_available():
                    if self.__fun_x():
                        self.char_pos += 1
                        raise Exception()
            # == end of for loop ===
            
            # since it passed the above loop
            # then it must be valid input
            print("Input is valid")
            
        except:
            print("Syntax error at character positon ", self.char_pos)
        

    def __fun_x(self):
        return (self.__peek_next_char()  in ['d', 'c'])

    def __get_next_char(self):
        self.char_pos += 1
        return( self.input_str[self.char_pos])

    def __peek_next_char(self):
        return (self.input_str[self.char_pos+1])
        
    def __more_char_available(self):
        return (self.char_pos < len(self.input_str)-1)


if __name__ == "__main__":
    for test in ["a", "aa", "b",  "bb", "c", "cc", "d", "dd", "bc", "abc", "acac",  "acd", 
        "aaad", "c", "2yz", "acx" , ""]:
        if test == "":
            print("Empty", end=":  " )
        else:
            print(test, end=":  " )
        sp = SimpleParser(test)
        sp.fun_s()
