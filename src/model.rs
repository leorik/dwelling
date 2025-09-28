use chrono::{DateTime, Utc};
use uuid::Uuid;

macro_rules! make_id_type {
    ($typeName:ident) => {
        pub struct $typeName {
            inner: Uuid,
        }

        impl From<Uuid> for $typeName {
            fn from(value: Uuid) -> Self {
                Self { inner: value }
            }
        }

        impl AsRef<Uuid> for $typeName {
            fn as_ref(&self) -> &Uuid {
                &self.inner
            }
        }
    };
}
make_id_type!(PostId);
make_id_type!(ThreadId);
make_id_type!(AccountId);

pub struct Post {
    pub id: PostId,
    pub author_id: AccountId,
    pub thread_id: ThreadId,
    pub timestamp: DateTime<Utc>,
    pub body: String,
}

pub struct Thread {
    pub id: ThreadId,
    pub title: String,
}
