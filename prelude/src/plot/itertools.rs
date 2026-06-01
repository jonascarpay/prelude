pub struct StopsAt<I: Iterator> {
    iter: I,
    end: I::Item,
    done: bool,
}

impl<I: Iterator> StopsAt<I> {
    pub fn new(iter: I, end: I::Item) -> Self {
        Self { iter, end, done: false }
    }
}
impl<I> Iterator for StopsAt<I>
where
    I: Iterator,
    I::Item: PartialEq + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            return None;
        }
        let item = self.iter.next()?;
        if item == self.end {
            self.done = true;
        }
        Some(item)
    }
}

pub struct StopsBefore<I: Iterator> {
    iter: I,
    end: I::Item,
    done: bool,
}

impl<I: Iterator> StopsBefore<I> {
    pub fn new(iter: I, end: I::Item) -> Self {
        Self { iter, end, done: false }
    }
}

impl<I> Iterator for StopsBefore<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            return None;
        }
        let item = self.iter.next()?;
        if item == self.end {
            self.done = true;
            None
        } else {
            Some(item)
        }
    }
}
