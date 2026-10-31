# Switon\Ammonia

IDE stubs for the switon HTML sanitizer. One class per file. The directory matches the namespace. Do not `include` these files and do not Composer-autoload them. The extension provides the same classes at runtime.

`Builder` wraps ammonia 4.2.4. Cleaning rules stay in ammonia. Setters are replayed in call order onto ammonia's `Builder`. A later call overrides an earlier one: `addTags(['div'])->tags(['p'])` leaves only `p`, and `tags(['p'])->addTags(['div'])` leaves both.

With no setters, ammonia's defaults apply: common formatting tags, `script` and `style` removed together with their contents, links get `rel="noopener noreferrer"`, and relative URLs are kept. `javascript:` and `data:` URLs are dropped. Protocol-relative URLs such as `//host` are kept.

`attribute_filter` and a custom relative-URL callback are not exposed. Both would call PHP during cleaning. `clean_from_reader` is not exposed either, because PHP has no equivalent of Rust's `Read` trait. The `clone_*`, `is_url_relative_*`, `get_link_rel`, and `will_strip_comments` methods are readers on ammonia's builder. This class only records setters, so there is no live state to read back; run `clean()` and inspect its output instead.

## Recommended use

Ammonia's default is the agreed policy for an untrusted fragment that should remain HTML: a comment, markdown output, or rich text mixed into a page. It is aimed at cross-site scripting, layout breaking, and clickjacking. It does not turn newlines into paragraphs or linkify bare URLs. Run the markup processor first, then `clean()`.

```php
$cleaner = new Builder();
$safe = $cleaner->clean($html);
```

Keep one `Builder` per policy and call `clean()` per fragment. With no setters the result already keeps common formatting, lists, tables, links, and images; deletes `script` and `style` together with their contents; drops `javascript:` and `data:` URLs; omits `style`, `class`, `id`, and event handlers; strips comments; and writes `rel="noopener noreferrer"` on links.

For user content on a site you control, tighten schemes and relative URLs. The default scheme list is the older broad set (`ftp`, `ssh`, `bitcoin`, `magnet`, and others), and relative URLs are passed through, including `/admin` and `//host`.

```php
$cleaner = (new Builder())
    ->urlSchemes(['https', 'mailto'])
    ->denyRelativeUrls()
    ->linkRel('noopener noreferrer ugc');
```

Add `tel` when phone links are part of the product. `ugc` marks the link as user-generated. Put `nofollow` in that same string when the product still wants the older ranking hint. `noopener noreferrer` is the part that blocks the tab-hijack ammonia already blocks by default.

If the editor must keep same-site links such as `/docs/a`, do not deny relative URLs. Resolve them:

```php
$cleaner = (new Builder())
    ->urlSchemes(['https', 'mailto'])
    ->rewriteRelativeUrls('https://example.com/docs/current')
    ->linkRel('noopener noreferrer ugc');
```

To keep the words and drop the markup, for a search index or an excerpt, call `textOnly()`. No tags are allowed, so all markup is stripped and only text remains. `script` and `style` contents are still deleted.

```php
$cleaner = (new Builder())->textOnly();
```

Do not start from `tags(['p', 'a', 'b'])` unless that list is the product's allow list. `tags()` replaces the whole default set. Add a tag the renderer emits with `addTags()`, and remove one with `rmTags()`.

## Classes

| Class | Role |
| --- | --- |
| `Switon\Ammonia\Builder` | Configured sanitizer. |
| `Switon\Ammonia\Exception` | Base type for failures from `clean()`. |
| `Switon\Ammonia\UrlException` | A rewrite base or root is not a valid URL. |
| `Switon\Ammonia\ConfigurationException` | The setter combination is one ammonia rejects. |
| `Switon\Ammonia\EncodingException` | The fragment passed to `clean()` is not valid UTF-8. |

`UrlException`, `ConfigurationException`, and `EncodingException` are thrown by `clean()`, not by the setters. Setters only record configuration.

## Methods

| PHP | ammonia |
| --- | --- |
| `tags` / `addTags` / `rmTags` | `tags` / `add_tags` / `rm_tags` |
| `cleanContentTags` / `addCleanContentTags` / `rmCleanContentTags` | `clean_content_tags` / `add_clean_content_tags` / `rm_clean_content_tags` |
| `genericAttributes` / `addGenericAttributes` / `rmGenericAttributes` | `generic_attributes` / `add_generic_attributes` / `rm_generic_attributes` |
| `genericAttributePrefixes` / `addGenericAttributePrefixes` / `rmGenericAttributePrefixes` | `generic_attribute_prefixes` / `add_generic_attribute_prefixes` / `rm_generic_attribute_prefixes` |
| `tagAttributes` | `tag_attributes`. An empty array clears the default per-tag attributes. |
| `addTagAttributes` / `rmTagAttributes` | `add_tag_attributes` / `rm_tag_attributes` |
| `addTagAttributeValues` / `rmTagAttributeValues` | `add_tag_attribute_values` / `rm_tag_attribute_values` |
| `tagAttributeValues` | `tag_attribute_values`. An empty array clears all value restrictions. |
| `setTagAttributeValue` / `rmSetTagAttributeValue` | `set_tag_attribute_value` / `rm_set_tag_attribute_value` |
| `setTagAttributeValues` | `set_tag_attribute_values`. An empty array clears all forced values. |
| `urlSchemes` / `addUrlSchemes` / `rmUrlSchemes` | `url_schemes` / `add_url_schemes` / `rm_url_schemes` |
| `passRelativeUrls` | `url_relative(PassThrough)` |
| `denyRelativeUrls` | `url_relative(Deny)` |
| `rewriteRelativeUrls($base)` | `url_relative(RewriteWithBase)` |
| `rewriteRelativeUrlsWithRoot($root, $path)` | `url_relative(RewriteWithRoot)` |
| `addAllowedClasses` / `rmAllowedClasses` | `add_allowed_classes` / `rm_allowed_classes` |
| `allowedClasses` | `allowed_classes`. An empty array clears all class allow lists. |
| `stripComments($strip)` | `strip_comments`. Comments are removed by default. |
| `linkRel($rel)` | `link_rel`. `null` writes no `rel`. The default is `noopener noreferrer`. |
| `idPrefix($prefix)` | `id_prefix`. `null` clears the prefix. No prefix is added by default. |
| `filterStyleProperties` | `filter_style_properties` |
| `textOnly()` | `empty`. Clears all tags, so only text remains. |
| `clean($html)` | `clean` |

Defaults and cleaning details that are not listed here follow [ammonia `Builder`](https://docs.rs/ammonia/4.2.4/ammonia/struct.Builder.html).

## Pitfalls

Setters only record options. ammonia checks the combination inside `clean()`, and a rejected combination becomes `ConfigurationException`. A bad rewrite base or root becomes `UrlException` at the same point.

`addTagAttributeValues` does not narrow an attribute that is already allowed. The value list is a second way in, not a filter. Defaults already allow `a href` and `img src`, so `addTagAttributeValues('a', 'href', ['/ok'])` still keeps every other `href`. Remove the attribute first:

```php
->rmTagAttributes('a', ['href'])
->addTagAttributeValues('a', 'href', ['/ok'])
```

A tag in `cleanContentTags` or `addCleanContentTags` must not also be an allowed tag, and must not have an entry in `tagAttributes`. Defaults allow `b` and `i`, and give `a` and `img` their own attributes. `addCleanContentTags(['b'])` throws until `rmTags(['b'])`. Deleting the contents of `a` also needs that tag dropped from `tagAttributes`. `cleanContentTags` replaces the whole list, so `script` and `style` stop having their contents deleted unless they are put back.

`rmCleanContentTags(['script'])` stops deleting the contents. The tag is still not allowed, so `<script>alert(1)</script>` becomes the text `alert(1)`.

`linkRel` defaults to `noopener noreferrer`. Allowing `rel` on every tag, or `rel` on `a`, throws while that default is set. Pass `linkRel(null)` before allowing caller-supplied `rel`.

`addAllowedClasses` throws when `class` is also a generic attribute, or an attribute of that same tag. `filterStyleProperties` throws when the `style` element is an allowed tag. It also does nothing until `style` is an allowed attribute. `idPrefix` does nothing until `id` is an allowed attribute.

Under ZTS, `clean()` calls are serialized in the process. ammonia reports the conflicts above by panicking, and the extension swaps the process-wide panic hook for the duration of the call.

## Layout for the next extension

```
stubs/<Namespace>/<Class>.php
stubs/README.md
```

Each PHP file is one class, named after that class, with English PHPDoc. The extension README keeps a short example and points here for the method list.
