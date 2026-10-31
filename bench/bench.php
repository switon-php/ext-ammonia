<?php

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';

use Switon\Ammonia\Builder;
use Symfony\Component\HtmlSanitizer\HtmlSanitizer;
use Symfony\Component\HtmlSanitizer\HtmlSanitizerConfig;

if (!class_exists(Builder::class)) {
    fwrite(STDERR, "switon extension is not loaded\n");
    exit(1);
}

$comment = <<<'HTML'
<p>Hello <b>there</b> <a href="https://example.com/a">link</a> <a href="javascript:alert(1)">bad</a></p>
<img src="https://example.com/a.png" alt="x" onerror="alert(1)">
<script>alert(1)</script>
<style>body{background:url(javascript:alert(1))}</style>
<ul><li>one</li><li>two</li></ul>
<!-- secret -->
HTML;
$article = str_repeat($comment, 40);

$cache = __DIR__ . '/cache';
if (!is_dir($cache) && !mkdir($cache, 0777, true) && !is_dir($cache)) {
    fwrite(STDERR, "cannot create {$cache}\n");
    exit(1);
}

$ammonia = new Builder();
$symfony = new HtmlSanitizer((new HtmlSanitizerConfig())->allowSafeElements());
$config = HTMLPurifier_Config::createDefault();
$config->set('Cache.SerializerPath', $cache);
$purifier = new HTMLPurifier($config);
$purifier->purify($comment);

/**
 * @param callable(string): string $fn
 */
function microseconds(string $html, int $iterations, callable $fn): float
{
    for ($i = 0; $i < 20; $i++) {
        $fn($html);
    }
    $start = hrtime(true);
    for ($i = 0; $i < $iterations; $i++) {
        $fn($html);
    }

    return (hrtime(true) - $start) / $iterations / 1000;
}

$rows = [
    ['comment', $comment, 4000, 1000, 400],
    ['article', $article, 400, 100, 40],
];

printf("%-8s %7s %10s %10s %10s\n", 'fragment', 'bytes', 'ammonia', 'symfony', 'purifier');
foreach ($rows as [$name, $html, $ammoniaN, $symfonyN, $purifierN]) {
    printf(
        "%-8s %7d %8.1f us %8.1f us %8.1f us\n",
        $name,
        strlen($html),
        microseconds($html, $ammoniaN, static fn (string $h): string => $ammonia->clean($h)),
        microseconds($html, $symfonyN, static fn (string $h): string => $symfony->sanitize($h)),
        microseconds($html, $purifierN, static fn (string $h): string => $purifier->purify($h)),
    );
}
