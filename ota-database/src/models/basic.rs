use rand::{distributions::*, thread_rng, Rng};

use crate::db::DatabaseError;

pub fn random_string(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    return rand_string;
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(min..max);
    number
}

pub fn random_i32() -> i32 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(i32::MIN..i32::MAX);
    number
}

pub fn random_i64() -> i64 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(i64::MIN..i64::MAX);
    number
}

use sqlx::PgPool;

pub trait HasId {
    fn id(&self) -> i32;
}

#[async_trait::async_trait]
pub trait CrudOperations<T0, Tn, Tu> {
    async fn all(pool: &PgPool) -> Result<Vec<T0>, DatabaseError>;
    async fn find(id: i32, pool: &PgPool) -> Result<T0, DatabaseError>;
    async fn create(data: Tn, pool: &PgPool) -> Result<T0, DatabaseError>;
    async fn update(id: i32, data: Tu, pool: &PgPool) -> Result<T0, DatabaseError>;
    async fn delete(id: i32, pool: &PgPool) -> Result<u64, DatabaseError>;
}
