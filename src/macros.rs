#[macro_export]
macro_rules! object_struct {
    ($name:ident) => (
        #[derive(INSObject)]
        pub struct $name {
            _private: (),
        }
    );
}
