use crate::server::fnbox::FnBox;
pub type Job = Box<dyn FnBox + Send + 'static>;
