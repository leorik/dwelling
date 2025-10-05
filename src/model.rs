use chrono::{DateTime, Utc};
use std::marker::PhantomData;
use std::sync::Arc;
use uuid::Uuid;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TypedId<T> {
    _type_holder: PhantomData<T>,
    id: Uuid,
}

impl<T> TypedId<T> {
    pub fn new() -> Self {
        TypedId {
            id: Uuid::now_v7(),
            _type_holder: PhantomData,
        }
    }
}

impl<T> From<Uuid> for TypedId<T> {
    fn from(value: Uuid) -> Self {
        TypedId {
            id: value,
            _type_holder: PhantomData,
        }
    }
}

impl<T> AsRef<Uuid> for TypedId<T> {
    fn as_ref(&self) -> &Uuid {
        &self.id
    }
}

pub struct Post {
    pub id: TypedId<Post>,
    pub author_id: TypedId<Account>,
    pub thread_id: TypedId<Thread>,
    pub created: DateTime<Utc>,
    pub body: Arc<String>,
}

pub struct Thread {
    pub id: TypedId<Thread>,
    pub title: String,
    pub hru: String,
}

pub struct Account {
    pub id: TypedId<Account>,
    pub name: String,
}
