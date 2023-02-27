use crate::{
    configs::db::Connection,
    schema::cookies::{self, dsl::*},
};
use cookie::{Cookie, ParseError};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct DieselCookie {
    id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = cookies)]
pub struct DieselCookieDto {
    name: String,
    value: String,
}

impl DieselCookie {
    pub fn insert(cookie_list: &Vec<DieselCookieDto>, conn: &mut Connection) -> QueryResult<usize> {
        let mut rows_inserted = 0;
        for c in cookie_list {
            rows_inserted += diesel::insert_into(cookies)
                .values(c)
                .on_conflict(name)
                .do_update()
                .set(value.eq(&c.value))
                .execute(conn)?;
        }

        Ok(rows_inserted)
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<DieselCookie>> {
        cookies.load::<DieselCookie>(conn)
    }
}

impl DieselCookieDto {
    pub fn new(cookie: &str) -> Result<DieselCookieDto, ParseError> {
        let c = Cookie::parse(cookie)?;

        Ok(DieselCookieDto {
            name: c.name().to_string(),
            value: c.value().to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        let cookie = Cookie::parse("asd=123;").unwrap();
        println!("{:#?}", cookie);
    }
}
