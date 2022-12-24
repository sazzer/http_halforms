use std::ops::Deref;

use http::{HeaderMap, StatusCode};
use serde::Serialize;

use crate::{Hal, Link, Template};

/// Representation of a HAL or HAL-FORMS response that can be returned to a client.
pub struct HalResponse {
    pub hal:         Hal,
    pub status_code: StatusCode,
    pub headers:     HeaderMap,
}

impl HalResponse {
    /// Add a new link to a HAL document.
    #[must_use]
    pub fn with_link<N, L>(mut self, name: N, link: L) -> Self
    where
        N: ToString,
        L: Into<Link>,
    {
        self.hal = self.hal.with_link(name, link);

        self
    }

    /// Add a new link to a HAL document.
    #[must_use]
    pub fn maybe_with_link<N, L>(mut self, name: N, link: Option<L>) -> Self
    where
        N: ToString,
        L: Into<Link>,
    {
        self.hal = self.hal.maybe_with_link(name, link);

        self
    }

    /// Add a new embedded HAL document to the HAL document.
    #[must_use]
    pub fn with_embedded<N, H>(mut self, name: N, value: H) -> Self
    where
        N: ToString,
        H: Into<Hal>,
    {
        self.hal = self.hal.with_embedded(name, value);

        self
    }

    /// Add a new embedded HAL document to the HAL document.
    #[must_use]
    pub fn maybe_with_embedded<N, H>(mut self, name: N, value: Option<H>) -> Self
    where
        N: ToString,
        H: Into<Hal>,
    {
        self.hal = self.hal.maybe_with_embedded(name, value);

        self
    }

    /// Add a new action template to the HAL-FORMS document.
    #[must_use]
    pub fn with_template<N, T>(mut self, name: N, value: T) -> Self
    where
        N: ToString,
        T: Into<Template>,
    {
        self.hal = self.hal.with_template(name, value);

        self
    }

    /// Add a new action template to the HAL-FORMS document.
    #[must_use]
    pub fn maybe_with_template<N, T>(mut self, name: N, template: Option<T>) -> Self
    where
        N: ToString,
        T: Into<Template>,
    {
        self.hal = self.hal.maybe_with_template(name, template);

        self
    }

    /// Specify the status code of the HTTP response.
    #[must_use]
    pub fn with_status_code<S>(mut self, status_code: S) -> Self
    where
        S: Into<StatusCode>,
    {
        self.status_code = status_code.into();

        self
    }

    /// Specify an HTTP Header to include in the response.
    ///
    /// # Parameters
    /// - `header` - The header to include.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // Making this a reference just makes the caller slightly more awkward for no benefit
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: headers_core::Header,
    {
        let mut values = vec![];
        header.encode(&mut values);

        for value in values {
            self.headers.append(H::name(), value);
        }

        self
    }
}

impl Deref for HalResponse {
    type Target = Hal;

    fn deref(&self) -> &Self::Target {
        &self.hal
    }
}

/// Create a new HAL response for the given payload value.
///
/// # Panics
/// This will panic if the value provided can not be serialized into JSON for some reason.
#[must_use]
pub fn new<V>(value: V) -> HalResponse
where
    V: Serialize,
{
    HalResponse {
        hal:         Hal::new(value),
        status_code: StatusCode::OK,
        headers:     HeaderMap::default(),
    }
}
