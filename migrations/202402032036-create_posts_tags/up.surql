DEFINE TABLE posts_tags SCHEMAFULL;

DEFINE FIELD in ON posts_tags TYPE record<posts>;
DEFINE FIELD out ON posts_tags TYPE record<tags>;

DEFINE INDEX unique_relationships ON TABLE posts_tags COLUMNS in, out UNIQUE;
