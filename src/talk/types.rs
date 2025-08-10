pub struct Talk {
    pub reply_fn: Box<dyn Fn(&str) -> (String, bool) + Send + Sync>,
}

impl Talk {
    pub fn new<F>(reply_fn: F) -> Self
    where
        F: Fn(&str) -> (String, bool) + Send + Sync + 'static,
    {
        Talk {
            reply_fn: Box::new(reply_fn),
        }
    }

    pub fn reply(&self, input: &str) -> (String, bool) {
        (self.reply_fn)(input)
    }
}
