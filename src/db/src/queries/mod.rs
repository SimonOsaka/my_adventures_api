use sqlx::Arguments;

pub mod adventures;

#[cfg(any(feature = "mysql"))]
type SqlArguments1 = sqlx::mysql::MySqlArguments;

#[cfg(any(feature = "postgres"))]
type SqlArguments = sqlx::postgres::PgArguments;

struct SqlParam {
    index: u8,
    placeholder: String,
    args: SqlArguments,
}

impl SqlParam {
    fn new() -> SqlParam {
        SqlParam { index: 1, placeholder: String::from(""), args: SqlArguments::default() }
    }

    fn value<'q, T: 'q + Send + sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>>(
        &mut self,
        value: T,
    ) -> String {
        if cfg!(feature = "postgres") {
            self.placeholder = format!("${:?}", self.index);
            self.index = self.index + 1;
        } else {
            self.placeholder = "?".to_owned();
        }
        self.args.add(value);
        self.placeholder.clone()
    }

    fn fetch_args(self) -> SqlArguments {
        self.args
    }
}
