use serde::Serialize;

use crate::TemplateProperty;

/// Representation of a single Template in a HAL-FORMS document.
#[derive(Debug, Serialize, Default)]
pub struct Template {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<TemplateProperty>,
}

impl Template {
    #[must_use]
    pub fn with_content_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.content_type = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_method<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.method = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_target<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.target = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_property<P>(mut self, value: P) -> Self
    where
        P: Into<TemplateProperty>,
    {
        self.properties.push(value.into());

        self
    }
}
