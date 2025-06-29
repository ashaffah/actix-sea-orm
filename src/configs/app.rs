use sea_orm::DatabaseConnection;

pub struct AppsConfig {
    pub db: DatabaseConnection,
}

impl AppsConfig {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
        }
    }
}
impl Clone for AppsConfig {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
        }
    }
}
