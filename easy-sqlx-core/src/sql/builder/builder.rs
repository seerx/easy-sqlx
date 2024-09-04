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

pub trait QueryBuilder<'a> {
    type DB: Database;

    fn fetch_one<'e, 'c: 'e, E, O>(self, executor: E) -> impl Future<Output = Result<O, Error>>
    where
        E: 'e + Executor<'c, Database = Self::DB>,
        O: 'e,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: std::marker::Send,
        O: Unpin;

    fn fetch_optional<'e, 'c: 'e, E, O>(
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

    fn fetch_all<'e, 'c: 'e, E, O>(
        self,
        executor: E,
    ) -> impl Future<Output = Result<Vec<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;

    fn fetch_page<'e, 'c: 'e, E, O>(
        &self,
        executor: E,
        page: &PageRequest,
    ) -> impl Future<Output = Result<PageResult<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB> + 'c + Copy,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;

    fn count<'c, E>(&self, executor: E) -> impl Future<Output = Result<usize, Error>>
    where
        E: 'c + Executor<'c, Database = Self::DB>;

    fn fetch_one_scalar<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<O, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;

    fn fetch_option_scalar<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<Option<O>, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;

    fn fetch_all_scalars<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<Vec<O>, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;
    // A: 'e,
}
