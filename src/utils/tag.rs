use std::collections::HashSet;

use crate::models::Tag;

pub fn tags_diff_set(tags: Vec<Tag>, not_in: &HashSet<String>) -> HashSet<String> {
    let tags_hashset = tags.into_iter().map(|tag| tag.name).collect::<HashSet<_>>();

    tags_hashset.symmetric_difference(not_in).cloned().collect()
}
