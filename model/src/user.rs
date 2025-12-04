use uuid::Uuid;

pub struct User {
    id: Uuid,
}

impl User {
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self { id }
    }

    pub fn new_from(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}
