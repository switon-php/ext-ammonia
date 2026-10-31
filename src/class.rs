use ext_php_rs::binary_slice::BinarySlice;
use ext_php_rs::exception::PhpException;
use ext_php_rs::prelude::*;

use std::collections::HashMap;

use crate::exception::{ConfigurationException, EncodingException, UrlException};
use crate::policy::{sanitize, Op, SanitizeError};

/// Tag name to attribute name to the values allowed for that attribute.
type AttributeValueMap = HashMap<String, HashMap<String, Vec<String>>>;

/// Tag name to attribute name to the value forced onto that attribute.
type ForcedValueMap = HashMap<String, HashMap<String, String>>;

/// Configured HTML sanitizer. Setters follow ammonia's `Builder` and apply in call order.
#[php_class]
#[php(name = "Switon\\Ammonia\\Builder")]
#[derive(Default)]
pub struct Builder {
    ops: Vec<Op>,
}

fn push(this: &mut Builder, op: Op) -> &mut Builder {
    this.ops.push(op);
    this
}

fn fail(err: SanitizeError) -> PhpException {
    match err {
        SanitizeError::BadUrl(detail) => {
            PhpException::from_class::<UrlException>(format!("invalid URL: {detail}"))
        }
        SanitizeError::Conflict(detail) => PhpException::from_class::<ConfigurationException>(
            format!("ammonia configuration error: {detail}"),
        ),
    }
}

#[php_impl]
impl Builder {
    /// Start from ammonia's default policy.
    pub fn __construct() -> Self {
        Self::default()
    }

    /// Strip all tags and keep only text. `script` and `style` contents are still deleted.
    #[php(name = "textOnly")]
    pub fn text_only(&mut self) -> &mut Self {
        push(self, Op::TextOnly)
    }

    /// Replace the allowed tag set.
    #[php(name = "tags")]
    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::Tags(tags))
    }

    /// Add allowed tags.
    #[php(name = "addTags")]
    pub fn add_tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::AddTags(tags))
    }

    /// Remove allowed tags.
    #[php(name = "rmTags")]
    pub fn rm_tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::RmTags(tags))
    }

    /// Replace tags whose contents are deleted, not just the tags themselves.
    #[php(name = "cleanContentTags")]
    pub fn clean_content_tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::CleanContentTags(tags))
    }

    #[php(name = "addCleanContentTags")]
    pub fn add_clean_content_tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::AddCleanContentTags(tags))
    }

    #[php(name = "rmCleanContentTags")]
    pub fn rm_clean_content_tags(&mut self, tags: Vec<String>) -> &mut Self {
        push(self, Op::RmCleanContentTags(tags))
    }

    /// Replace attributes allowed on every tag.
    #[php(name = "genericAttributes")]
    pub fn generic_attributes(&mut self, attributes: Vec<String>) -> &mut Self {
        push(self, Op::GenericAttributes(attributes))
    }

    #[php(name = "addGenericAttributes")]
    pub fn add_generic_attributes(&mut self, attributes: Vec<String>) -> &mut Self {
        push(self, Op::AddGenericAttributes(attributes))
    }

    #[php(name = "rmGenericAttributes")]
    pub fn rm_generic_attributes(&mut self, attributes: Vec<String>) -> &mut Self {
        push(self, Op::RmGenericAttributes(attributes))
    }

    /// Replace attribute-name prefixes allowed on every tag, such as `data-`.
    #[php(name = "genericAttributePrefixes")]
    pub fn generic_attribute_prefixes(&mut self, prefixes: Vec<String>) -> &mut Self {
        push(self, Op::GenericAttributePrefixes(prefixes))
    }

    #[php(name = "addGenericAttributePrefixes")]
    pub fn add_generic_attribute_prefixes(&mut self, prefixes: Vec<String>) -> &mut Self {
        push(self, Op::AddGenericAttributePrefixes(prefixes))
    }

    #[php(name = "rmGenericAttributePrefixes")]
    pub fn rm_generic_attribute_prefixes(&mut self, prefixes: Vec<String>) -> &mut Self {
        push(self, Op::RmGenericAttributePrefixes(prefixes))
    }

    /// Replace the attributes allowed on each tag. An empty array clears ammonia's defaults.
    #[php(name = "tagAttributes")]
    pub fn tag_attributes(&mut self, attributes: HashMap<String, Vec<String>>) -> &mut Self {
        push(self, Op::TagAttributes(attributes.into_iter().collect()))
    }

    /// Allow more attributes on one tag.
    #[php(name = "addTagAttributes")]
    pub fn add_tag_attributes(&mut self, tag: String, attributes: Vec<String>) -> &mut Self {
        push(
            self,
            Op::AddTagAttributes {
                tag,
                attrs: attributes,
            },
        )
    }

    #[php(name = "rmTagAttributes")]
    pub fn rm_tag_attributes(&mut self, tag: String, attributes: Vec<String>) -> &mut Self {
        push(
            self,
            Op::RmTagAttributes {
                tag,
                attrs: attributes,
            },
        )
    }

    /// Allow specific values of one attribute on one tag.
    #[php(name = "addTagAttributeValues")]
    pub fn add_tag_attribute_values(
        &mut self,
        tag: String,
        attribute: String,
        values: Vec<String>,
    ) -> &mut Self {
        push(
            self,
            Op::AddTagAttributeValues {
                tag,
                attribute,
                values,
            },
        )
    }

    #[php(name = "rmTagAttributeValues")]
    pub fn rm_tag_attribute_values(
        &mut self,
        tag: String,
        attribute: String,
        values: Vec<String>,
    ) -> &mut Self {
        push(
            self,
            Op::RmTagAttributeValues {
                tag,
                attribute,
                values,
            },
        )
    }

    /// Replace the allowed values for each attribute on each tag.
    #[php(name = "tagAttributeValues")]
    pub fn tag_attribute_values(&mut self, values: AttributeValueMap) -> &mut Self {
        push(
            self,
            Op::TagAttributeValues(
                values
                    .into_iter()
                    .map(|(tag, attrs)| (tag, attrs.into_iter().collect()))
                    .collect(),
            ),
        )
    }

    /// Force an attribute value onto a tag when that tag is emitted.
    #[php(name = "setTagAttributeValue")]
    pub fn set_tag_attribute_value(
        &mut self,
        tag: String,
        attribute: String,
        value: String,
    ) -> &mut Self {
        push(
            self,
            Op::SetTagAttributeValue {
                tag,
                attribute,
                value,
            },
        )
    }

    #[php(name = "rmSetTagAttributeValue")]
    pub fn rm_set_tag_attribute_value(&mut self, tag: String, attribute: String) -> &mut Self {
        push(self, Op::RmSetTagAttributeValue { tag, attribute })
    }

    /// Replace the attribute values forced onto tags when they are emitted.
    #[php(name = "setTagAttributeValues")]
    pub fn set_tag_attribute_values(&mut self, values: ForcedValueMap) -> &mut Self {
        push(
            self,
            Op::SetTagAttributeValues(
                values
                    .into_iter()
                    .map(|(tag, attrs)| (tag, attrs.into_iter().collect()))
                    .collect(),
            ),
        )
    }

    /// Replace the allowed URL schemes on `href` and `src`.
    #[php(name = "urlSchemes")]
    pub fn url_schemes(&mut self, schemes: Vec<String>) -> &mut Self {
        push(self, Op::UrlSchemes(schemes))
    }

    #[php(name = "addUrlSchemes")]
    pub fn add_url_schemes(&mut self, schemes: Vec<String>) -> &mut Self {
        push(self, Op::AddUrlSchemes(schemes))
    }

    #[php(name = "rmUrlSchemes")]
    pub fn rm_url_schemes(&mut self, schemes: Vec<String>) -> &mut Self {
        push(self, Op::RmUrlSchemes(schemes))
    }

    /// Leave relative URLs unchanged. This is ammonia's default.
    #[php(name = "passRelativeUrls")]
    pub fn pass_relative_urls(&mut self) -> &mut Self {
        push(self, Op::RelativePass)
    }

    /// Drop relative URLs.
    #[php(name = "denyRelativeUrls")]
    pub fn deny_relative_urls(&mut self) -> &mut Self {
        push(self, Op::RelativeDeny)
    }

    /// Resolve relative URLs against an absolute base.
    #[php(name = "rewriteRelativeUrls")]
    pub fn rewrite_relative_urls(&mut self, base: String) -> &mut Self {
        push(self, Op::RelativeRewriteBase(base))
    }

    /// Resolve relative URLs against a site root, using `path` for the current document.
    #[php(name = "rewriteRelativeUrlsWithRoot")]
    pub fn rewrite_relative_urls_with_root(&mut self, root: String, path: String) -> &mut Self {
        push(self, Op::RelativeRewriteRoot { root, path })
    }

    #[php(name = "addAllowedClasses")]
    pub fn add_allowed_classes(&mut self, tag: String, classes: Vec<String>) -> &mut Self {
        push(self, Op::AddAllowedClasses { tag, classes })
    }

    #[php(name = "rmAllowedClasses")]
    pub fn rm_allowed_classes(&mut self, tag: String, classes: Vec<String>) -> &mut Self {
        push(self, Op::RmAllowedClasses { tag, classes })
    }

    /// Replace the allowed CSS classes for each tag.
    #[php(name = "allowedClasses")]
    pub fn allowed_classes(&mut self, classes: HashMap<String, Vec<String>>) -> &mut Self {
        push(self, Op::AllowedClasses(classes.into_iter().collect()))
    }

    #[php(name = "stripComments")]
    pub fn strip_comments(&mut self, strip: bool) -> &mut Self {
        push(self, Op::StripComments(strip))
    }

    /// Set the `rel` written onto links. `null` disables it.
    #[php(name = "linkRel")]
    pub fn link_rel(&mut self, rel: Option<String>) -> &mut Self {
        push(self, Op::LinkRel(rel))
    }

    /// Prefix every `id` value. `null` disables the prefix.
    #[php(name = "idPrefix")]
    pub fn id_prefix(&mut self, prefix: Option<String>) -> &mut Self {
        push(self, Op::IdPrefix(prefix))
    }

    /// Keep only these CSS properties inside `style` attributes.
    #[php(name = "filterStyleProperties")]
    pub fn filter_style_properties(&mut self, properties: Vec<String>) -> &mut Self {
        push(self, Op::FilterStyleProperties(properties))
    }

    /// Sanitize one HTML fragment with the setters called so far.
    pub fn clean(&self, html: BinarySlice<u8>) -> Result<String, PhpException> {
        let bytes: &[u8] = html.into();
        let html = std::str::from_utf8(bytes).map_err(|_| {
            PhpException::from_class::<EncodingException>("input is not valid UTF-8".to_string())
        })?;
        sanitize(html, &self.ops).map_err(fail)
    }
}
