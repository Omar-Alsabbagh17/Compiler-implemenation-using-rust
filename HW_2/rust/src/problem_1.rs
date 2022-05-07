
// define the struct
struct Stack<I32> {
    items: Vec<I32>,
  }
  
  impl<I32> Stack<I32> {
    fn new() -> Self {
      Stack { items: Vec::new() }
    }
  
    
    fn push(&mut self, item: I32) {
       // add new element to the stack
      self.items.push(item)
    }

    fn pop(&mut self) -> Option<I32> {
        //remove an element from stack and returns it.
        // returns"Option", so you will have to unwrap it
        return self.items.pop();
      }
    
    fn peek(&self) -> Option<&I32> {
      // returns the most recent element (without removing it)
      // returns"Option", so you will have to unwrap it
      return self.items.last();

    }
  }

// ========== Test cases  =======================
pub fn run()
{
    let mut st = Stack::new();
    st.push(1);
    st.push(2);
    assert_eq!(*(st.peek().unwrap()), 2);
    let mut poped = st.pop().unwrap();
    assert_eq!(poped, 2);
    st.push(99);
    assert_eq!(*(st.peek().unwrap()), 99);
    poped = st.pop().unwrap();
    assert_eq!(poped, 99);
    assert_eq!(*(st.peek().unwrap()), 1);
    poped = st.pop().unwrap();
    assert_eq!(poped, 1);
    println!("Testing was succeful")
}




