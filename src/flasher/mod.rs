use serde::ser::{Serialize, Serializer, SerializeStruct};

use rocket::request::FlashMessage;
use std::ops::Deref;

pub struct Flasher(pub FlashMessage);

impl Deref for Flasher {
    type Target = FlashMessage;
    fn deref(&self) -> &FlashMessage {
        &self.0
    }
}

impl Serialize for Flasher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FlashMessage", 2)?;
        state.serialize_field("name", self.name())?;
        state.serialize_field("msg", self.msg())?;
        state.end()
    }
}
