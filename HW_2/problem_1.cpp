#include <iostream>
#include <vector>
using namespace std;
class Stack
{
    /*
    this class is implementation of Stack data strcutre 
    which is LIFO (last in, first out) type data structure
    think of it like you have pile of dishes
    the new dish comes in the top of the pile
    and that new dish will be the first to be washed.
    */
    private:
        // used to store items of the stack
        vector<int> data;
    public:
        Stack();    // default constructor
        Stack(int);
        ~Stack();   // destructor
        void push(int);      // add new element to the stack (ie add new dish)
        int pop();          // move the top element (ie wash the most recent dish)
        int peek();        // return the top element in the stack
        void traverse();  // traverse the stack
        void size();     // returns the number of items in the stack (ie how many dished we have)
};
// end of class definition

Stack::Stack()
{
    this->data.reserve(5);
}

Stack::Stack(int n)
{
    vector<int> data;
    this->data.reserve(n);

}
//==============================
void Stack::push(int item)
{
    // adds the parameter item into top of the stack
    this->data.push_back(item);
}
//==================
int Stack::pop()
{
    // pops and returns the topmost item in the stack
    if (this->data.empty())
    {
        cout <<"ERROR!!! can't remove any element, the stack is empty\n";
        throw -1;
    }
    int temp = this->data.back();
    this->data.pop_back();
    return temp;
}
//======================
int Stack::peek()
{
    // returns topmost item in the stack
    if (this->data.empty())
    {
        cout <<"ERROR!!! can't Peek, the stack is empty\n";
        throw -1;
    }
    return this->data.back();
}
//====================
void Stack::traverse()
{
    if (this->data.empty())
    {
        cout <<"Can't traverse empty stack\n";
        return;
    }
    cout <<"The items in the stack: \n";
    for (auto i = this->data.rbegin(); i != this->data.rend(); ++i)
        cout << *i <<" ";
    cout <<"\n";
}
//=================================
void Stack::size()
{
    cout <<"Total items in the stack: " <<this->data.size() <<endl;
}
//==============================


Stack::~Stack() { }//Destructor
//==================================================================
//end of class implementation

int main()
{
    Stack st;
    st.traverse();
    st.push(3);
    st.push(55);
    st.push(47);
    st.size();
    st.traverse();
    cout <<"Poped: "<<st.pop() <<endl;
    cout << "Peeking: " <<  st.peek() <<endl;
    cout <<"Poped: " << st.pop() <<endl;
    cout << "Peeking: " <<  st.peek() <<endl;

}

