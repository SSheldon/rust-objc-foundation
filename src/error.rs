use objc_id::Id;
use {INSObject, NSInteger, NSDictionary, NSString, INSString};

pub type NSErrorUserInfoKey = NSString;

#[allow(non_snake_case)]
pub fn NSLocalizedDescriptionKey() -> Id<NSString> {
    NSString::from_str(&"NSLocalizedDescription")
}

pub trait INSError: INSObject {
    fn new<T>(code: NSInteger, domain: Id<NSString>, user_info: Id<NSDictionary<NSErrorUserInfoKey, T>>) -> Id<Self> {
        let cls = Self::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithDomain:domain
                                                          code:code
                                                      userInfo:user_info];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSError);

impl INSError for NSError {}

