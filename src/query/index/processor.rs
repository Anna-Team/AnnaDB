// use crate::response::QueryResponse;
// use crate::{DBError, PathToValue, Storage};
//
// pub fn index(
//     storage: &Storage,
//     collection_name: String,
//     path: PathToValue,
// ) -> Result<QueryResponse, DBError> {
//     match storage.get_collection(collection_name.to_string()) {
//         Some(collection) => {
//             fs::remove_file(collection.get_path(self.wh_path.clone()))?; // TODO clean internal collection too
//             self.warehouse.remove(collection_name);
//         }
//         _ => {}
//     };
//     Ok(QueryResponse::new(
//         Item::Vector(VectorItem::ResponseIds(links)),
//         Meta::InsertMeta(meta),
//         QueryStatus::Ready,
//     ))
// }
