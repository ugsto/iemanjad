SELECT *, string::split(<string>id, ':')[1] AS id FROM tags WHERE name = $tag_name
