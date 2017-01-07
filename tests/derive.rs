extern crate objc;
extern crate objc_foundation;
#[macro_use]
extern crate objc_foundation_derive;

use objc_foundation::INSObject;

#[derive(INSObject)]
struct NSObject {
    _private: (),
}

#[test]
fn test_derive() {
    let cls = NSObject::class();
    assert_eq!(cls.name(), "NSObject");
}
