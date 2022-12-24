use serde::Serialize;

use crate::TemplateOptions;

/// Representation of a single Property in a HAL-FORMS Template.
#[derive(Debug, Serialize)]
pub struct TemplateProperty {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(rename = "readOnly", skip_serializing_if = "is_false")]
    pub readonly: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,

    #[serde(skip_serializing_if = "is_false")]
    pub required: bool,

    #[serde(skip_serializing_if = "is_false")]
    pub templated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cols: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,

    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,

    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<TemplateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl TemplateProperty {
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: ToString,
    {
        Self {
            name:        name.to_string(),
            prompt:      None,
            readonly:    false,
            regex:       None,
            required:    false,
            templated:   false,
            value:       None,
            cols:        None,
            max:         None,
            max_length:  None,
            min:         None,
            min_length:  None,
            options:     None,
            placeholder: None,
            rows:        None,
            step:        None,
            r#type:      None,
        }
    }

    #[must_use]
    pub fn with_prompt<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.prompt = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn readonly(self) -> Self {
        self.with_readonly(true)
    }

    #[must_use]
    pub fn with_readonly(mut self, value: bool) -> Self {
        self.readonly = value;

        self
    }

    #[must_use]
    pub fn with_regex<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.regex = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn required(self) -> Self {
        self.with_required(true)
    }

    #[must_use]
    pub fn with_required(mut self, value: bool) -> Self {
        self.required = value;

        self
    }

    #[must_use]
    pub fn templated(mut self) -> Self {
        self.templated = true;

        self
    }

    #[must_use]
    pub fn with_value<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.value = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_cols<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.cols = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_max<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.max = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_max_length<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.max_length = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_min<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.min = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_min_length<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.min_length = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_options<O>(mut self, option: O) -> Self
    where
        O: Into<TemplateOptions>,
    {
        self.options = Some(option.into());

        self
    }

    #[must_use]
    pub fn with_placeholder<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.placeholder = Some(value.to_string());

        self
    }

    #[must_use]
    pub fn with_rows<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.rows = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_step<S>(mut self, value: S) -> Self
    where
        S: Into<u32>,
    {
        self.step = Some(value.into());

        self
    }

    #[must_use]
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.r#type = Some(value.to_string());

        self
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Needed for Serde.
fn is_false(t: &bool) -> bool {
    t == &false
}
