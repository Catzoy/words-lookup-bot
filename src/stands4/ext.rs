use crate::stands4::entities::AbbreviationDefinition;
use std::collections::HashMap;
use std::ops::Not;

pub trait SliceAbbreviationsExt {
    const UNFILLED: &'static str = "UNFILED";
    fn categorized(&self) -> Vec<(&str, Vec<&AbbreviationDefinition>)>;
}

impl SliceAbbreviationsExt for [AbbreviationDefinition] {
    fn categorized(&self) -> Vec<(&str, Vec<&AbbreviationDefinition>)> {
        let categorized = &mut self
            .iter()
            .filter(|def| def.category.eq(Self::UNFILLED).not())
            .fold(
                HashMap::<&str, Vec<&AbbreviationDefinition>>::new(),
                |mut map, def| {
                    let category = def.category.as_str();
                    match map.get_mut(category) {
                        Some(v) => {
                            v.push(def);
                        }
                        None => {
                            map.insert(category, vec![def]);
                        }
                    };
                    map
                },
            );

        let mut common = categorized.drain().collect::<Vec<_>>();
        common.sort_by(|(_, v1), (_, v2)| v2.len().cmp(&v1.len()));
        common
    }
}
