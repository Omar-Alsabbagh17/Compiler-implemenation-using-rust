use crate::cstream::CStream;


// Will convert chars gathered from cstream into primitive types
// Primitive types are alpha (letter), digit, space, newline, and special
#[derive(PartialEq)]

pub enum PrimitiveType {
    Alpha,
    Digit,
    Space,
    NewLine,
    Operator,
    UnderScore,
    Negative,
    Dot,
    Eof,
}

// Implement <Primtive> == <PrimitiveType> comparisons
impl PartialEq<PrimitiveType> for Primitive {
    fn eq(&self, other: &PrimitiveType) -> bool {
        self.prim_type == *other
    }
}

// Implement <PrimitiveType> == <Primitive> comparisons
impl PartialEq<Primitive> for PrimitiveType {
    fn eq(&self, other: &Primitive) -> bool {
        *self == other.prim_type
    }
}

impl std::fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alpha => write!(f, "Alpha"),
            Self::Digit => write!(f, "Digit"),
            Self::Space => write!(f, "Space"),
            Self::NewLine => write!(f, "NewLine"),
            Self::Operator => write!(f, "Operator"),
            Self::UnderScore => write!(f, "Underscore"),
            Self::Negative => write!(f, "Negative"),
            Self::Dot => write!(f, "Dot"),
            Self::Eof => write!(f, "End of File"),
        }
    }
}

pub struct Primitive {
    pub symbol: char,
    pub prim_type : PrimitiveType,
}

impl Primitive {
    fn new(sym: char, ty: PrimitiveType) -> Self{
        Self {
            symbol: sym,
            prim_type: ty,
        }
    }

    pub fn print_prim(&self){
        println!("{} {}", self.symbol, self.prim_type);
    }
}
pub struct PrimitiveCollection{
    all_prims: Vec<Primitive>,
    curr_prim_idx: i32,
    all_prims_len: usize,
}

impl PrimitiveCollection{
    pub fn new() -> Self {
        Self {
            all_prims: Vec::new(),
            curr_prim_idx: -1,
            all_prims_len: 0,
        }
    }

    pub fn create_prims(&mut self, mut stream: CStream){
        stream.parse_file();
        while stream.more_available() {
            let curr = stream.get_next_char();
            let curr_type = self.match_primitive(&curr);
            self.all_prims.push(Primitive::new(curr, curr_type));
        }
        self.all_prims.push(Primitive::new('`', PrimitiveType::Eof));
        self.all_prims_len = self.all_prims.len();
        
    }


    pub fn more_available(&self) -> bool{
        self.curr_prim_idx + 1 < (self.all_prims_len as i32)
    }

    pub fn get_next_prim(&mut self) -> &Primitive {
        self.curr_prim_idx += 1;
        &self.all_prims[self.curr_prim_idx as usize]
    }

    pub fn get_curr_prim(&self) -> &Primitive {
        &self.all_prims[self.curr_prim_idx as usize]
    }

    pub fn peek_next_prim(&self) -> &Primitive {
        &self.all_prims[(self.curr_prim_idx+1) as usize]
    }

    fn match_primitive(&self, symb: &char) -> PrimitiveType{
        match symb {
            'A'..='Z' | 'a'..='z' => {
                return PrimitiveType::Alpha;
            },
            '0'..='9' => {
                return PrimitiveType::Digit;
            },
            ' ' => {
                return PrimitiveType::Space;
            },
            '\n' => {
                return PrimitiveType::NewLine;
            },
            '_' => {
                return PrimitiveType::UnderScore;
            },
            '.' => {
                return PrimitiveType::Dot;
            },
            '-' => {
                return PrimitiveType::Negative;
            },
            _ => {
                return PrimitiveType::Operator;
            }
        }
    }


}
