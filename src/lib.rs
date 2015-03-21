#[derive(Debug)]
pub struct Board {
    pub squares: Vec<Square>,
    pub size: u8,
}

impl Board {
    pub fn new() -> Board {
        let mut sqs = Vec::with_capacity(81);
        for _ in 0..81 {
            sqs.push(Square::new());
        }

        Board {
            squares: sqs,
            size: 9
        }
    }
}

#[derive(Debug)]
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
