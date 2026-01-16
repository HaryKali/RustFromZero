// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// impl Iterator for Counter {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//     }
// }
//
// fn main() {
//     let c = Counter{..}
//     c.next()
// }

// pub trait Iterator<Item> {
//     fn next(&mut self) -> Option<Item>;
// }
//
// pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
//     type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
//     fn is_null(&self) -> bool;
// }
// pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
//     type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
//     fn is_null(&self) -> bool;
// }

// trait Container<A,B> {
//     fn contains(&self,a: A,b: B) -> bool;
// }
//
// fn difference<A,B,C>(container: &C) -> i32
// where
//     C : Container<A,B> {...}
