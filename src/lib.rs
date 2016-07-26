#![crate_name = "objc_foundation"]
#![crate_type = "lib"]

#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate block;

pub use self::array::{
    INSArray, INSMutableArray,
    NSArray, NSComparisonResult, NSMutableArray, NSRange,
    NSMutableSharedArray, NSSharedArray,
};
pub use self::data::{INSData, INSMutableData, NSData, NSMutableData};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::enumerator::{INSFastEnumeration, NSEnumerator, NSFastEnumerator};
pub use self::object::{INSObject, NSObject};
pub use self::string::{INSCopying, INSMutableCopying, INSString, NSString};
pub use self::value::{INSValue, NSValue};

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "Foundation", kind = "framework")]
extern { }

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use objc::runtime::Class;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[link(name = "gnustep-base", kind = "dylib")]
extern { static _OBJC_CLASS_NSObject : Class; }

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[allow(dead_code)]
unsafe fn get_class_to_force_linkage() -> &'static Class {
    &_OBJC_CLASS_NSObject
}

#[macro_use]
mod macros;

mod array;
mod data;
mod dictionary;
mod enumerator;
mod object;
mod string;
mod value;
