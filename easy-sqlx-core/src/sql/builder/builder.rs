use std::future::Future;

use futures::stream::BoxStream;
use sqlx::{Database, Error, Executor};

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

    fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<O, Error>>
    where
        E: 'e + Executor<'c, Database = Self::DB>,
        O: 'e;

    // fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<<Self::DB as Database>::Row, Error>>
    // where
    //     'a: 'e,
    //     E: Executor<'c, Database = Self::DB>;
    // for<'e> &'e mut E: Executor<'c, Database = Self::DB>;
}

fn t() {
    // sqlx::query_as("").fetch(executor)
}
