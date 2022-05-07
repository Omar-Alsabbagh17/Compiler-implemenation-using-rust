class Stack:
    """"
    this is a class for Stack data structure.
    Stack is LIFO datastructure (ie Last element in, is the first one removed)
    think of it like you have pile of dishes
    the new dish comes in the top of the pile
    and that new dish will be the first to be washed.
    """
    # initializer
    def __init__(self) -> None:
        self.data = []
    
    # add new item to the stack
    def push(self, item):
        self.data.append(item)
    # remove the recently added item from the stack
    def pop(self):
        if (len(self.data)== 0):
            print("Didn't remove any element, the stack is empty")
            return 
        removed_item = self.data.pop()
        return removed_item
    # returns the most recent element (without removing it)
    def peek(self):
        if (len(self.data) == 0):
            print("Can't peek, the stack is empty")
            return
        return self.data[-1]


#testing the class
if __name__ == "__main__":
    stack = Stack()
    stack.pop()
    stack.peek()
    stack.push(3)
    print("Peeked: ", stack.peek())
    stack.push(6)
    stack.push(9)
    print("Peeked: ", stack.peek())
    print("data removed: ", stack.pop())
    print("peeked: ", stack.peek())
