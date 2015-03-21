use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub struct Board {
    pub squares: Vec<Square>,
    pub size: usize,
    pub root: usize,
}

impl Board {
    pub fn new() -> Board {
        let mut sqs = Vec::with_capacity(81);
        for _ in 0..81 {
            sqs.push(Square::new());
        }

        Board {
            squares: sqs,
            size: 9,
            root: 3,
        }
    }


    pub fn parse(serialized_board: String) -> Board {
        let mut initial = serialized_board.chars();
        let mut sqs = Vec::with_capacity(81);
        for _ in 0..81 {
            let sq = match initial.next() {
                Some('-') => { Square::new() },
                Some(val) => { Square::known(val.to_digit(10).unwrap() as u8) },
                None => { Square::new() },
            };
            sqs.push(sq);
        }

        Board {
            squares: sqs,
            size: 9,
            root: 3,
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let root = self.root;
        let mut sqs = self.squares.iter();
        for y_chunk in (0..root) {
            for _ in (0..root) {
                for x_chunk in (0..root) {
                    for _ in (0..root) {
                        try!(write!(f, "{:?}", sqs.next().unwrap()))
                    }
                    if x_chunk < root - 1 {
                        try!(write!(f, "|", ))
                    }
                }
                try!(write!(f, "\n" ))
            }
            if y_chunk < root - 1 {
                try!(write!(f, "===+===+===\n" ))
            }
        }
        Ok(())
    }
}

pub enum Square {
    Known(u8),
    Guess(Vec<u8>),
}

impl Square {

    pub fn new() -> Square {
        Square::Guess(vec![1,2,3,4,5,6,7,8,9])
    }

    pub fn known(val: u8) -> Square {
        Square::Known(val)
    }

}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            &Square::Known(ref val) => {
                write!(f, "{:?}", *val)
            },
            &Square::Guess(_) => {
                write!(f, "-" )
            },
        }

    }
}
