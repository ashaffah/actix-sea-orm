use sea_orm::DatabaseConnection;

pub struct AppsConfig {
    pub db: DatabaseConnection,
    pub templates: tera::Tera,
}

impl AppsConfig {
    pub fn new(db: DatabaseConnection, templates: tera::Tera) -> Self {
        Self {
            db,
            templates,
        }
    }
}
impl Clone for AppsConfig {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            templates: self.templates.clone(),
        }
    }
}
