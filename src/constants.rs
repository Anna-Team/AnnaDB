pub const STRING: &str = "s";
pub const NUMBER: &str = "n";
pub const BOOL: &str = "b";
pub const NULL: &str = "null";
pub const KEEP: &str = "keep";
pub const COLLECTION_NAME: &str = "collection";
pub const PATH_TO_VALUE: &str = "value";
pub const DELETED: &str = "deleted";
pub const UTS: &str = "uts";

pub const STORAGE_MAP: &str = "m";

pub const STORAGE_VECTOR: &str = "v";

// QUERIES
pub const QUERY_SET: &str = "q";
pub const INSERT_QUERY: &str = "insert";
pub const FIND_QUERY: &str = "find";
pub const GET_QUERY: &str = "get";
pub const UPDATE_QUERY: &str = "update";
pub const DELETE_QUERY: &str = "delete";
pub const SORT_QUERY: &str = "sort";
pub const LIMIT_QUERY: &str = "limit";
pub const OFFSET_QUERY: &str = "offset";
pub const PROJECT_QUERY: &str = "project";

// FIND OPERATORS
pub const EQ_OPERATOR: &str = "eq";
pub const NEQ_OPERATOR: &str = "neq";
pub const GT_OPERATOR: &str = "gt";
pub const GTE_OPERATOR: &str = "gte";
pub const LT_OPERATOR: &str = "lt";
pub const LTE_OPERATOR: &str = "lte";

pub const AND_OPERATOR: &str = "and";
pub const OR_OPERATOR: &str = "or";

pub const NOT_OPERATOR: &str = "not";

// SORT OPERATORS
pub const ASC_OPERATOR: &str = "asc";
pub const DESC_OPERATOR: &str = "desc";

// SET OPERATORS
pub const SET_OPERATOR: &str = "set";
pub const INC_OPERATOR: &str = "inc";

// RESPONSE
pub const RESPONSE_OBJECTS: &str = "objects";
pub const RESPONSE_IDS: &str = "ids";
pub const QUERY_RESPONSE: &str = "response";
pub const TRANSACTION_RESPONSE: &str = "result";
pub const INSERT_META: &str = "insert_meta";
pub const GET_META: &str = "get_meta";
pub const FIND_META: &str = "find_meta";
pub const UPDATE_META: &str = "update_meta";
pub const DELETE_META: &str = "delete_meta";

// OTHER
pub const ROOT: &str = "root";
pub const INTERNAL_COLLECTION_NAME: &str = "_internal";

pub const FETCH_DEPTH_LIMIT: i32 = 1024;
