use serde::Serialize;

use crate::Link;

/// Representation of the options for a single template property.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum TemplateOptions {
    Inline {
        inline: Vec<InlineOption>,

        #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
        max_items: Option<u32>,

        #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
        min_items: Option<u32>,

        #[serde(rename = "selectedValues", skip_serializing_if = "Vec::is_empty")]
        selected_values: Vec<String>,
    },
    Link {
        link: Link,

        #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
        max_items: Option<u32>,

        #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
        min_items: Option<u32>,

        #[serde(rename = "selectedValues", skip_serializing_if = "Vec::is_empty")]
        selected_values: Vec<String>,
    },
}

/// Representation of a single option for a single template property.
#[derive(Debug, Serialize)]
pub struct InlineOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    pub value: String,
}

impl TemplateOptions {
    #[must_use]
    pub fn inline<O>(options: Vec<O>) -> Self
    where
        O: Into<InlineOption>,
    {
        Self::Inline {
            inline:          options.into_iter().map(Into::into).collect(),
            max_items:       None,
            min_items:       None,
            selected_values: vec![],
        }
    }

    #[must_use]
    pub fn link<L>(link: L) -> Self
    where
        L: Into<Link>,
    {
        Self::Link {
            link:            link.into(),
            max_items:       None,
            min_items:       None,
            selected_values: vec![],
        }
    }

    #[must_use]
    pub fn with_max_items<V>(self, value: V) -> Self
    where
        V: Into<u32>,
    {
        match self {
            Self::Inline {
                inline,
                max_items: _,
                min_items,
                selected_values,
            } => Self::Inline {
                inline,
                max_items: Some(value.into()),
                min_items,
                selected_values,
            },
            Self::Link {
                link,
                max_items: _,
                min_items,
                selected_values,
            } => Self::Link {
                link,
                max_items: Some(value.into()),
                min_items,
                selected_values,
            },
        }
    }

    #[must_use]
    pub fn with_min_items<V>(self, value: V) -> Self
    where
        V: Into<u32>,
    {
        match self {
            Self::Inline {
                inline,
                max_items,
                min_items: _,
                selected_values,
            } => Self::Inline {
                inline,
                max_items,
                min_items: Some(value.into()),
                selected_values,
            },
            Self::Link {
                link,
                max_items,
                min_items: _,
                selected_values,
            } => Self::Link {
                link,
                max_items,
                min_items: Some(value.into()),
                selected_values,
            },
        }
    }

    #[must_use]
    pub fn with_selected_value<V>(self, value: V) -> Self
    where
        V: ToString,
    {
        match self {
            Self::Inline {
                inline,
                max_items,
                min_items,
                mut selected_values,
            } => {
                selected_values.push(value.to_string());
                Self::Inline {
                    inline,
                    max_items,
                    min_items,
                    selected_values,
                }
            },
            Self::Link {
                link,
                max_items,
                min_items,
                mut selected_values,
            } => {
                selected_values.push(value.to_string());
                Self::Link {
                    link,
                    max_items,
                    min_items,
                    selected_values,
                }
            },
        }
    }
}

impl InlineOption {
    #[must_use]
    pub fn new<V>(value: V) -> Self
    where
        V: ToString,
    {
        Self {
            prompt: None,
            value:  value.to_string(),
        }
    }

    #[must_use]
    pub fn with_prompt<V>(mut self, value: V) -> Self
    where
        V: ToString,
    {
        self.prompt = Some(value.to_string());

        self
    }
}

impl<S> From<S> for InlineOption
where
    S: ToString,
{
    fn from(value: S) -> Self {
        Self::new(value)
    }
}
