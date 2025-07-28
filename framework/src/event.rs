use serde::{Deserialize, Serialize};

pub trait Event: Serialize + for<'de> Deserialize<'de> + Send + Sync {}
