DEFINE TABLE tags SCHEMAFULL;

DEFINE FIELD name ON TABLE tags TYPE string;
DEFINE INDEX unique_name ON TABLE tags FIELDS name UNIQUE;
