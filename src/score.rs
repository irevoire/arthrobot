use std::{collections::BTreeMap, ops::{Deref, DerefMut}};
use std::sync::{Arc};

use serenity::{model::id::UserId, prelude::{Mutex, TypeMapKey}};
// #[derive(Serialize, Deserialize)]

#[derive(Default)]
pub struct Score(pub BTreeMap<UserId, isize>);

impl Deref for Score {
    type Target = BTreeMap<UserId, isize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Score {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TypeMapKey for Score {
    type Value = Arc<Mutex<Self>>;
}

impl Score {
    pub fn typemapkey_score() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }
}
