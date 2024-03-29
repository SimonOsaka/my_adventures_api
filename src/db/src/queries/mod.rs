use sqlx::Arguments;

pub mod adventures;

#[cfg(any(feature = "mysql"))]
type SqlArguments = sqlx::mysql::MySqlArguments;

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

#[cfg(test)]
mod tests {
    use crate::queries::SqlParam;
    use sql_builder::SqlBuilder;

    #[test]
    fn sql_delete_param() {
        let mut sql_param = SqlParam::new();
        let mut sql_builder = SqlBuilder::delete_from("table");

        let sql = sql_builder.and_where_eq("id", sql_param.value(123)).sql().unwrap();
        debug!("{:?}", &sql);
        assert_eq!(sql, "DELETE FROM table WHERE id = $1;");
    }
}
