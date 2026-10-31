# ext-ammonia

PHP extension. `Switon\Ammonia\Builder` sanitizes HTML with ammonia 4.2.4. Supported platforms are macOS Apple Silicon, Linux glibc (`x86_64` and `arm64`), and Windows x64, PHP 8.2–8.5, NTS and ZTS. Contributor conventions are in [AGENTS.md](AGENTS.md).

## Ammonia

[Ammonia](https://github.com/rust-ammonia/ammonia) is a whitelist HTML sanitizer written in Rust. It parses a fragment with [html5ever](https://github.com/servo/html5ever), the HTML5 parser, walks that tree, and keeps only tags, attributes, and URL schemes the policy allows. The result is still HTML. It does not insert paragraphs for newlines or turn bare URLs into links, so a markup processor runs first when the source is Markdown.

What we rely on:

- The parse happens before any filtering. Unclosed tags, misnested tags, and comment tricks are turned into a tree the way a browser parses a fragment. The policy never matches the raw string.
- The default policy is already the one for untrusted HTML that should stay HTML. Formatting, lists, tables, links, and images stay. `script` and `style` are removed with their contents. Event handlers, `class`, `style`, and `id` are not in the default allow lists. `javascript:` and `data:` are dropped. Each link gets `rel="noopener noreferrer"`, so a new tab cannot reach back into the page that opened it.
- The walk stays inside the extension. `clean()` does not call back into PHP. That is the speed gap in the table below.
- Setters only store a policy, in call order. Build one `Builder` and reuse it for every fragment that shares that policy.

HTML Purifier still goes further when the product needs CSS filtering. Ammonia stops at the fragment policy above.

```php
use Switon\Ammonia\Builder;

// Untrusted HTML that should stay HTML: comments, markdown output, rich text.
$cleaner = new Builder();
$safe = $cleaner->clean($input);
```

Build one `Builder` per policy and reuse it. The no-setter form is ammonia's default: formatting, lists, tables, links, and images stay; `script` and `style` are removed with their contents; `javascript:` and `data:` URLs are dropped; links get `rel="noopener noreferrer"`.

User content on a site you control usually adds three calls. The default scheme list still contains `ftp`, `ssh`, and `bitcoin`, and relative URLs such as `/admin` and `//host` are kept.

```php
$cleaner = (new Builder())
    ->urlSchemes(['https', 'mailto'])
    ->denyRelativeUrls()
    ->linkRel('noopener noreferrer ugc');
```

Use `rewriteRelativeUrls($base)` instead of `denyRelativeUrls()` when the product's own editor must keep links like `/docs/a`. Other policies, including stripping a fragment down to text, are in [stubs/README.md](stubs/README.md), with the method list, defaults, exceptions, and pitfalls.

## Compared with PHP

The same job in pure PHP is [Symfony HtmlSanitizer](https://symfony.com/doc/current/html_sanitizer.html) or [HTML Purifier](http://htmlpurifier.org/). Both are whitelist sanitizers for an untrusted fragment. Neither needs a native build, and HTML Purifier still has the larger configuration surface, including CSS. Symfony is the current pure-PHP HTML5 option (`allowSafeElements()`). This extension is ammonia: the parsing and the walk run in the extension, and the policies above are the ones to start from. Output is not byte-for-byte the same. ammonia adds `rel` on links and keeps a different default tag set.

One local run, PHP 8.3.30 NTS on Apple Silicon, after warmup. Symfony HtmlSanitizer 7.4.22 and HTML Purifier 4.19.1 used their usual safe defaults. The Purifier definition cache was already warm.

| Fragment | ammonia | Symfony | HTML Purifier |
| --- | --- | --- | --- |
| 300 bytes | 16 µs | 120 µs | 250 µs |
| 12 KB | 0.4 ms | 4.3 ms | 6.1 ms |

That is about 7× Symfony and 15× HTML Purifier on the short fragment, and about 11× and 15× on the 12 KB fragment. It is one machine, not a portability claim. Under ZTS, `clean()` calls in one process run one at a time, so the throughput gain is smaller than the single-call numbers.

`make bench` repeats that run. It installs the two PHP libraries under `bench/vendor` (not committed) and is separate from `make test`.

## Build

The library matches the `php` and `php-config` on `PATH` while it compiles. Those two commands must print the same version. On Windows there is no `php-config`; the build reads `php -v` and links the matching `php8.lib` or `php8ts.lib`. The result loads only on that PHP minor version, that NTS or ZTS build, and that CPU. macOS and Linux need Rust 1.85 or newer. Windows needs Rust nightly.

From a clone of this repository:

```bash
make test
```

`make test` runs the Rust tests, builds the release library, and loads it. The file is `target/release/libammonia.dylib` on macOS, `target/release/libammonia.so` on Linux, and `target/release/ammonia.dll` on Windows. `php -m` lists the module as `ammonia`. The classes stay under `Switon\Ammonia`.

```bash
php -d extension="$PWD/target/release/libammonia.so" -m
```

On macOS change the filename to `libammonia.dylib`. On Windows use `ammonia.dll`.

### Homebrew

Apple Silicon. The Command Line Tools supply clang.

```bash
xcode-select --install
brew install rust php
export PATH="$(brew --prefix php)/bin:$PATH"
php -v
php-config --version
rustc --version
make test
```

### apt

Debian and Ubuntu. The `rustc` package is often older than 1.85, so Rust comes from rustup. `php-dev` follows the distribution's default PHP, which must be 8.2–8.5.

```bash
sudo apt update
sudo apt install build-essential clang libclang-dev pkg-config php-cli php-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
php -v
php-config --version
rustc --version
make test
```

To compile against a specific minor version, install that pair instead, for example `php8.3-cli` and `php8.3-dev`, and put that `php` and `php-config` first on `PATH`.

### yum

RHEL, AlmaLinux, Rocky Linux, and Fedora. On Fedora the same packages install with `dnf`. The distro `rust` package is often older than 1.85, so Rust comes from rustup. `php-devel` must be PHP 8.2–8.5.

```bash
sudo yum install gcc make clang clang-devel pkgconf-pkg-config php-cli php-devel curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
php -v
php-config --version
rustc --version
make test
```

A ZTS library is a second build. Point `PATH` and `PHP_CONFIG` at the ZTS `php` and `php-config`, then run `make test` again. On Windows, put the ZTS `php.exe` first on `PATH` instead.

### Windows

Windows x64. PHP is a [windows.php.net](https://windows.php.net/) build, VS16 for 8.2 and 8.3, VS17 for 8.4 and 8.5. Visual Studio 2022 Build Tools with the C++ workload supplies `cl.exe`. LLVM supplies `libclang`. Rust nightly is required: the Windows build uses the `vectorcall` ABI, which stable Rust does not provide. Run the commands from Git Bash so `make` can see `uname`.

```bash
rustup toolchain install nightly --component rustfmt
choco install llvm make -y
export LIBCLANG_PATH="/c/Program Files/LLVM/bin"
export PATH="/c/php:$PATH"
php -v
rustc +nightly --version
make test
```

Put the chosen PHP minor, NTS or ZTS, on `PATH` as `/c/php`. The machine also needs the Visual C++ Redistributable listed for that PHP build. `ammonia.dll` links `php8.dll` or `php8ts.dll` and that runtime. It does not ship a second project library.

The extension version comes from the git tag. A tag `v1.0.0` builds version `1.0.0`, and `php --ri ammonia` reports that. A commit after the tag is `1.0.0` plus the distance and commit, such as `1.0.0-3-gabcdef`. With no tag in the clone, the build uses the `version` in `Cargo.toml`. A release asset is `ammonia-1.0.0-php8.3-nts-linux-x64.tar.gz`: that version, the PHP minor, `nts` or `zts`, `linux`, `macos`, or `windows`, `x64` or `arm64`. Unpack it. The only file inside is `libammonia.so` on Linux, `libammonia.dylib` on macOS, or `ammonia.dll` on Windows. Load the file:

```ini
extension=/path/libammonia.so
```

`extension=ammonia` looks for `ammonia.so` on Linux and macOS, and for `php_ammonia.dll` on Windows. It does not load `libammonia.so` or `ammonia.dll`. On Windows the line is `extension=C:/path/ammonia.dll`. The file still has to match the PHP minor version, NTS or ZTS, and the CPU. A Linux build from Ubuntu 24.04 needs that glibc or a newer one. Windows builds are x64.

## CI

[`.github/workflows/ci.yml`](.github/workflows/ci.yml) runs `make test` on every push and pull request.

| Runner | What it covers |
| --- | --- |
| `ubuntu-24.04` | PHP 8.2–8.5, NTS and ZTS, glibc `x86_64` |
| `ubuntu-24.04-arm` | PHP 8.3 and 8.5 NTS, PHP 8.5 ZTS, glibc `arm64` |
| `macos-15` | PHP 8.3 and 8.5 NTS, PHP 8.5 ZTS, Apple Silicon |
| `windows-latest` | PHP 8.3 and 8.5 NTS, PHP 8.5 ZTS, x64 |

A `v*` tag runs [`.github/workflows/release.yml`](.github/workflows/release.yml). That build is the full set: PHP 8.2–8.5, NTS and ZTS, on all four runners. Each archive contains one library, and CI rejects a build that links a second project library. `make bench` is not part of CI.
