pub struct Rcn<'a> {
    pub commands: Option<Commands<'a>>,
}

pub enum Commands<'a> {
    Models { name: &'a str, ts_js: &'a str },
    Help,
    Version,
}
