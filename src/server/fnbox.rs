pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce() + Send> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)();
    }
}
