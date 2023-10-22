use std::collections::HashMap;

// The keywords of language X are: unsigned, char, short, int, long, float, double, while, if,
// return, void, and main.

enum EntryType {
    Unsigned,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    While,
    If,
    Return,
    Void,
    Main,
}

struct Entry {
    type: EntryType,
}

pub struct SymbolSection{
    scope: &str,
    symbols: HashMap<&str, Entry>,
}

pub struct SymbolTable{
    symbol_section_stack: Vec<SymbolSection>,
}


impl SymbolSection{
    pub fn new(scope_intput: &str) -> Self {
        Self {
            scope: scope_input,
            symbols: HashMap::new();
        }
    }
}


impl SymbolTable{
    pub fn new() -> Self{
        Self {
            symbol_section_stack: Vec::new();
        }
    }
}