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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_operation_variants() {
        assert_ne!(QueryOperation::InsertOperation, QueryOperation::FindOperation);
        assert_ne!(QueryOperation::GetOperation, QueryOperation::DeleteOperation);
        assert_ne!(QueryOperation::UpdateOperation, QueryOperation::SortOperation);
    }
}
