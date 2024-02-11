SELECT *, string::split(<string>id, ':')[1] AS id FROM tags ORDER BY name LIMIT $limit START $offset
