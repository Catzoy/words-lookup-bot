use crate::format::{LookupFormatter, StringBuilderExt};
use crate::stands4::SynAntDefinitions;
use string_builder::Builder;

pub trait SynAntFormatterExt<R> {
    fn push_syn_ant(builder: &mut Builder, def: &SynAntDefinitions, on_empty: fn() -> String);
}

impl<L, R> SynAntFormatterExt<R> for L
where
    L: LookupFormatter<R>,
{
    fn push_syn_ant(builder: &mut Builder, def: &SynAntDefinitions, on_empty: fn() -> String) {
        let mut cmds: Vec<Box<dyn FnMut(&mut Builder)>> = vec![];
        if !def.synonyms.is_empty() {
            let handler = |builder: &mut Builder| {
                builder.append("*Synonyms*: ");
                builder.list_words(&def.synonyms);
                builder.append("\n");
            };
            cmds.push(Box::new(handler));
        }
        if !def.antonyms.is_empty() {
            let handler = |builder: &mut Builder| {
                builder.append("*Antonyms*: ");
                builder.list_words(&def.antonyms);
                builder.append("\n");
            };
            cmds.push(Box::new(handler));
        }
        if cmds.is_empty() {
            builder.append(on_empty())
        } else {
            for mut expr in cmds {
                expr(builder);
            }
        }
    }
}
