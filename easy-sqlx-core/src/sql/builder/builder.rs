use std::future::Future;

use sqlx::{Database, Error, Executor, FromRow};

use crate::sql::dialects::page::{PageRequest, PageResult};

pub trait ExecuteBuilder {
    type DB: Database;

    fn execute<C>(
        &self,
        conn: &mut C,
    ) -> impl Future<Output = Result<<Self::DB as Database>::QueryResult, Error>>
    where
        for<'e> &'e mut C: Executor<'e, Database = Self::DB>;
}

pub trait QueryBuilder<'a, O> {
    type DB: Database;

    fn fetch_one<'e, 'c: 'e, E>(self, executor: E) -> impl Future<Output = Result<O, Error>>
    where
        E: 'e + Executor<'c, Database = Self::DB>,
        O: 'e,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: std::marker::Send,
        O: Unpin;

    fn fetch_optional<'e, 'c: 'e, E>(
        self,
        executor: E,
    ) -> impl Future<Output = Result<Option<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;

    fn fetch_all<'e, 'c: 'e, E>(self, executor: E) -> impl Future<Output = Result<Vec<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;

    fn fetch_page<'e, 'c: 'e, E>(self, executor: E, page: &PageRequest) -> impl Future<Output = Result<PageResult<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;
    // A: 'e,
}
