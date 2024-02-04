use surrealdb::Surreal;

pub const MIGRATIONS: &[&str] = &[
    include_str!("../migrations/202402032031-create_tags/up.surql"),
    include_str!("../migrations/202402032035-create_posts/up.surql"),
    include_str!("../migrations/202402032036-create_posts_tags/up.surql"),
];

pub async fn exec_migrations(db: &Surreal<surrealdb::engine::any::Any>, migrations: &[&str]) {
    for migration in migrations {
        db.query(migration.to_string()).await.unwrap();
    }
}
