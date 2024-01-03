//! Inspect macOS system for locale configuration

use objc::runtime::Object;
use objc_foundation::{INSString, NSString};

use super::{LanguageRange, Locale};

pub fn system_locale() -> Option<Locale> {
    let locale_identifier = unsafe {
        let nslocale = class!(NSLocale);
        let current_locale: *mut Object = msg_send![nslocale, currentLocale];
        let locale_identifier: *const NSString = msg_send![current_locale, localeIdentifier];
        locale_identifier.as_ref().unwrap()
    };
    // newer macOS may add additional information in the format of @key=value,...
    let id = locale_identifier.as_str().splitn(2, "@").next()?;
    LanguageRange::from_unix(id)
        .ok()
        .map(Locale::from)
}
