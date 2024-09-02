use std::future::Future;

use sqlx::{Database, Error, Executor};

pub trait Builder {
    type DB: Database;

    fn execute<C>(
        &self,
        conn: &mut C,
    ) -> impl Future<Output = Result<<Self::DB as Database>::QueryResult, Error>>
    where
        for<'e> &'e mut C: Executor<'e, Database = Self::DB>;
}
