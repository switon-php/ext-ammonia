mod class;
mod exception;
mod policy;

use ext_php_rs::prelude::*;
use ext_php_rs::zend::ModuleEntry;
use ext_php_rs::{info_table_end, info_table_row, info_table_start};

use crate::class::Builder;
use crate::exception::{AmmoniaException, ConfigurationException, EncodingException, UrlException};

/// Shown by `phpinfo()` and `php --ri ammonia`.
pub extern "C" fn module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("Vendor", "Switon");
    info_table_row!("URL", "https://github.com/switon-php/ext-ammonia");
    info_table_row!("Summary", "Sanitizes untrusted HTML");
    info_table_row!("Version", env!("CARGO_PKG_VERSION"));
    info_table_row!("Ammonia", "4.2.4");
    info_table_end!();
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .info_function(module_info)
        .class::<AmmoniaException>()
        .class::<UrlException>()
        .class::<ConfigurationException>()
        .class::<EncodingException>()
        .class::<Builder>()
}
