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
    /// 获取一条记录
    fn one<'e, 'c: 'e, E, O>(self, executor: E) -> impl Future<Output = Result<O, Error>>
    where
        E: 'e + Executor<'c, Database = Self::DB>,
        O: 'e,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: std::marker::Send,
        O: Unpin;
    /// 获取一条记录，如果不存在返回 None
    fn optional<'e, 'c: 'e, E, O>(
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

    /// 获取全部记录
    fn all<'e, 'c: 'e, E, O>(self, executor: E) -> impl Future<Output = Result<Vec<O>, Error>>
    where
        // 'q: 'e,
        E: 'e + Executor<'c, Database = Self::DB>,
        for<'r> O: FromRow<'r, <Self::DB as Database>::Row>,
        O: 'e,
        O: std::marker::Send,
        O: Unpin;

    /// 分页查询
    fn page<'e, 'c: 'e, E, O>(
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

    /// 查询记录数
    fn count<'c, E>(&self, executor: E) -> impl Future<Output = Result<usize, Error>>
    where
        E: 'c + Executor<'c, Database = Self::DB>;

    /// 获取一个标量
    fn one_scalar<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<O, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;
    /// 获取一个可选标量，如果不存在返回 None
    fn optional_scalar<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<Option<O>, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;
    /// 获取全部标量
    fn all_scalars<'q, 'c, E, O>(
        &self,
        executor: E,
        field: &'q str,
    ) -> impl Future<Output = Result<Vec<O>, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'c + Executor<'c, Database = Self::DB>,
        O: Send + Unpin;

    /// 分页获取标量
    fn page_scalars<'e, 'c: 'e, E, O>(
        &self,
        executor: E,
        field: &'c str,
        page: &PageRequest,
    ) -> impl Future<Output = Result<PageResult<O>, Error>>
    where
        (O,): for<'r> FromRow<'r, <Self::DB as Database>::Row>,
        E: 'e + Executor<'c, Database = Self::DB> + 'c + Copy,
        O: Send + Unpin;
}
