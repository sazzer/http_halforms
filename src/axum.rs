use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::{Hal, HalResponse};

impl IntoResponse for HalResponse {
    fn into_response(self) -> Response {
        let status_code = self.status_code;

        let content_type = if has_templates(&self.hal) {
            "application/prs.hal-forms+json"
        } else if !self.hal.links.is_empty() | !self.hal.embedded.is_empty() {
            "application/hal+json"
        } else {
            "application/json"
        };

        let body = Json(self.hal);
        let mut response = (status_code, body).into_response();

        let headers = response.headers_mut();

        for (header_name, header_value) in self.headers {
            if let Some(header_name) = header_name {
                headers.append(header_name, header_value);
            }
        }

        response
            .headers_mut()
            .insert("content-type", content_type.parse().unwrap());
        response
    }
}

/// Helper to see if the HAL document or any documented embedded into it have any templated values.
fn has_templates(hal: &Hal) -> bool {
    if hal.templates.is_empty() {
        hal.embedded
            .iter()
            .flat_map(|e| e.1.iter())
            .any(has_templates)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use assert2::check;
    use axum::response::IntoResponse;
    use headers::{CacheControl, ContentType, ETag};
    use http::StatusCode;
    use insta::assert_json_snapshot;
    use serde_json::{json, Value};

    use crate::Hal;

    #[tokio::test]
    async fn no_values() {
        let sut = crate::new(());

        let response = sut.into_response();
        check!(response.status() == StatusCode::OK);
        check!(response.headers().get("Content-Type").unwrap() == "application/json");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"{}"###);
    }

    #[tokio::test]
    async fn hal_example() {
        let sut = crate::new(json!({
          "currentlyProcessing": 14,
          "shippedToday": 20
        }))
        .with_link("self", "/orders")
        .with_link("next", "/orders?page=2")
        .with_link("find", crate::Link::new("/orders{?id}").templated())
        .with_embedded(
            "orders",
            crate::Hal::new(json!({
              "total": 30.00,
              "currency": "USD",
              "status": "shipped"
            }))
            .with_link("self", "/orders/123")
            .with_link("basket", "/baskets/98712")
            .with_link("customer", "/customers/7809"),
        )
        .with_embedded(
            "orders",
            crate::Hal::new(json!({
              "total": 20.00,
              "currency": "USD",
              "status": "processing"
            }))
            .with_link("self", "/orders/124")
            .with_link("basket", "/baskets/97213")
            .with_link("customer", "/customers/12369"),
        );

        let response = sut.into_response();
        check!(response.status() == StatusCode::OK);
        check!(response.headers().get("Content-Type").unwrap() == "application/hal+json");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "_links": {
            "find": {
              "href": "/orders{?id}",
              "templated": true
            },
            "next": {
              "href": "/orders?page=2"
            },
            "self": {
              "href": "/orders"
            }
          },
          "_embedded": {
            "orders": [
              {
                "_links": {
                  "basket": {
                    "href": "/baskets/98712"
                  },
                  "customer": {
                    "href": "/customers/7809"
                  },
                  "self": {
                    "href": "/orders/123"
                  }
                },
                "total": 30.0,
                "currency": "USD",
                "status": "shipped"
              },
              {
                "_links": {
                  "basket": {
                    "href": "/baskets/97213"
                  },
                  "customer": {
                    "href": "/customers/12369"
                  },
                  "self": {
                    "href": "/orders/124"
                  }
                },
                "total": 20.0,
                "currency": "USD",
                "status": "processing"
              }
            ]
          },
          "currentlyProcessing": 14,
          "shippedToday": 20
        }
        "###);
    }

    #[tokio::test]
    async fn with_template() {
        let sut = crate::new(()).with_template("default", crate::Template::default());

        let response = sut.into_response();
        check!(response.status() == StatusCode::OK);
        check!(response.headers().get("Content-Type").unwrap() == "application/prs.hal-forms+json");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "_templates": {
            "default": {}
          }
        }
        "###);
    }

    #[tokio::test]
    async fn with_nested_template() {
        let sut = crate::new(()).with_embedded(
            "other",
            Hal::new(()).with_template("default", crate::Template::default()),
        );

        let response = sut.into_response();
        check!(response.status() == StatusCode::OK);
        check!(response.headers().get("Content-Type").unwrap() == "application/prs.hal-forms+json");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"
        {
          "_embedded": {
            "other": {
              "_templates": {
                "default": {}
              }
            }
          }
        }
        "###);
    }

    #[tokio::test]
    async fn status_code() {
        let sut = crate::new(()).with_status_code(StatusCode::ACCEPTED);

        let response = sut.into_response();
        check!(response.status() == StatusCode::ACCEPTED);
        check!(response.headers().get("Content-Type").unwrap() == "application/json");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"{}"###);
    }

    #[tokio::test]
    async fn headers() {
        let sut = crate::new(())
            .with_header(
                CacheControl::new()
                    .with_public()
                    .with_max_age(std::time::Duration::from_secs(3600)),
            )
            .with_header(ETag::from_str("\"Hello\"").unwrap())
            .with_header(ContentType::xml());

        let response = sut.into_response();
        check!(response.status() == StatusCode::OK);
        check!(response.headers().get("Content-Type").unwrap() == "application/json");
        check!(response.headers().get("Cache-Control").unwrap() == "public, max-age=3600");
        check!(response.headers().get("ETag").unwrap() == "\"Hello\"");

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        check!(body.len() != 0);
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_json_snapshot!(body, @r###"{}"###);
    }
}
