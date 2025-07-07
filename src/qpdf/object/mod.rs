pub struct ObjectHandler {}

pub trait Manage<T> {
    fn get(&self, key: String) -> T;
    fn create(&self, key: String, val: T);
    fn replace(&self, key: String, val: T);
}
