use tui::widgets::ListState;

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: items,
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        if self.items.len() > 0 {
            self.state.select(Some(0));
        } else {
            self.unselect();
        }
    }

    pub fn next(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let i = if i >= self.items.len() - 1 { 0 } else { i + 1 };
                self.state.select(Some(i));
            }
            None => self.unselect(),
        }
    }

    pub fn previous(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let i = if i == 0 { self.items.len() - 1 } else { i - 1 };
                self.state.select(Some(i));
            }
            None => self.unselect(),
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn selected(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selected() {
        let mut list = StatefulList::with_items(
            vec!["hoge", "foo", "bar"]
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        );
        assert_eq!(list.selected(), None);

        list.state.select(Some(1));

        assert_eq!(list.selected(), Some(&"foo".to_owned()));
    }
}
