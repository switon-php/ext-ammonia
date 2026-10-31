<?php

declare(strict_types=1);

namespace Switon\Ammonia;

/**
 * Sanitizes an untrusted HTML fragment.
 *
 * This is the PHP surface of the switon HTML sanitizer. It wraps ammonia 4.2.4,
 * a Rust whitelist sanitizer that parses the fragment with the HTML5 parser html5ever
 * and drops anything the policy does not allow. The walk stays in the extension.
 * Tag lists, URL schemes, and the other cleaning rules stay in ammonia. Setters on
 * this class are replayed in call order, and {@see clean()} returns the sanitized fragment.
 *
 * Start with no setters. That is the policy for an untrusted fragment that should stay HTML.
 * For user content on a site, restrict schemes to `https` and `mailto`, deny relative URLs
 * (or rewrite them onto the site), and set `linkRel` to `noopener noreferrer ugc`.
 * Build one instance per policy and reuse it.
 *
 * IDE stub. The extension provides this class at runtime. Do not include this file.
 */
class Builder
{
    /**
     * Start from ammonia's default policy.
     */
    public function __construct()
    {
    }

    /**
     * Strip all tags and keep only text.
     *
     * No tags are allowed, so all markup is stripped and only text remains.
     * `script` and `style` contents are still deleted.
     */
    public function textOnly(): self
    {
    }

    /**
     * Replace the allowed tags.
     *
     * The default set is common formatting, tables, links, and images. It does not include `script`, `iframe`, `form`, or `svg`.
     *
     * @param list<string> $tags
     */
    public function tags(array $tags): self
    {
    }

    /**
     * Add tags to the allowed set.
     *
     * @param list<string> $tags
     */
    public function addTags(array $tags): self
    {
    }

    /**
     * Remove tags from the allowed set.
     *
     * @param list<string> $tags
     */
    public function rmTags(array $tags): self
    {
    }

    /**
     * Replace the tags whose contents are deleted, not just the tags themselves.
     *
     * The default is `script` and `style`. This replaces that list, so `script` and `style` are no longer content-deleted unless they are included again.
     *
     * A tag here must not also be allowed, and must not have an entry in {@see tagAttributes()}. Defaults already allow tags such as `b`, and give `a` and `img` their own attributes. {@see clean()} throws {@see ConfigurationException} when either overlap remains. Remove the tag with {@see rmTags()} first, and drop its {@see tagAttributes()} entry when it has one.
     *
     * @param list<string> $tags
     */
    public function cleanContentTags(array $tags): self
    {
    }

    /**
     * Add tags whose contents are deleted.
     *
     * Same overlap rules as {@see cleanContentTags()}. A default-allowed tag such as `b` must be removed with {@see rmTags()} first.
     *
     * @param list<string> $tags
     */
    public function addCleanContentTags(array $tags): self
    {
    }

    /**
     * Remove tags from the content-deletion list.
     *
     * The tag is still stripped when it is not allowed, but its contents stay as text. `rmCleanContentTags(['script'])` turns `<script>alert(1)</script>` into `alert(1)`.
     *
     * @param list<string> $tags
     */
    public function rmCleanContentTags(array $tags): self
    {
    }

    /**
     * Replace the attributes allowed on every tag.
     *
     * The default is `lang` and `title`. `class`, `style`, `id`, and event handlers are not included.
     *
     * @param list<string> $attributes
     */
    public function genericAttributes(array $attributes): self
    {
    }

    /**
     * Add attributes allowed on every tag.
     *
     * @param list<string> $attributes
     */
    public function addGenericAttributes(array $attributes): self
    {
    }

    /**
     * Remove attributes from the set allowed on every tag.
     *
     * @param list<string> $attributes
     */
    public function rmGenericAttributes(array $attributes): self
    {
    }

    /**
     * Replace the attribute-name prefixes allowed on every tag, such as `data-`.
     *
     * No prefixes are allowed by default.
     *
     * @param list<string> $prefixes
     */
    public function genericAttributePrefixes(array $prefixes): self
    {
    }

    /**
     * Add attribute-name prefixes allowed on every tag.
     *
     * @param list<string> $prefixes
     */
    public function addGenericAttributePrefixes(array $prefixes): self
    {
    }

    /**
     * Remove attribute-name prefixes allowed on every tag.
     *
     * @param list<string> $prefixes
     */
    public function rmGenericAttributePrefixes(array $prefixes): self
    {
    }

    /**
     * Replace the attributes allowed on each tag.
     *
     * An empty array clears ammonia's defaults, such as `href` on `a` and `src` on `img`. {@see addTagAttributes()} can still add attributes afterward.
     *
     * @param array<string, list<string>> $attributes Tag name to attribute names.
     */
    public function tagAttributes(array $attributes): self
    {
    }

    /**
     * Add attributes allowed on one tag.
     *
     * @param list<string> $attributes
     */
    public function addTagAttributes(string $tag, array $attributes): self
    {
    }

    /**
     * Remove attributes allowed on one tag.
     *
     * @param list<string> $attributes
     */
    public function rmTagAttributes(string $tag, array $attributes): self
    {
    }

    /**
     * Allow only these values for one attribute on one tag.
     *
     * This does not narrow an attribute that is already allowed. The value list is a second way in. Defaults allow `a href` and `img src`, so those attributes must be removed with {@see rmTagAttributes()} before this list can reject other values.
     *
     * @param list<string> $values
     */
    public function addTagAttributeValues(string $tag, string $attribute, array $values): self
    {
    }

    /**
     * Remove values from one tag attribute's allow list.
     *
     * @param list<string> $values
     */
    public function rmTagAttributeValues(string $tag, string $attribute, array $values): self
    {
    }

    /**
     * Replace the allowed values for each attribute on each tag.
     *
     * An empty array clears all value restrictions. Defaults have none, so attributes
     * that are allowed accept any value. Use this to set the whole map at once instead
     * of calling {@see addTagAttributeValues()} per tag and attribute.
     *
     * @param array<string, array<string, list<string>>> $values Tag name to attribute name to allowed values.
     */
    public function tagAttributeValues(array $values): self
    {
    }

    /**
     * Force an attribute value onto a tag when that tag is emitted, for example `target="_blank"` on `a`.
     */
    public function setTagAttributeValue(string $tag, string $attribute, string $value): self
    {
    }

    /**
     * Stop forcing the attribute value set by {@see setTagAttributeValue()}.
     */
    public function rmSetTagAttributeValue(string $tag, string $attribute): self
    {
    }

    /**
     * Replace the attribute values forced onto tags when they are emitted.
     *
     * An empty array clears all forced values. Use this to set the whole map at once
     * instead of calling {@see setTagAttributeValue()} per tag and attribute.
     *
     * @param array<string, array<string, string>> $values Tag name to attribute name to forced value.
     */
    public function setTagAttributeValues(array $values): self
    {
    }

    /**
     * Replace the URL schemes allowed on `href` and `src`.
     *
     * The default includes `http`, `https`, `mailto`, `tel`, and `ftp`. It does not include `javascript` or `data`.
     *
     * @param list<string> $schemes
     */
    public function urlSchemes(array $schemes): self
    {
    }

    /**
     * Add allowed URL schemes.
     *
     * @param list<string> $schemes
     */
    public function addUrlSchemes(array $schemes): self
    {
    }

    /**
     * Remove allowed URL schemes.
     *
     * @param list<string> $schemes
     */
    public function rmUrlSchemes(array $schemes): self
    {
    }

    /**
     * Leave relative URLs unchanged.
     *
     * This is the default. Protocol-relative URLs such as `//host` are kept.
     */
    public function passRelativeUrls(): self
    {
    }

    /**
     * Drop relative URLs.
     */
    public function denyRelativeUrls(): self
    {
    }

    /**
     * Resolve relative URLs against an absolute base.
     *
     * {@see clean()} throws {@see UrlException} when `$base` is not a valid URL.
     */
    public function rewriteRelativeUrls(string $base): self
    {
    }

    /**
     * Resolve relative URLs against a site root. `$path` is the current document path.
     *
     * {@see clean()} throws {@see UrlException} when `$root` is not a valid URL.
     */
    public function rewriteRelativeUrlsWithRoot(string $root, string $path): self
    {
    }

    /**
     * Allow these CSS classes on one tag.
     *
     * Do not also allow `class` as a generic attribute or as an attribute of this tag. {@see clean()} throws {@see ConfigurationException} when both are set.
     *
     * @param list<string> $classes
     */
    public function addAllowedClasses(string $tag, array $classes): self
    {
    }

    /**
     * Remove CSS classes from one tag's allow list.
     *
     * @param list<string> $classes
     */
    public function rmAllowedClasses(string $tag, array $classes): self
    {
    }

    /**
     * Replace the allowed CSS classes for each tag.
     *
     * An empty array clears all class allow lists. Do not also allow `class` as a
     * generic attribute or as an attribute of the same tag. {@see clean()} throws
     * {@see ConfigurationException} when both are set.
     *
     * @param array<string, list<string>> $classes Tag name to allowed CSS classes.
     */
    public function allowedClasses(array $classes): self
    {
    }

    /**
     * Whether HTML comments are removed.
     *
     * Comments are removed by default.
     */
    public function stripComments(bool $strip): self
    {
    }

    /**
     * Set the `rel` attribute written onto links.
     *
     * The default is `noopener noreferrer`. `null` writes no `rel`. If callers may supply their own `rel`, pass `null`, or {@see clean()} throws {@see ConfigurationException}.
     */
    public function linkRel(?string $rel): self
    {
    }

    /**
     * Prefix every `id` value.
     *
     * No prefix is added by default. `null` clears a prefix set earlier. `id` is kept only when it is also an allowed attribute.
     */
    public function idPrefix(?string $prefix): self
    {
    }

    /**
     * Keep only these CSS properties inside `style` attributes.
     *
     * This has no effect unless `style` is an allowed attribute. It throws {@see ConfigurationException} from {@see clean()} when the `style` element itself is an allowed tag.
     *
     * @param list<string> $properties
     */
    public function filterStyleProperties(array $properties): self
    {
    }

    /**
     * Sanitize one HTML fragment with the setters called so far.
     *
     * @throws EncodingException `$html` is not valid UTF-8.
     * @throws UrlException A rewrite base or root cannot be parsed as a URL.
     * @throws ConfigurationException The setter combination is one ammonia rejects.
     */
    public function clean(string $html): string
    {
    }
}
