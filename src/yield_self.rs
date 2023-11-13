pub trait YieldSelf
where
    Self: Sized,
{
    #[inline(always)]
    fn yield_self<T>(self, func: impl FnOnce(&Self) -> T) -> T {
        func(&self)
    }
}

impl<T> YieldSelf for T where T: Sized {}
