use indexmap::IndexSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) struct StringId(usize);

#[derive(Default, Debug, Clone)]
pub(crate) struct StringInterner {
    map: IndexSet<String>,
}

impl StringInterner {
    /// Get an already-interned string.
    pub(crate) fn lookup(&self, s: &str) -> Option<StringId> {
        self.map.get_index_of(s).map(StringId)
    }

    pub(crate) fn get(&self, id: StringId) -> &str {
        &self.map[id.0]
    }

    pub(crate) fn intern(&mut self, s: &str) -> StringId {
        if let Some(id) = self.lookup(s) {
            id
        } else {
            let (idx, is_new) = self.map.insert_full(s.to_owned());
            debug_assert!(is_new);
            StringId(idx)
        }
    }
}
