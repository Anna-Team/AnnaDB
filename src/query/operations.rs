#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum QueryOperation {
    InsertOperation,
    GetOperation,
    FindOperation,
    UpdateOperation,
    DeleteOperation,
    SortOperation,
    LimitOperation,
    OffsetOperation,
    ProjectOperation,
    IndexOperation,
}
