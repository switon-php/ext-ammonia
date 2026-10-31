use std::collections::HashSet;
use std::panic::AssertUnwindSafe;
use std::sync::Mutex;

use ammonia_sanitizer::{Builder, Url, UrlRelative};

/// Tag name to attribute name to the values allowed for that attribute.
type AttributeValueMap = Vec<(String, Vec<(String, Vec<String>)>)>;

/// One setter on `Switon\Ammonia\Builder`, replayed onto ammonia in call order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Tags(Vec<String>),
    AddTags(Vec<String>),
    RmTags(Vec<String>),
    CleanContentTags(Vec<String>),
    AddCleanContentTags(Vec<String>),
    RmCleanContentTags(Vec<String>),
    GenericAttributes(Vec<String>),
    AddGenericAttributes(Vec<String>),
    RmGenericAttributes(Vec<String>),
    GenericAttributePrefixes(Vec<String>),
    AddGenericAttributePrefixes(Vec<String>),
    RmGenericAttributePrefixes(Vec<String>),
    TagAttributes(Vec<(String, Vec<String>)>),
    AddTagAttributes {
        tag: String,
        attrs: Vec<String>,
    },
    RmTagAttributes {
        tag: String,
        attrs: Vec<String>,
    },
    AddTagAttributeValues {
        tag: String,
        attribute: String,
        values: Vec<String>,
    },
    RmTagAttributeValues {
        tag: String,
        attribute: String,
        values: Vec<String>,
    },
    TagAttributeValues(AttributeValueMap),
    SetTagAttributeValue {
        tag: String,
        attribute: String,
        value: String,
    },
    RmSetTagAttributeValue {
        tag: String,
        attribute: String,
    },
    SetTagAttributeValues(Vec<(String, Vec<(String, String)>)>),
    UrlSchemes(Vec<String>),
    AddUrlSchemes(Vec<String>),
    RmUrlSchemes(Vec<String>),
    RelativePass,
    RelativeDeny,
    RelativeRewriteBase(String),
    RelativeRewriteRoot {
        root: String,
        path: String,
    },
    AddAllowedClasses {
        tag: String,
        classes: Vec<String>,
    },
    RmAllowedClasses {
        tag: String,
        classes: Vec<String>,
    },
    AllowedClasses(Vec<(String, Vec<String>)>),
    StripComments(bool),
    LinkRel(Option<String>),
    IdPrefix(Option<String>),
    FilterStyleProperties(Vec<String>),
    TextOnly,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SanitizeError {
    BadUrl(String),
    Conflict(String),
}

pub fn sanitize(html: &str, ops: &[Op]) -> Result<String, SanitizeError> {
    let mut builder = Builder::default();
    apply(&mut builder, ops)?;
    // ammonia reports contradictory builder settings with assert. Catch that and
    // hide the default panic hook so PHP only sees the exception. The hook is
    // process-global, so the lock keeps two requests from swapping it.
    static PANIC_HOOK: Mutex<()> = Mutex::new(());
    let _guard = PANIC_HOOK.lock().unwrap_or_else(|err| err.into_inner());
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(AssertUnwindSafe(|| builder.clean(html).to_string()));
    std::panic::set_hook(previous);
    match result {
        Ok(cleaned) => Ok(cleaned),
        Err(payload) => Err(SanitizeError::Conflict(panic_message(payload))),
    }
}

fn apply<'a>(builder: &mut Builder<'a>, ops: &'a [Op]) -> Result<(), SanitizeError> {
    for op in ops {
        match op {
            Op::Tags(values) => {
                builder.tags(as_set(values));
            }
            Op::AddTags(values) => {
                builder.add_tags(as_refs(values));
            }
            Op::RmTags(values) => {
                builder.rm_tags(as_refs(values));
            }
            Op::CleanContentTags(values) => {
                builder.clean_content_tags(as_set(values));
            }
            Op::AddCleanContentTags(values) => {
                builder.add_clean_content_tags(as_refs(values));
            }
            Op::RmCleanContentTags(values) => {
                builder.rm_clean_content_tags(as_refs(values));
            }
            Op::GenericAttributes(values) => {
                builder.generic_attributes(as_set(values));
            }
            Op::AddGenericAttributes(values) => {
                builder.add_generic_attributes(as_refs(values));
            }
            Op::RmGenericAttributes(values) => {
                builder.rm_generic_attributes(as_refs(values));
            }
            Op::GenericAttributePrefixes(values) => {
                builder.generic_attribute_prefixes(as_set(values));
            }
            Op::AddGenericAttributePrefixes(values) => {
                builder.add_generic_attribute_prefixes(as_refs(values));
            }
            Op::RmGenericAttributePrefixes(values) => {
                builder.rm_generic_attribute_prefixes(as_refs(values));
            }
            Op::TagAttributes(entries) => {
                builder.tag_attributes(
                    entries
                        .iter()
                        .map(|(tag, attrs)| (tag.as_str(), as_set(attrs)))
                        .collect(),
                );
            }
            Op::AddTagAttributes { tag, attrs } => {
                builder.add_tag_attributes(tag.as_str(), as_refs(attrs));
            }
            Op::RmTagAttributes { tag, attrs } => {
                builder.rm_tag_attributes(tag.as_str(), as_refs(attrs));
            }
            Op::AddTagAttributeValues {
                tag,
                attribute,
                values,
            } => {
                builder.add_tag_attribute_values(tag.as_str(), attribute.as_str(), as_refs(values));
            }
            Op::RmTagAttributeValues {
                tag,
                attribute,
                values,
            } => {
                builder.rm_tag_attribute_values(tag.as_str(), attribute.as_str(), as_refs(values));
            }
            Op::TagAttributeValues(entries) => {
                builder.tag_attribute_values(
                    entries
                        .iter()
                        .map(|(tag, attrs)| {
                            (
                                tag.as_str(),
                                attrs
                                    .iter()
                                    .map(|(attr, vals)| (attr.as_str(), as_set(vals)))
                                    .collect(),
                            )
                        })
                        .collect(),
                );
            }
            Op::SetTagAttributeValue {
                tag,
                attribute,
                value,
            } => {
                builder.set_tag_attribute_value(tag.as_str(), attribute.as_str(), value.as_str());
            }
            Op::RmSetTagAttributeValue { tag, attribute } => {
                builder.rm_set_tag_attribute_value(tag.as_str(), attribute.as_str());
            }
            Op::SetTagAttributeValues(entries) => {
                builder.set_tag_attribute_values(
                    entries
                        .iter()
                        .map(|(tag, attrs)| {
                            (
                                tag.as_str(),
                                attrs
                                    .iter()
                                    .map(|(attr, val)| (attr.as_str(), val.as_str()))
                                    .collect(),
                            )
                        })
                        .collect(),
                );
            }
            Op::UrlSchemes(values) => {
                builder.url_schemes(as_set(values));
            }
            Op::AddUrlSchemes(values) => {
                builder.add_url_schemes(as_refs(values));
            }
            Op::RmUrlSchemes(values) => {
                builder.rm_url_schemes(as_refs(values));
            }
            Op::RelativePass => {
                builder.url_relative(UrlRelative::PassThrough);
            }
            Op::RelativeDeny => {
                builder.url_relative(UrlRelative::Deny);
            }
            Op::RelativeRewriteBase(base) => {
                let url = parse_url(base)?;
                builder.url_relative(UrlRelative::RewriteWithBase(url));
            }
            Op::RelativeRewriteRoot { root, path } => {
                let root = parse_url(root)?;
                builder.url_relative(UrlRelative::RewriteWithRoot {
                    root,
                    path: path.clone(),
                });
            }
            Op::AddAllowedClasses { tag, classes } => {
                builder.add_allowed_classes(tag.as_str(), as_refs(classes));
            }
            Op::RmAllowedClasses { tag, classes } => {
                builder.rm_allowed_classes(tag.as_str(), as_refs(classes));
            }
            Op::AllowedClasses(entries) => {
                builder.allowed_classes(
                    entries
                        .iter()
                        .map(|(tag, classes)| (tag.as_str(), as_set(classes)))
                        .collect(),
                );
            }
            Op::StripComments(strip) => {
                builder.strip_comments(*strip);
            }
            Op::LinkRel(value) => {
                builder.link_rel(value.as_deref());
            }
            Op::IdPrefix(value) => {
                builder.id_prefix(value.as_deref());
            }
            Op::FilterStyleProperties(values) => {
                builder.filter_style_properties(as_set(values));
            }
            Op::TextOnly => {
                builder.tags(HashSet::new());
            }
        }
    }
    Ok(())
}

fn as_refs(values: &[String]) -> impl Iterator<Item = &str> {
    values.iter().map(String::as_str)
}

fn as_set(values: &[String]) -> HashSet<&str> {
    values.iter().map(String::as_str).collect()
}

fn parse_url(value: &str) -> Result<Url, SanitizeError> {
    Url::parse(value).map_err(|err| SanitizeError::BadUrl(err.to_string()))
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|msg| (*msg).to_string()))
        .unwrap_or_else(|| "ammonia rejected this configuration".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clean(html: &str, ops: &[Op]) -> String {
        sanitize(html, ops).expect("sanitize")
    }

    #[test]
    fn default_strips_script_and_event_handler() {
        let html = "<b><img src='' onerror=alert(1)>ok</b><script>alert(1)</script>";
        let cleaned = clean(html, &[]);
        assert_eq!(cleaned, Builder::default().clean(html).to_string());
        assert!(!cleaned.contains("onerror"));
        assert!(!cleaned.contains("script"));
    }

    #[test]
    fn tags_call_replaces_an_earlier_add() {
        let html = "<div>a</div><p>b</p>";
        let replaced = clean(
            html,
            &[Op::AddTags(vec!["div".into()]), Op::Tags(vec!["p".into()])],
        );
        let extended = clean(
            html,
            &[Op::Tags(vec!["p".into()]), Op::AddTags(vec!["div".into()])],
        );
        assert!(!replaced.contains("<div"));
        assert!(extended.contains("<div>a</div>"));
    }

    #[test]
    fn url_scheme_and_relative_policy_follow_call_order() {
        let html = "<a href=\"http://a\">h</a><a href=\"/r\">r</a>";
        let https_only = clean(html, &[Op::UrlSchemes(vec!["https".into()])]);
        assert!(!https_only.contains("http://a"));
        let denied = clean(html, &[Op::RelativeDeny]);
        assert!(!denied.contains("href=\"/r\""));
        let passed = clean(html, &[Op::RelativeDeny, Op::RelativePass]);
        assert!(passed.contains("href=\"/r\""));
    }

    #[test]
    fn tag_attributes_replaces_the_default_map() {
        let html = "<a href=\"/r\">r</a>";
        let cleared = clean(html, &[Op::TagAttributes(Vec::new())]);
        assert!(!cleared.contains("href="));
        let restored = clean(
            html,
            &[
                Op::TagAttributes(Vec::new()),
                Op::AddTagAttributes {
                    tag: "a".into(),
                    attrs: vec!["href".into()],
                },
            ],
        );
        assert!(restored.contains("href=\"/r\""));
    }

    #[test]
    fn bad_base_url_is_an_error() {
        let err = sanitize(
            "<a href=\"/a\">x</a>",
            &[Op::RelativeRewriteBase("not a url".into())],
        );
        assert!(matches!(err, Err(SanitizeError::BadUrl(_))));
    }

    #[test]
    fn clean_content_tags_replace_the_default_list() {
        let cleaned = clean(
            "<script>alert(1)</script><style>x</style><p>ok</p>",
            &[Op::CleanContentTags(vec![])],
        );
        assert_eq!(cleaned, "alert(1)x<p>ok</p>");
    }

    #[test]
    fn bad_root_url_is_an_error() {
        let err = sanitize(
            "<a href=\"/a\">x</a>",
            &[Op::RelativeRewriteRoot {
                root: "not a url".into(),
                path: "index.html".into(),
            }],
        );
        assert!(matches!(err, Err(SanitizeError::BadUrl(_))));
    }

    #[test]
    fn rel_attribute_conflicts_with_link_rel() {
        let generic = sanitize(
            "<a href=\"/r\">r</a>",
            &[Op::AddGenericAttributes(vec!["rel".into()])],
        );
        assert!(matches!(generic, Err(SanitizeError::Conflict(_))));
        let on_anchor = sanitize(
            "<a href=\"/r\" rel=\"me\">r</a>",
            &[Op::AddTagAttributes {
                tag: "a".into(),
                attrs: vec!["rel".into()],
            }],
        );
        assert!(matches!(on_anchor, Err(SanitizeError::Conflict(_))));
    }

    #[test]
    fn style_element_conflicts_with_style_filter() {
        let err = sanitize(
            "<style>a{}</style>",
            &[
                Op::RmCleanContentTags(vec!["style".into()]),
                Op::AddTags(vec!["style".into()]),
                Op::FilterStyleProperties(vec!["color".into()]),
            ],
        );
        assert!(
            matches!(err, Err(SanitizeError::Conflict(message)) if message.contains("stylesheet"))
        );
    }

    #[test]
    fn contradictory_class_rules_are_an_error() {
        let ops = [
            Op::GenericAttributes(vec!["class".into()]),
            Op::AddAllowedClasses {
                tag: "span".into(),
                classes: vec!["hidden".into()],
            },
        ];
        let err = sanitize("<span class=\"hidden\">x</span>", &ops);
        assert!(matches!(err, Err(SanitizeError::Conflict(_))));
    }

    #[test]
    fn tag_attribute_values_replaces_the_whole_map() {
        let html = "<a href=\"/ok\">y</a><a href=\"/no\">n</a>";
        let set = clean(
            html,
            &[
                Op::RmTagAttributes {
                    tag: "a".into(),
                    attrs: vec!["href".into()],
                },
                Op::TagAttributeValues(vec![(
                    "a".into(),
                    vec![("href".into(), vec!["/ok".into()])],
                )]),
            ],
        );
        assert!(set.contains("href=\"/ok\""));
        assert!(!set.contains("href=\"/no\""));
    }

    #[test]
    fn set_tag_attribute_values_replaces_the_whole_map() {
        let html = "<a href=\"/r\">r</a>";
        let forced = clean(
            html,
            &[Op::SetTagAttributeValues(vec![(
                "a".into(),
                vec![("target".into(), "_blank".into())],
            )])],
        );
        assert!(forced.contains("target=\"_blank\""));
    }

    #[test]
    fn allowed_classes_replaces_the_whole_map() {
        let html = "<span class=\"ok bad\">t</span>";
        let cleaned = clean(
            html,
            &[Op::AllowedClasses(vec![("span".into(), vec!["ok".into()])])],
        );
        assert_eq!(cleaned, "<span class=\"ok\">t</span>");
    }

    #[test]
    fn text_only_strips_all_tags() {
        let html = "<b>bold</b><script>alert(1)</script><p>text</p>";
        let cleaned = clean(html, &[Op::TextOnly]);
        // script contents are still deleted because textOnly only clears tags,
        // not clean_content_tags.
        assert_eq!(cleaned, "boldtext");
    }
}
