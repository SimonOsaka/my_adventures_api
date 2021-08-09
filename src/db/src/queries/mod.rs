pub mod adventures;

#[derive(Clone, Debug)]
struct SqlParam {
    index: u8,
    placeholder: String,
}

impl SqlParam {
    fn new() -> SqlParam {
        SqlParam { index: 1, placeholder: String::from("") }
    }

    fn sql_placeholder(&mut self) -> String {
        self.placeholder = format!("${:?}", self.index);
        self.index = self.index + 1;
        self.placeholder.clone()
    }
}
