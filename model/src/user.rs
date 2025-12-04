use uuid::Uuid;

pub struct User {
    id: Uuid,
}

impl User {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}
