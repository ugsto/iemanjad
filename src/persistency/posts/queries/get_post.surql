SELECT *, string::split(<string>id, ':')[1] AS id, (SELECT *, string::split(<string>id, ':')[1] AS id FROM ->posts_tags->tags.*) AS tags FROM <record>$post_id
