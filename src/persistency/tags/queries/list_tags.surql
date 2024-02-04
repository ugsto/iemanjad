SELECT *, string::split(<string>id, ':')[1] AS id FROM tags LIMIT $limit START $offset
