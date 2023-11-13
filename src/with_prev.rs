#[derive(Clone, Debug)]
pub struct WithPrev<I: Iterator> {
    iter: I,
    prev: Option<I::Item>,
}
impl<I: Iterator> WithPrev<I> {
    pub fn new(iter: I) -> WithPrev<I> {
        WithPrev { iter, prev: None }
    }
}

impl<I> Iterator for WithPrev<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (<I as Iterator>::Item, <I as Iterator>::Item);

    #[inline]
    fn next(&mut self) -> Option<(<I as Iterator>::Item, <I as Iterator>::Item)> {
        let mut elem = self.iter.next()?;
        let mut prev = self.prev.take();

        if prev.is_none() {
            prev = Some(elem);
            elem = self.iter.next()?;
        }

        self.prev = Some(elem.clone());
        prev.map(|prev_elem| (prev_elem, elem))
    }
}

pub trait IteratorWithPrev: Iterator {
    fn with_prev(self) -> WithPrev<Self>
    where
        Self: Sized,
    {
        WithPrev::new(self)
    }
}

impl<T: Iterator> IteratorWithPrev for T {}
