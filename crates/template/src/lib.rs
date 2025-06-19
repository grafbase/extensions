mod content;

use std::borrow::Cow;

use grafbase_sdk::types::Error;
use hashbrown::hash_table::{Entry, HashTable};
use rapidhash::rapidhash;

use crate::content::JsonContent;

#[derive(Default)]
pub struct Templates {
    table: HashTable<Template>,
}

impl Templates {
    pub fn insert<'a>(&mut self, source: impl Into<Cow<'a, str>>) -> Result<(), Error> {
        let source: Cow<'a, str> = source.into();
        let h = rapidhash(source.as_ref().as_ref());
        if let Entry::Vacant(entry) =
            self.table
                .entry(h, |t| t.0.source() == source, |t| rapidhash(t.0.source().as_ref()))
        {
            entry.insert(Template(
                ramhorns::Template::new(source.into_owned()).map_err(|err| err.to_string())?,
            ));
        }
        Ok(())
    }

    pub fn get(&self, source: &str) -> Option<&Template> {
        let h = rapidhash(source.as_ref());
        self.table.find(h, |t| t.0.source() == source)
    }

    pub fn get_or_insert<'a>(&mut self, source: impl Into<Cow<'a, str>>) -> Result<&Template, Error> {
        let source: Cow<'a, str> = source.into();
        let h = rapidhash(source.as_ref().as_ref());
        let entry = match self
            .table
            .entry(h, |t| t.0.source() == source, |t| rapidhash(t.0.source().as_ref()))
        {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert(Template(
                ramhorns::Template::new(source.into_owned()).map_err(|err| err.to_string())?,
            )),
        };
        Ok(entry.into_mut())
    }
}

pub struct Template(ramhorns::Template<'static>);

impl Template {
    pub fn new(source: impl Into<Cow<'static, str>>) -> Result<Self, Error> {
        let source: Cow<'static, str> = source.into();
        let template = ramhorns::Template::new(source).map_err(|err| err.to_string())?;
        Ok(Self(template))
    }

    pub fn render_unescaped(&self, content: &serde_json::Value) -> String {
        self.0.render(&JsonContent {
            value: Cow::Borrowed(content),
            escaping: None,
        })
    }

    pub fn render_json(&self, content: &serde_json::Value) -> String {
        self.0.render(&JsonContent {
            value: Cow::Borrowed(content),
            escaping: Some(TemplateEscaping::Json),
        })
    }

    pub fn render_url(&self, content: &serde_json::Value) -> String {
        self.0.render(&JsonContent {
            value: Cow::Borrowed(content),
            escaping: Some(TemplateEscaping::Url),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TemplateEscaping {
    Json,
    Url,
}
