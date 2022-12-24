use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::Value;

use crate::{utils::single_multiple::SingleOrMultiple, Link, Template};

/// Representation of a HAL document.
#[derive(Debug, Serialize)]
pub struct Hal {
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, SingleOrMultiple<Link>>,

    #[serde(rename = "_embedded")]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub embedded: BTreeMap<String, SingleOrMultiple<Hal>>,

    #[serde(rename = "_templates")]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub templates: BTreeMap<String, Template>,

    #[serde(flatten)]
    pub payload: Value,
}

impl Hal {
    /// Create a new HAL document for the given payload value.
    ///
    /// # Panics
    /// This will panic if the value provided can not be serialized into JSON for some reason.
    #[must_use]
    pub fn new<V>(value: V) -> Self
    where
        V: Serialize,
    {
        let payload = serde_json::to_value(value).unwrap();

        Self {
            payload,
            links: BTreeMap::new(),
            embedded: BTreeMap::new(),
            templates: BTreeMap::new(),
        }
    }

    /// Add a new link to a HAL document.
    #[must_use]
    pub fn with_link<N, L>(mut self, name: N, link: L) -> Self
    where
        N: ToString,
        L: Into<Link>,
    {
        let name = name.to_string();
        let link = link.into();

        let links = match self.links.remove(&name) {
            None => SingleOrMultiple::Single(link),
            Some(links) => links.insert(link),
        };

        self.links.insert(name, links);

        self
    }

    /// Add a new link to a HAL document.
    #[must_use]
    pub fn maybe_with_link<N, L>(self, name: N, link: Option<L>) -> Self
    where
        N: ToString,
        L: Into<Link>,
    {
        if let Some(link) = link {
            self.with_link(name, link)
        } else {
            self
        }
    }

    /// Add a new embedded HAL document to the HAL document.
    #[must_use]
    pub fn with_embedded<N, H>(mut self, name: N, value: H) -> Self
    where
        N: ToString,
        H: Into<Hal>,
    {
        let name = name.to_string();
        let value = value.into();

        let embedded = match self.embedded.remove(&name) {
            None => SingleOrMultiple::Single(value),
            Some(links) => links.insert(value),
        };

        self.embedded.insert(name, embedded);

        self
    }

    /// Add a new embedded HAL document to a HAL document.
    #[must_use]
    pub fn maybe_with_embedded<N, L>(self, name: N, embedded: Option<L>) -> Self
    where
        N: ToString,
        L: Into<Hal>,
    {
        if let Some(embedded) = embedded {
            self.with_embedded(name, embedded)
        } else {
            self
        }
    }

    /// Add a new action template to the HAL-FORMS document.
    #[must_use]
    pub fn with_template<N, T>(mut self, name: N, value: T) -> Self
    where
        N: ToString,
        T: Into<Template>,
    {
        self.templates.insert(name.to_string(), value.into());

        self
    }

    /// Add a new action template to the HAL-FORMS document.
    #[must_use]
    pub fn maybe_with_template<N, T>(self, name: N, template: Option<T>) -> Self
    where
        N: ToString,
        T: Into<Template>,
    {
        if let Some(template) = template {
            self.with_template(name, template)
        } else {
            self
        }
    }
}
