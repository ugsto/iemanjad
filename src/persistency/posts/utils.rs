use super::models::SurrealPostEntityInput;

pub fn create_post_entity(
    title: String,
    content: String,
    now: chrono::DateTime<chrono::Utc>,
) -> SurrealPostEntityInput {
    SurrealPostEntityInput {
        title,
        content,
        created_at: surrealdb::sql::Datetime(now),
        updated_at: surrealdb::sql::Datetime(now),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_post_entity() {
        let now = chrono::Utc::now();
        let post_entity = create_post_entity("title".to_string(), "content".to_string(), now);

        assert_eq!(post_entity.title, "title");
        assert_eq!(post_entity.content, "content");
        assert_eq!(post_entity.created_at, surrealdb::sql::Datetime(now));
        assert_eq!(post_entity.updated_at, surrealdb::sql::Datetime(now));

        assert!(serde_json::to_string(&post_entity).is_ok());

        assert_eq!(
            serde_json::to_string(&post_entity).unwrap(),
            serde_json::to_string(&SurrealPostEntityInput {
                title: "title".to_string(),
                content: "content".to_string(),
                created_at: surrealdb::sql::Datetime(now),
                updated_at: surrealdb::sql::Datetime(now),
            })
            .unwrap()
        );
    }
}
