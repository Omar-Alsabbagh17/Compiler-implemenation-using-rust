
# Description of problems


## Problem 1 (36 points)
A stack (Wikipedia) is an abstract data type that contains a collection of elements with two principal
**operations:**  
    *    push: add an element to the collection    
    *    pop: removes the most recently added element from the collection and returns the element   
Because pop only removes the most recently added element, a stack follows the LIFO (last in first out)
order.  

For this problem, you need to define the stack data structure in all three languages (C++, Python and
Rust). In other words, in C++ and Python, define a class called Stack and in Rust, define a struct called
Stack. For simplicity, we assume the stacks can only contain intergers (i32 for Rust).
Your class/struct should define (at least) the following functions:
1
• the initializer: a stack is always initizlied as empty. Don’t need to worry about initializing a stack
with an array of numbers.
• push: add an element onto the stack
• pop: removes the most recently added element from the collection and returns the element
• peek: returns the most recently added element (without removing it)
Language: Write the function in C++, Python, and Rust. You may use whichever you are most
comfortable with as the first language. (See syllabus for details.) (<br>)  
#### Examples: 
Let’s start with an empty stack st and perform the following functions in order.
>  st.push(3)   
> st.peek() (should return 3)    
> st.push(4)   
>  st.push(5)   
> st.pop() (should delete and return 5)   
> st.peek() (should return 4)   
> st.push(6)    
> st.pop() (should delete and return 6)


## Problem 2 (48 points)
Define a data structure–a class in C++/Python and a struct in Rust–called CStream that can read in a file
character by character.
The class/struct should have (at least) the following attributes:   
• filename: a string to store the name of the input file    
• line_num: the line number of the current character (starting with line 0)   
• char_pos: the position of the current character on this line (starting with position 0)     
The class/struct should also define (at least) the following functions:      
• the initializer:
– The initializer should take only one argument: the name of the input file.     
– line_num and char_pos should be initialized to -1 since we haven’t started reading the file yet   
• more_available(): returns True if there are still characters available, i.e., remaining to be read; False 
if we have reached the end of the file     
• get_cur_char(): returns the current character, i.e., the one that at the char_pos character position on    
line line_num      
• get_next_char(): Moves to the next character and returns it     
2       
– Do NOT worry about reaching the end of the file for now. Assume the function is only called    
when there are characters available, i.e., more_available returns true.      
• peek_next_char(): Returns the next character     
– Do NOT worry about reaching the end of the file for now. Assume the function is only called     
when there are characters available, i.e., more_available returns true.              
• peek_ahead_char(k): Returns the kth character ahead in the stream      
– peek_ahead_char(0) returns the same character as peek_next_char()       
– Do NOT worry about reaching the end of the file for now. Assume the function is only called      
when there are at least k+1 characters available      
Language: Write the function in Rust and one of C++ and Python. You may use whichever you are     
most comfortable with as the first language. (See syllabus for details.)         
***Examples:****   
Suppose our file looks like this: 
1 dog
2 cat
3 apple     
There are three lines in the file. Let perform the following functions in order:      
1 initializer and let the object be called      
Result: line_num = -1, char_pos = -1   
2. f.peek_next_char()          
Result: should return character ‘d’; line_num and char_pos remain unchanged   
3. f.get_next_char()      
Result: should return character ‘d’; line_num and char_pos are both incremented by 1      
4. f.peek_ahead_char(4)    
Result: should return character ‘t’; line_num and char_pos remain unchanged    
5. f.get_next_char()    
Result: should return character ‘o’; line_num remains unchanged; char_pos is incremented by 1   
6. f.get_next_char()     
Result: should return character ‘g’; line_num remains unchanged; char_pos is incremented by 1    
7. f.get_next_char()    
Result: should return character ‘c’; line_num is incremented by 1; char_pos is incremented by 1     
8. f.get_cur_char()     
Result: should return character ‘c’; line_num and char_pos remain unchanged     
9. f.more_available():     
should return true
Hint:      
• C++ has a file stream library you may use if you decide to use C++.
• Rust does not have such libraries and it is difficult to do actual streaming in Rust. The easiest way
is probably store the whole content of the file as a string or a vector of strings first and then just
read from there.    
   
## Non-Programming Problems (16 points)    
1. (6 points) Given the following lexemes in C++:    
• “Hello World”    
• myName   
• *   
• 98   
• 5.4    
• return   
What is each of the lexeme’s token? Suppose the tokens we have are constant, keyword, identifier,
and operator.
2. (10 points) Given the following BNF:   
1 < expr > -> < expr > + < expr > | < integer > | < float >   
2 < float > -> < integer > | < integer > . < whlnum >   
3 < integer > -> < whlnum > | - < whlnum >   
4 < whlnum > -> < digit > | < digit > < whlnum >   
5 < digit > -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9     
Draw two different parse trees for 2.5 + -1 + 4
