DELETE posts_tags WHERE ->(tags WHERE id NOT IN (<array<record>>$tag_ids)) AND <-(posts WHERE id = (<record>$post_id));
RELATE (<record>$post_id)->posts_tags->(SELECT id FROM tags WHERE id IN (<array<record>>$tag_ids) AND NOT(<-posts_tags<-(posts WHERE id = (<record>$post_id))));
