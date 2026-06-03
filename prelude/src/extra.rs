pub trait BoolExt {
    /// Chosen to mirror the functionality of Result::ok.
    fn ok(self) -> Option<()>;
    fn or_err<E>(self, err: E) -> Result<(), E>;
    fn then_else<T>(self, t: T, f: T) -> T;
}

impl BoolExt for bool {
    fn ok(self) -> Option<()> {
        self.then_some(())
    }
    fn or_err<E>(self, err: E) -> Result<(), E> {
        if self {
            Ok(())
        } else {
            Err(err)
        }
    }
    fn then_else<T>(self, t: T, f: T) -> T {
        if self {
            t
        } else {
            f
        }
    }
}
