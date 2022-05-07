
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// helper function to parse the content of the file 
//reference https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// helper function to parse the content of the file 
//returns content of the file as vector of char
fn helper(filename: &str) -> Vec<char>
{
    let mut v : Vec<char> = Vec::new();
    let lines = lines_from_file(filename);
    for line in lines {
        for i  in 0..line.len() as usize
        {
            v.push(line.chars().nth(i).unwrap());
        }
        v.push('\n');
    }
    return v;
}

// define the struct
struct CStream{
    data: Vec<char>,
    line_num: i32,
    char_pos: i32
  }
  
  // Functions for the struct
  impl CStream {
    // constructor
    fn new(file_name:&str) -> Self {
        CStream { 
            
            data: helper(file_name), //returned the content of the file as vector of char
            line_num: -1,
            char_pos: -1    
        }
    }
    // =========== internal function ===================
    fn internal_func(&mut self, update:bool, peak_ahead: usize) -> char
    {
        let mut c = ' ';
        let mut i:usize = 0;
        if self.char_pos == -1
        {
            c = self.data[i];
            if update
            {
                self.line_num = 1;
                self.char_pos = 0;
            } 
            return c;
        }
        if (peak_ahead != 0)
        {
            i = (self.char_pos) as usize + peak_ahead;
            c = self.data[i];
        }
        else
        {
            i = (self.char_pos) as usize + 1;
            c = self.data[i];
        }
        if update
        {
            self.char_pos += 1;
        }
        if (c == '\n'  && update)
        {
            self.line_num += 1;
        }
        return c; 
    }
    // =============================================================
    fn more_available(&self) -> bool
    {
        let count = self.data.len() as i32;
        return  self.char_pos < count -1 ;  
    }

    fn peek_next_char(&mut self) -> char
    {
        return self.internal_func(false, 0);
    }
    fn get_next_char(&mut self) -> char
    {
        return self.internal_func(true, 0);
    }
    fn peek_ahead_char(&mut self, n: usize) -> char
    {
        return self.internal_func(false, n+1);
    }
    fn get_cur_char(&self) -> char
    {
        let i = (self.char_pos) as usize;
        return self.data[i];
    }
    fn get_char_pos(&self) -> i32
    {return self.char_pos;}

    fn get_lin_num(&self) -> i32
    {return self.line_num;}


}


// ======== Testing Funciton  =============
pub fn run2()
{
    println!("Test case 1:");
    let  mut obj = CStream::new("myfile.txt");
    println!("{}",obj.peek_next_char());
    println!("{}",obj.get_next_char());
    println!("{}",obj.peek_ahead_char(4));
    println!("{}",obj.get_next_char());
    println!("{}",obj.get_next_char());
    assert_eq!(obj.get_next_char(), '\n');
    println!("{}",obj.get_next_char());
    println!("{}",obj.get_cur_char());
    println!("{}",obj.more_available());

    println!("\nTest case 2:");
    obj = CStream::new("myfile.txt"); //reread for test 2
    while obj.more_available()
    {
        println!("{}", obj.get_next_char())
    }
    println!("character left: {}", obj.more_available());

}
