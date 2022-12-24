use serde::Serialize;

/// Representation of a single Link in a HAL document.
#[derive(Debug, Serialize)]
pub struct Link {
    pub href:        String,
    #[serde(skip_serializing_if = "is_false")]
    pub templated:   bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type:      Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:        Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile:     Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title:       Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang:    Option<String>,
}

impl Link {
    /// Create a new Link with the given href.
    #[must_use]
    pub fn new<H>(href: H) -> Self
    where
        H: ToString,
    {
        Self {
            href:        href.to_string(),
            templated:   false,
            r#type:      None,
            deprecation: None,
            name:        None,
            profile:     None,
            title:       None,
            hreflang:    None,
        }
    }

    /// Indicate that the link is templated.
    #[must_use]
    pub fn templated(mut self) -> Self {
        self.templated = true;

        self
    }

    /// Specify the type value on the link.
    #[must_use]
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.r#type = Some(value.to_string());

        self
    }

    /// Specify the deprecation value on the link.
    #[must_use]
    pub fn with_deprecation<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.deprecation = Some(value.to_string());

        self
    }

    /// Specify the name value on the link.
    #[must_use]
    pub fn with_name<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.name = Some(value.to_string());

        self
    }

    /// Specify the profile value on the link.
    #[must_use]
    pub fn with_profile<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.profile = Some(value.to_string());

        self
    }

    /// Specify the title value on the link.
    #[must_use]
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }

    /// Specify the hreflang value on the link.
    #[must_use]
    pub fn with_hreflang<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.hreflang = Some(value.to_string());

        self
    }
}

impl<S> From<S> for Link
where
    S: ToString,
{
    fn from(href: S) -> Self {
        Self::new(href.to_string())
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Needed for Serde.
fn is_false(t: &bool) -> bool {
    t == &false
}
