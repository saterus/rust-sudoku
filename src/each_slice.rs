
pub struct EachSlicer<I> where I: Iterator {
    iter: I,
    per_slice: usize
}

impl<I> Iterator for EachSlicer<I> where I: Iterator {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        let mut slice: Vec<I::Item> = Vec::with_capacity(self.per_slice);

        for _ in (0..self.per_slice) {
            match self.iter.next() {
                Some(item) => { slice.push(item) },
                None => { break; },
            }
        }

        if slice.len() > 0 {
            Some(slice)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower,upper) = self.iter.size_hint();
        let lower_limit = ((lower as f32) / (self.per_slice as f32)).floor() as usize;
        let upper_limit = upper.map(|u| ((u as f32) / (self.per_slice as f32)).ceil() as usize );

        (lower_limit, upper_limit)
    }

}


impl<I> EachSlice for I where I: Iterator {}

pub trait EachSlice: Iterator {

    /// `each_slice` yields N elements from an iterator per `next()`.
    ///
    /// ```
    /// use sudoku::each_slice::EachSlice;
    ///
    /// let v = vec![1i32,2,3,4,5,6,7,8,9,10];
    /// for s in v.iter().each_slice(3) {
    ///     println!("s = {:?}", s);
    /// }
    /// // =>
    /// // s = [1, 2, 3]
    /// // s = [4, 5, 6]
    /// // s = [7, 8, 9]
    /// // s = [10]
    /// ```
    ///
    /// **panics** if per_slice is zero.
    fn each_slice(self, per_slice: usize) -> EachSlicer<Self> where Self: Sized {
        assert!(per_slice > 0, "Must slice 1 or more elements!");

        EachSlicer {
            iter: self,
            per_slice: per_slice
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::EachSlice;

    #[test]
    fn produces_groups_test() {
        let v = [1,2,3,4];
        let mut pairs = v.iter().each_slice(2);
        assert!(pairs.next() == Some(vec![&1,&2]));
        assert!(pairs.next() == Some(vec![&3,&4]));
    }

    #[test]
    fn produces_remainder_test() {
        let v = [1,2,3];
        let mut pairs = v.iter().each_slice(2);
        assert!(pairs.next() == Some(vec![&1,&2]));
        assert!(pairs.next() == Some(vec![&3]));
    }

    #[test]
    fn drains_iter_test() {
        let v = [1,2];
        let mut pairs = v.iter().each_slice(2);
        assert!(pairs.next() == Some(vec![&1,&2]));
        assert!(pairs.next() == None);
        assert!(pairs.next() == None);
    }

    #[test]
    fn empty_iter_test() {
        let v: [u32; 0] = [];
        let mut pairs = v.iter().each_slice(2);
        assert!(pairs.next() == None);
    }
}
