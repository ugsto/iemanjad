SELECT *, string::split(<string>id, ':')[1] AS id FROM (CREATE tags CONTENT $tag)
