use lazycell::AtomicLazyCell;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as PgPool, PooledConnection},
};

static GLOBAL_PG: GlobalPostgres = GlobalPostgres {
    inner: AtomicLazyCell::NONE,
};

pub struct GlobalPostgres {
    inner: AtomicLazyCell<Postgres>,
}

impl GlobalPostgres {
    pub fn initialize(url: impl Into<String>) -> anyhow::Result<()> {
        GLOBAL_PG
            .inner
            .fill(Postgres::new(url))
            .map_err(|_| anyhow::anyhow!("GlobalPostgres is already initialized"))
    }

    pub fn get() -> Postgres {
        GLOBAL_PG.inner.borrow().unwrap().clone()
    }
}

/*
 * =========
 * Internals
 * =========
 */
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Postgres {
    pool: PgPool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    pub fn new(url: impl Into<String>) -> Postgres {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = PgPool::new(manager).unwrap();
        Postgres { pool }
    }

    pub async fn with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || Ok(func(pool.get()?))).await?
    }

    pub async fn try_with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.with_conn(func).await?
    }
}
