class CStream:
    # read in a file, character by character.

    def __init__(self, file_name) -> None:
        self._line_num,  self._char_pos = -1, -1
        with open(file_name, 'r') as f:
            self._data = f.read()
            f.close()
        

    
    #========= internal function ===========
    def _read_char(self, update:bool, peak_ahead=0):
        
        if self._char_pos == -1:
            char = self._data[peak_ahead]
            if update:
                self._line_num = 1
                self._char_pos = 0
            return char
       
        if (peak_ahead):
            char =  self._data[self._char_pos + peak_ahead]
        else:
            char = self._data[self._char_pos+1]
        if update:
            self._char_pos += 1
        if (char == '\n'):
            char =  '\\n'  # make end-of-line character be visualized at the print
            if update:
                self._line_num += 1
        return char
    #================================
    def more_available(self) -> bool: 
        return ( self._char_pos < len(self._data)-1)
        
    def peek_next_char(self):
        return self._read_char(update= False)
        
    def peek_ahead_char(self, n:int):
        return self._read_char(update=False , peak_ahead=n+1)
        
    def get_next_char(self):
        return self._read_char(update=True)
        
    def get_cur_char(self):
        if (self._data[self._char_pos] == '\n'):
            return '\\n'
        return self._data[self._char_pos]
    
    def get_char_pos(self) -> int:
        return self._char_pos
        
    def get_line_num(self) -> int:
        return self._line_num
        
    
#============= main ==================

if __name__ == "__main__":
    obj = CStream("myfile.txt")
    print(obj.peek_next_char())
    print(obj.get_next_char())
    print(obj.peek_ahead_char(4))
    print(obj.get_next_char())
    print(obj.get_next_char())
    print(obj.get_next_char())
    print(obj.get_cur_char())
    print(obj.more_available())


    
    