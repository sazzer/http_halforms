use std::collections::BTreeMap;

use serde::Serialize;

/// Representation of Link Hints from draft-ietf-httpapi-link-hint-01
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LinkHints {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow:            Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub formats:          BTreeMap<String, LinkHintFormat>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub accept_post:      BTreeMap<String, LinkHintFormat>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accept_patch:     Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accept_ranges:    Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accept_prefer:    Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub precondition_req: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub auth_schemes:     Vec<LinkHintAuthSchemes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status:           Option<String>,
}

/// Representation of object for the "format" field of the Link Hints.
#[derive(Debug, Serialize, Default)]
pub struct LinkHintFormat {
    #[serde(skip_serializing_if = "is_false")]
    pub deprecated: bool,
}

/// Representation of object for the "auth-schemes" field of the Link Hints.
#[derive(Debug, Serialize)]
pub struct LinkHintAuthSchemes {
    pub scheme: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub realms: Vec<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // Needed for Serde.
fn is_false(t: &bool) -> bool {
    t == &false
}

impl LinkHints {
    /// Specify an allow value.
    #[must_use]
    pub fn with_allow<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.allow.push(value.to_string());

        self
    }

    /// Specify a format value.
    #[must_use]
    pub fn with_format<S, V>(mut self, format: S, value: V) -> Self
    where
        S: ToString,
        V: Into<LinkHintFormat>,
    {
        self.formats.insert(format.to_string(), value.into());

        self
    }

    /// Specify an accept-post value.
    #[must_use]
    pub fn with_accept_post<S, V>(mut self, format: S, value: V) -> Self
    where
        S: ToString,
        V: Into<LinkHintFormat>,
    {
        self.accept_post.insert(format.to_string(), value.into());

        self
    }

    /// Specify an accept-patch value.
    #[must_use]
    pub fn with_accept_patch<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.accept_patch.push(value.to_string());

        self
    }

    /// Specify an accept-range value.
    #[must_use]
    pub fn with_accept_range<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.accept_ranges.push(value.to_string());

        self
    }

    /// Specify an accept-prefer value.
    #[must_use]
    pub fn with_accept_prefer<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.accept_prefer.push(value.to_string());

        self
    }

    /// Specify a precondition-req value.
    #[must_use]
    pub fn with_precondition_req<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.precondition_req.push(value.to_string());

        self
    }

    /// Specify a precondition-req value of "etag".
    #[must_use]
    pub fn with_precondition_req_etag(self) -> Self {
        self.with_precondition_req("etag")
    }

    /// Specify a precondition-req value of "last-modified".
    #[must_use]
    pub fn with_precondition_req_last_modified(self) -> Self {
        self.with_precondition_req("last-modified")
    }

    /// Specify an auth-scheme value.
    #[must_use]
    pub fn with_auth_scheme<V>(mut self, value: V) -> Self
    where
        V: Into<LinkHintAuthSchemes>,
    {
        self.auth_schemes.push(value.into());

        self
    }

    /// Specify a status value.
    #[must_use]
    pub fn with_status<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.status = Some(value.to_string());

        self
    }

    /// Specify a status value of "deprecated".
    #[must_use]
    pub fn with_status_deprecated(self) -> Self {
        self.with_status("deprecated")
    }

    /// Specify a status value of "gone".
    #[must_use]
    pub fn with_status_gone(self) -> Self {
        self.with_status("gone")
    }
}

impl LinkHintFormat {
    /// Specify a deprecated value.
    #[must_use]
    pub fn with_deprecated(mut self) -> Self {
        self.deprecated = true;

        self
    }
}

impl LinkHintAuthSchemes {
    pub fn new<S>(scheme: S) -> Self
    where
        S: ToString,
    {
        Self {
            scheme: scheme.to_string(),
            realms: vec![],
        }
    }

    /// Specify a realm value.
    #[must_use]
    pub fn with_realm<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.realms.push(value.to_string());

        self
    }
}
