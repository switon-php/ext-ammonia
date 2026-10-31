<?php

declare(strict_types=1);

use Switon\Ammonia\Builder;
use Switon\Ammonia\ConfigurationException;
use Switon\Ammonia\EncodingException;
use Switon\Ammonia\Exception as AmmoniaException;
use Switon\Ammonia\UrlException;

function check(bool $condition, string $message): void
{
    if (!$condition) {
        fwrite(STDERR, $message . PHP_EOL);
        exit(1);
    }
}

check(class_exists(Builder::class), 'Switon\\Ammonia\\Builder is not loaded');
check(phpversion('ammonia') === '1.0.0', 'module version is not 1.0.0');
ob_start();
phpinfo(INFO_MODULES);
$info = ob_get_clean();
check(str_contains($info, 'Switon'), 'module vendor is missing');
check(str_contains($info, 'https://github.com/switon-php/ext-ammonia'), 'module URL is missing');
check(str_contains($info, 'Sanitizes untrusted HTML'), 'module summary is missing');
check(str_contains($info, '4.2.4'), 'ammonia version is missing from module info');

$clean = (new Builder())->clean('<b><img src="" onerror=alert(1)>ok</b><script>alert(1)</script>');
check(!str_contains($clean, 'onerror'), 'event handler survived');
check(!str_contains($clean, 'script'), 'script survived');
check(str_contains($clean, '<b>'), 'safe tag was removed');

$html = '<div>a</div><p>b</p>';
$replaced = (new Builder())->addTags(['div'])->tags(['p'])->clean($html);
$extended = (new Builder())->tags(['p'])->addTags(['div'])->clean($html);
check(!str_contains($replaced, '<div'), 'tags() did not replace an earlier addTags()');
check(str_contains($extended, '<div>a</div>'), 'addTags() after tags() did not keep div');

$links = '<a href="http://a">h</a><a href="/r">r</a>';
$httpsOnly = (new Builder())->urlSchemes(['https'])->clean($links);
check(!str_contains($httpsOnly, 'http://a'), 'http URL survived a https-only scheme list');
$denied = (new Builder())->denyRelativeUrls()->clean($links);
check(!str_contains($denied, 'href="/r"'), 'relative URL was not denied');
$passed = (new Builder())->denyRelativeUrls()->passRelativeUrls()->clean($links);
check(str_contains($passed, 'href="/r"'), 'later passRelativeUrls() did not override deny');

$kept = (new Builder())->clean('<a href="/r">r</a>');
check(str_contains($kept, 'href='), 'default policy dropped href');
$cleared = (new Builder())->tagAttributes([])->clean('<a href="/r">r</a>');
check(!str_contains($cleared, 'href='), 'empty tagAttributes kept the default href');
$restored = (new Builder())
    ->tagAttributes([])
    ->addTagAttributes('a', ['href'])
    ->clean('<a href="/r">r</a>');
check(str_contains($restored, 'href='), 'addTagAttributes after tagAttributes did not restore href');

try {
    (new Builder())->rewriteRelativeUrls('not a url')->clean('<a href="/a">x</a>');
    check(false, 'bad base URL did not throw');
} catch (UrlException $e) {
    check($e instanceof AmmoniaException, 'UrlException does not extend the base');
    check(str_contains($e->getMessage(), 'invalid URL'), 'bad URL exception has no URL detail');
}

try {
    (new Builder())->clean("\xFF");
    check(false, 'invalid UTF-8 did not throw');
} catch (EncodingException $e) {
    check($e instanceof AmmoniaException, 'EncodingException does not extend the base');
}

try {
    (new Builder())
        ->genericAttributes(['class'])
        ->addAllowedClasses('span', ['hidden'])
        ->clean('<span class="hidden">x</span>');
    check(false, 'contradictory class rules did not throw');
} catch (ConfigurationException $e) {
    check($e instanceof AmmoniaException, 'ConfigurationException does not extend the base');
    check(str_contains($e->getMessage(), 'configuration'), 'conflict exception is unclear');
}

$rmTags = (new Builder())->tags(['p', 'div'])->rmTags(['div'])->clean('<div>a</div><p>b</p>');
check($rmTags === 'a<p>b</p>', 'rmTags left the div tag');

$cleanContent = (new Builder())->rmTags(['b'])->cleanContentTags(['b'])->clean('<b>secret</b><p>ok</p>');
check($cleanContent === '<p>ok</p>', 'cleanContentTags left the deleted content');

$addClean = (new Builder())->rmTags(['i'])->addCleanContentTags(['i'])->clean('<i>no</i>yes');
check($addClean === 'yes', 'addCleanContentTags left the deleted content');

$rmClean = (new Builder())->rmCleanContentTags(['script'])->clean('<script>alert(1)</script>');
check($rmClean === 'alert(1)', 'rmCleanContentTags still deleted the script text');

$generic = (new Builder())->genericAttributes(['title'])->clean('<p title="t" lang="en">x</p>');
check($generic === '<p title="t">x</p>', 'genericAttributes did not replace the default set');

$addGeneric = (new Builder())->addGenericAttributes(['class'])->clean('<p class="x">t</p>');
check($addGeneric === '<p class="x">t</p>', 'addGenericAttributes dropped class');

$rmGeneric = (new Builder())->rmGenericAttributes(['title'])->clean('<p title="t">x</p>');
check($rmGeneric === '<p>x</p>', 'rmGenericAttributes kept title');

$prefix = (new Builder())->genericAttributePrefixes(['data-'])->clean('<b data-val="1">x</b>');
check($prefix === '<b data-val="1">x</b>', 'genericAttributePrefixes dropped data-val');

$addPrefix = (new Builder())->addGenericAttributePrefixes(['data-'])->clean('<b data-val="1">x</b>');
check($addPrefix === '<b data-val="1">x</b>', 'addGenericAttributePrefixes dropped data-val');

$rmPrefix = (new Builder())
    ->addGenericAttributePrefixes(['data-'])
    ->rmGenericAttributePrefixes(['data-'])
    ->clean('<b data-val="1">x</b>');
check($rmPrefix === '<b>x</b>', 'rmGenericAttributePrefixes kept data-val');

$rmTag = (new Builder())->rmTagAttributes('a', ['href'])->clean('<a href="/r">r</a>');
check(!str_contains($rmTag, 'href='), 'rmTagAttributes kept href');

$addValues = (new Builder())
    ->rmTagAttributes('a', ['href'])
    ->addTagAttributeValues('a', 'href', ['/ok'])
    ->clean('<a href="/ok">y</a><a href="/no">n</a>');
check(str_contains($addValues, 'href="/ok"'), 'addTagAttributeValues dropped the allowed href');
check(!str_contains($addValues, 'href="/no"'), 'addTagAttributeValues kept a value outside the list');

$rmValues = (new Builder())
    ->rmTagAttributes('a', ['href'])
    ->addTagAttributeValues('a', 'href', ['/ok'])
    ->rmTagAttributeValues('a', 'href', ['/ok'])
    ->clean('<a href="/ok">y</a>');
check(!str_contains($rmValues, 'href='), 'rmTagAttributeValues kept the removed value');

$setAttr = (new Builder())->setTagAttributeValue('a', 'target', '_blank')->clean('<a href="/r">r</a>');
check(str_contains($setAttr, 'target="_blank"'), 'setTagAttributeValue did not force target');

$rmSet = (new Builder())
    ->setTagAttributeValue('a', 'target', '_blank')
    ->rmSetTagAttributeValue('a', 'target')
    ->clean('<a href="/r">r</a>');
check(!str_contains($rmSet, 'target='), 'rmSetTagAttributeValue kept target');

$addScheme = (new Builder())->addUrlSchemes(['cid'])->clean('<a href="cid:abc">x</a>');
check(str_contains($addScheme, 'href="cid:abc"'), 'addUrlSchemes dropped cid');

$rmScheme = (new Builder())->rmUrlSchemes(['https'])->clean('<a href="https://a">x</a>');
check(!str_contains($rmScheme, 'https://a'), 'rmUrlSchemes kept https');

$rewrite = (new Builder())
    ->rewriteRelativeUrls('https://example.com/dir/page.html')
    ->clean('<a href="next">x</a>');
check(str_contains($rewrite, 'href="https://example.com/dir/next"'), 'rewriteRelativeUrls did not resolve next');

$rewriteRoot = (new Builder())
    ->rewriteRelativeUrlsWithRoot('https://example.com/', 'dir/page.html')
    ->clean('<a href="next">x</a>');
check(
    str_contains($rewriteRoot, 'href="https://example.com/dir/next"'),
    'rewriteRelativeUrlsWithRoot did not resolve next',
);

$classes = (new Builder())->addAllowedClasses('span', ['ok'])->clean('<span class="ok bad">t</span>');
check($classes === '<span class="ok">t</span>', 'addAllowedClasses kept a class outside the list');

$rmClasses = (new Builder())
    ->addAllowedClasses('span', ['ok', 'bad'])
    ->rmAllowedClasses('span', ['bad'])
    ->clean('<span class="ok bad">t</span>');
check($rmClasses === '<span class="ok">t</span>', 'rmAllowedClasses kept the removed class');

$comments = (new Builder())->stripComments(false)->clean('<!-- secret -->ok');
check($comments === '<!-- secret -->ok', 'stripComments(false) removed the comment');

$linkRel = (new Builder())->linkRel(null)->clean('<a href="/r">r</a>');
check($linkRel === '<a href="/r">r</a>', 'linkRel(null) still wrote rel');

$id = (new Builder())->addGenericAttributes(['id'])->idPrefix('user-')->clean('<p id="1">t</p>');
check($id === '<p id="user-1">t</p>', 'idPrefix did not prefix the id');

$style = (new Builder())
    ->addGenericAttributes(['style'])
    ->filterStyleProperties(['color'])
    ->clean('<p style="font-weight: bold; color: red">t</p>');
check($style === '<p style="color:red">t</p>', 'filterStyleProperties kept font-weight');

$keptScript = (new Builder())
    ->cleanContentTags([])
    ->clean('<script>alert(1)</script><style>x</style><p>ok</p>');
check($keptScript === 'alert(1)x<p>ok</p>', 'cleanContentTags([]) still deleted script and style text');

try {
    (new Builder())
        ->rewriteRelativeUrlsWithRoot('not a url', 'index.html')
        ->clean('<a href="/a">x</a>');
    check(false, 'bad root URL did not throw');
} catch (UrlException $e) {
    check(str_contains($e->getMessage(), 'invalid URL'), 'bad root exception has no URL detail');
}

try {
    (new Builder())->addGenericAttributes(['rel'])->clean('<a href="/r">r</a>');
    check(false, 'generic rel with the default linkRel did not throw');
} catch (ConfigurationException $e) {
    check(str_contains($e->getMessage(), 'configuration'), 'generic rel conflict is unclear');
}

try {
    (new Builder())->addTagAttributes('a', ['rel'])->clean('<a href="/r" rel="me">r</a>');
    check(false, 'a rel with the default linkRel did not throw');
} catch (ConfigurationException $e) {
    check(str_contains($e->getMessage(), 'configuration'), 'a rel conflict is unclear');
}

try {
    (new Builder())
        ->rmCleanContentTags(['style'])
        ->addTags(['style'])
        ->filterStyleProperties(['color'])
        ->clean('<style>a{}</style>');
    check(false, 'style element with filterStyleProperties did not throw');
} catch (ConfigurationException $e) {
    check(str_contains($e->getMessage(), 'stylesheet'), 'style filter conflict is unclear');
}

$textOnly = (new Builder())->textOnly()->clean('<b>bold</b><script>alert(1)</script><p>text</p>');
check($textOnly === 'boldtext', 'textOnly() did not strip all tags');

$allowedClasses = (new Builder())
    ->allowedClasses(['span' => ['ok']])
    ->clean('<span class="ok bad">t</span>');
check($allowedClasses === '<span class="ok">t</span>', 'allowedClasses kept a class outside the list');

$tagAttrVals = (new Builder())
    ->rmTagAttributes('a', ['href'])
    ->tagAttributeValues(['a' => ['href' => ['/ok']]])
    ->clean('<a href="/ok">y</a><a href="/no">n</a>');
check(str_contains($tagAttrVals, 'href="/ok"'), 'tagAttributeValues dropped the allowed href');
check(!str_contains($tagAttrVals, 'href="/no"'), 'tagAttributeValues kept a value outside the list');

$setAttrVals = (new Builder())
    ->setTagAttributeValues(['a' => ['target' => '_blank']])
    ->clean('<a href="/r">r</a>');
check(str_contains($setAttrVals, 'target="_blank"'), 'setTagAttributeValues did not force target');

echo "ok\n";
