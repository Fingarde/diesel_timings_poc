use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Debugable: Sized {
    fn debug(self) -> DebugQuery<Self>;
}

impl<T> Debugable for T {
    fn debug(self) -> DebugQuery<Self> {
        DebugQuery { inner: self }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct DebugQuery<T> {
    inner: T,
}

impl<T> DebugQuery<T> {

    pub fn exec<'a, U>(self, conn: &mut PgConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'a, PgConnection, U>,
    {
        let results = self.load::<U>(conn)?;
        Ok(results)
    }
}

impl<T: Query> Query for DebugQuery<T> {
    type SqlType = T::SqlType;
}


impl<T> RunQueryDsl<PgConnection> for DebugQuery<T> {

}


impl<T> QueryFragment<Pg> for DebugQuery<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        let uuid = uuid::Uuid::new_v4();
        self.inner.walk_ast(out.reborrow())?;
        out.push_sql(format!("; -- QUERY_ID={}", uuid).as_str());
        Ok(())
    }

    
}