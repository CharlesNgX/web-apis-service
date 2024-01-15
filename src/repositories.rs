use crate::schema::*;
use crate::models::*;
use diesel::prelude::*;
pub struct RustaceanRepository { }

impl RustaceanRepository {

    pub fn find_mutilple(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
        .limit(limit)
        .order(rustaceans::id.desc())
        .load(conn)
    }

    pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table
        .find(id)
        .get_result::<Rustacean>(conn)
    }

    pub fn create(conn: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
        .values(&new_rustacean)
        .get_result::<Rustacean>(conn)
    }

    pub fn save(conn: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
        .set((
            rustaceans::email.eq(rustacean.email.to_owned()),
            rustaceans::name.eq(rustacean.name.to_owned()),
        ))
        .get_result::<Rustacean>(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
        .execute(conn)
    }

}

pub struct CrateReposity { }

impl CrateReposity {

    pub fn find_mutilple(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(conn)
    }

    pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table
            .find(id)
            .get_result::<Crate>(conn)
    }

    pub fn create(conn: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(&new_crate)
            .get_result::<Crate>(conn)
    }

    pub fn save(conn: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::code.eq(a_crate.code.to_owned()),
                crates::name.eq(a_crate.name.to_owned()),
                crates::version.eq(a_crate.version.to_owned()),
                crates::description.eq(a_crate.description.to_owned()),
                crates::rustaceans_id.eq(a_crate.rustaceans_id.to_owned()),
            ))
            .get_result::<Crate>(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table
            .find(id))
            .execute(conn)
    }

}