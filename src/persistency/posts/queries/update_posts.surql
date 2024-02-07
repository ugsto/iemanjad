SELECT *, string::split(<string>id, ':')[1] AS id FROM (UPDATE (<record>$id) SET title = $title, content = $content)
