use std::fs;


pub struct CStream{
    file_name: String, // String variable which will contain the file name
    content_vec: Vec<char>, // Vector which will contain every character as an entry
    line_num: i32, // Variable which will track the current line the program is on
    char_pos: i32, // Variable which will track the current character position
    total_char_count: i32, // Variable which will contain the total number of characters
    wrd_cnt_per_line: Vec<i32>, // Vector which will contain the number of characters on each line, plus 1
}

impl CStream {
    pub fn parse_file(&mut self){
        self.content_vec = self.open_file().chars().collect();
        for entry in &self.content_vec { // Iterate through the file, character by character
            // self.content_vec.push(entry as char);
            self.total_char_count += 1;
            // println!("{} {}", &self.total_char_count, &entry);
            if *entry == '\n' { // If the character is a newline, append the running total char count to the word cnt per line vector
                self.wrd_cnt_per_line.push(self.total_char_count);
            }
            
        }
        // Append the final total character count to the wrd cnt per line vector
        self.wrd_cnt_per_line.push(self.total_char_count);
    }

    // If the character position is less than the total amount of characters avialble, return true. Else, return false
    pub fn more_available(&self) -> bool{
        // println!("{} {} {}", &self.char_pos, &self.total_char_count, &self.content_vec.len());
        self.char_pos + 1 < (self.content_vec.len() as i32) 
    }

    fn open_file(&self) -> String {
        fs::read_to_string(&self.file_name)
            .expect("Couldn't open file\n")
    }


    // Return the character which char pos is currently pointing to
    fn get_cur_char(&self) -> char{
        self.content_vec[self.char_pos as usize]
    }

    pub fn get_next_char(&mut self) -> char{
        // Increment the char pos so we can get the next character in the character array
        self.char_pos += 1;
        // If this is the first time we are running this command, set the line number to be zero
        if self.line_num < 0 {
            self.line_num = 0;
        }
        // If the character position is greater than or equal to the number of characters seen up to the end of that line,
        // then it must be on a new line. Increment the counter
        if self.char_pos >= self.wrd_cnt_per_line[self.line_num as usize] {
            self.line_num += 1;
        }
        self.content_vec[self.char_pos as usize]
    }

    fn peek_ahead_char(&self, k: i32) -> char{
        self.content_vec[(self.char_pos + k) as usize]
    }

    fn peek_next_char(&self) -> char{
        self.peek_ahead_char(1)
    }

    pub fn print_to_screen(&self) {
        println!("{}", self.open_file());
    }
}

pub fn build_cstream() -> CStream {
    let cmd_line_args: Vec<String> = std::env::args().collect();
    if cmd_line_args.len() == 2 {
        let mut new_cstream = CStream {
            file_name : cmd_line_args[1].to_string(),
            line_num : -1,
            char_pos : -1,
            total_char_count : 0,
            content_vec : Vec::new(),
            wrd_cnt_per_line : Vec::new(),
        };
        new_cstream.parse_file();
        return new_cstream;
        
    }
    else {
        panic!("Inusfficent command line arguments");
    }
}

// fn test_case_1(){
//     let mut cstream = build_cstream();
//     cstream.parse_file();
//     cstream.print_to_screen();
//     println!("{} on line {} at position {}", cstream.peek_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.peek_ahead_char(4), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_cur_char(), cstream.line_num, cstream.char_pos);
//     println!("{}", cstream.more_available());
// }

// fn test_case_2(){
//     let mut cstream = build_cstream();
//     cstream.parse_file();
//     println!("{:?}", cstream.wrd_cnt_per_line);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     println!("{} on line {} at position {}", cstream.get_next_char(), cstream.line_num, cstream.char_pos);
//     for n in 0..12{
//         println!("{} on line {} at position {}", cstream.peek_ahead_char(n), cstream.line_num, cstream.char_pos);
//     }
//     println!("{}", cstream.more_available());

// }

// fn tester(){
//     test_case_1();
//     println!("\n");
//     // test_case_2();

// }

// fn main() {
//     tester();
// }
