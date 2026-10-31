use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;

/// Base type for every failure raised by `Switon\Ammonia`.
#[php_class]
#[php(name = "Switon\\Ammonia\\Exception")]
#[php(extends(ce = ce::exception, stub = "Exception"))]
#[derive(Default)]
pub struct AmmoniaException;

/// `rewriteRelativeUrls` or `rewriteRelativeUrlsWithRoot` was given a base PHP cannot parse as a URL.
#[php_class]
#[php(name = "Switon\\Ammonia\\UrlException")]
#[php(extends(AmmoniaException))]
#[derive(Default)]
pub struct UrlException;

/// The setter combination is one ammonia rejects, such as allowing `class` both freely and by whitelist.
#[php_class]
#[php(name = "Switon\\Ammonia\\ConfigurationException")]
#[php(extends(AmmoniaException))]
#[derive(Default)]
pub struct ConfigurationException;

/// The fragment passed to `clean` is not valid UTF-8.
#[php_class]
#[php(name = "Switon\\Ammonia\\EncodingException")]
#[php(extends(AmmoniaException))]
#[derive(Default)]
pub struct EncodingException;
