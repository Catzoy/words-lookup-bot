pub trait StringBuilderExt {
    fn join<T, Action, Separator>(&mut self, arr: &Vec<T>, action: Action, separator: Separator)
    where
        Action: FnMut(&mut Self, &T),
        Separator: FnMut(&mut Self);

    fn list_words(&mut self, arr: &Vec<String>);
}

impl StringBuilderExt for string_builder::Builder {
    fn join<T, Action, Separator>(
        &mut self,
        arr: &Vec<T>,
        mut action: Action,
        mut separator: Separator,
    ) where
        Action: FnMut(&mut Self, &T),
        Separator: FnMut(&mut Self),
    {
        if let Some(first) = arr.first() {
            action(self, first);
            if arr.len() > 1 {
                for item in arr.iter().skip(1) {
                    separator(self);
                    action(self, &item);
                }
            }
        }
    }

    fn list_words(&mut self, arr: &Vec<String>) {
        self.join(
            arr,
            |it, word| it.append(format!("`{}`", word)),
            |it| it.append(", "),
        )
    }
}
