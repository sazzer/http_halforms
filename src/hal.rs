#[allow(clippy::module_inception)]
mod hal;
mod link;
mod option;
mod property;
mod template;

pub use hal::*;
pub use link::*;
pub use option::*;
pub use property::*;
pub use template::*;

#[cfg(test)]
mod tests {
    use assert2::let_assert;
    use insta::assert_json_snapshot;
    use serde_json::json;

    use super::*;

    #[test]
    fn serialize_empty() {
        let sut = Hal::new(());

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"{}"###);
    }

    #[test]
    fn serialize_payload() {
        let sut = Hal::new(json!({"name": "hal", "answer": 42}));

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "name": "hal",
          "answer": 42
        }
        "###);
    }

    #[test]
    fn serialize_single_basic_link() {
        let sut = Hal::new(()).with_link("self", "/");

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_links": {
            "self": {
              "href": "/"
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_multiple_basic_links() {
        let sut = Hal::new(())
            .with_link("other", "/abc")
            .with_link("other", "/def");

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_links": {
            "other": [
              {
                "href": "/abc"
              },
              {
                "href": "/def"
              }
            ]
          }
        }
        "###);
    }

    #[test]
    fn serialize_single_full_link() {
        let sut = Hal::new(()).with_link(
            "self",
            Link::new("/")
                .templated()
                .with_deprecation("Deprecated")
                .with_hreflang("en")
                .with_name("Hal")
                .with_profile("Profile")
                .with_title("Hal Link")
                .with_type("application/hal+json"),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_links": {
            "self": {
              "href": "/",
              "templated": true,
              "type": "application/hal+json",
              "deprecation": "Deprecated",
              "name": "Hal",
              "profile": "Profile",
              "title": "Hal Link",
              "hreflang": "en"
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_single_embedded() {
        let sut = Hal::new(()).with_embedded("test", Hal::new(json!({"name": "test"})));

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_embedded": {
            "test": {
              "name": "test"
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_multiple_embedded() {
        let sut = Hal::new(())
            .with_embedded("test", Hal::new(json!({"name": "test1"})))
            .with_embedded("test", Hal::new(json!({"name": "test2"})));

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_embedded": {
            "test": [
              {
                "name": "test1"
              },
              {
                "name": "test2"
              }
            ]
          }
        }
        "###);
    }

    #[test]
    fn serialize_hal_example() {
        let sut = Hal::new(json!({
          "currentlyProcessing": 14,
          "shippedToday": 20
        }))
        .with_link("self", "/orders")
        .with_link("next", "/orders?page=2")
        .with_link("find", Link::new("/orders{?id}").templated())
        .with_embedded(
            "orders",
            Hal::new(json!({
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
            Hal::new(json!({
              "total": 20.00,
              "currency": "USD",
              "status": "processing"
            }))
            .with_link("self", "/orders/124")
            .with_link("basket", "/baskets/97213")
            .with_link("customer", "/customers/12369"),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
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

    #[test]
    fn serialize_empty_template() {
        let sut = Hal::new(()).with_template("default", Template::default());

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {}
          }
        }
        "###);
    }

    #[test]
    fn serialize_simple_template() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default()
                .with_method("POST")
                .with_target("/abc")
                .with_content_type("application/json")
                .with_title("Do Something"),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "contentType": "application/json",
              "method": "POST",
              "target": "/abc",
              "title": "Do Something"
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_template_simple_property() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default()
                .with_method("POST")
                .with_property(TemplateProperty::new("name")),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "method": "POST",
              "properties": [
                {
                  "name": "name"
                }
              ]
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_template_detailed_property() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default().with_method("POST").with_property(
                TemplateProperty::new("name")
                    .with_type("text")
                    .with_max_length(500u32)
                    .with_min_length(10u32)
                    .with_prompt("Name")
                    .required(),
            ),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "method": "POST",
              "properties": [
                {
                  "name": "name",
                  "prompt": "Name",
                  "required": true,
                  "maxLength": 500,
                  "minLength": 10,
                  "type": "text"
                }
              ]
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_template_simple_inline_options() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default().with_method("POST").with_property(
                TemplateProperty::new("name")
                    .with_type("text")
                    .with_options(
                        TemplateOptions::inline(vec!["a", "b", "c"])
                            .with_max_items(5u32)
                            .with_min_items(3u32)
                            .with_selected_value("b"),
                    ),
            ),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "method": "POST",
              "properties": [
                {
                  "name": "name",
                  "options": {
                    "inline": [
                      {
                        "value": "a"
                      },
                      {
                        "value": "b"
                      },
                      {
                        "value": "c"
                      }
                    ],
                    "maxItems": 5,
                    "minItems": 3,
                    "selectedValues": [
                      "b"
                    ]
                  },
                  "type": "text"
                }
              ]
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_template_inline_options() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default().with_method("POST").with_property(
                TemplateProperty::new("name")
                    .with_type("text")
                    .with_options(
                        TemplateOptions::inline(vec![
                            InlineOption::new("a").with_prompt("A"),
                            InlineOption::new("b").with_prompt("B"),
                            InlineOption::new("c").with_prompt("C"),
                        ])
                        .with_max_items(5u32)
                        .with_min_items(3u32)
                        .with_selected_value("b"),
                    ),
            ),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "method": "POST",
              "properties": [
                {
                  "name": "name",
                  "options": {
                    "inline": [
                      {
                        "prompt": "A",
                        "value": "a"
                      },
                      {
                        "prompt": "B",
                        "value": "b"
                      },
                      {
                        "prompt": "C",
                        "value": "c"
                      }
                    ],
                    "maxItems": 5,
                    "minItems": 3,
                    "selectedValues": [
                      "b"
                    ]
                  },
                  "type": "text"
                }
              ]
            }
          }
        }
        "###);
    }

    #[test]
    fn serialize_template_line_options() {
        let sut = Hal::new(()).with_template(
            "default",
            Template::default().with_method("POST").with_property(
                TemplateProperty::new("name")
                    .with_type("text")
                    .with_options(
                        TemplateOptions::link("/options")
                            .with_max_items(5u32)
                            .with_min_items(3u32)
                            .with_selected_value("b"),
                    ),
            ),
        );

        let result = serde_json::to_value(sut);
        let_assert!(Ok(value) = result);

        assert_json_snapshot!(value, @r###"
        {
          "_templates": {
            "default": {
              "method": "POST",
              "properties": [
                {
                  "name": "name",
                  "options": {
                    "link": {
                      "href": "/options"
                    },
                    "maxItems": 5,
                    "minItems": 3,
                    "selectedValues": [
                      "b"
                    ]
                  },
                  "type": "text"
                }
              ]
            }
          }
        }
        "###);
    }
}
