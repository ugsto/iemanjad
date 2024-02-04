SELECT *, string::split(<string>id, ':')[1] AS id FROM (UPDATE tags SET name = $tag_new_name WHERE name = $tag_old_name)
