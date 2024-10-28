use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> DebugQuery<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> DebugQuery<Self> {
        println!("Paginating to page {}", page);
        DebugQuery { inner: self }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct DebugQuery<T> {
    inner: T,
}

impl<T> DebugQuery<T> {
    pub fn per_page(self) -> Self {
        DebugQuery {
            ..self
        }
    }

    pub fn load_and_count_pages<'a, U>(self, conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<'a, PgConnection, (U, i64)>,
    {
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok((records, total))
    }
}

impl<T: Query> Query for DebugQuery<T> {
    type SqlType = (T::SqlType, BigInt);
}


impl<T> RunQueryDsl<PgConnection> for DebugQuery<T> {}


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