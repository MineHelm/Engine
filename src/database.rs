use std::ops::Deref;

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DB(PgPool);

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub fn pool(&self) -> &PgPool { &self.0 }
}

impl Deref for DB {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
