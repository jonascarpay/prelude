pub trait Functor {
    // TODO: Move to trait?
    type Param;
    type Output<B>: Functor<Param = B>;

    fn map<B, F: FnMut(Self::Param) -> B>(self, f: F) -> Self::Output<B>;
}

pub fn map_into<F: Functor, B>(f: F) -> F::Output<B>
where
    B: From<F::Param>,
{
    f.map(|x| x.into())
}

impl<T, const N: usize> Functor for [T; N] {
    type Param = T;

    type Output<B> = [B; N];

    fn map<B, F: FnMut(Self::Param) -> B>(self, f: F) -> Self::Output<B> {
        self.map(f)
    }
}

/* Implementation Template
    impl<T> Functor for MyFunctor<T> {
        type Param = T;
        type Output<B> = MyFunctor<B>;
        fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> MyFunctor<B> {
            MyFunctor {
                x: f(self.x),
                y: f(self.y),
            }
        }
    }
*/
