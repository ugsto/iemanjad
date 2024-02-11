SELECT *, string::split(<string>id, ':')[1] AS id, (SELECT *, string::split(<string>id, ':')[1] AS id FROM ->posts_tags->tags.*) AS tags FROM posts ORDER BY created_at LIMIT $limit START $offset
