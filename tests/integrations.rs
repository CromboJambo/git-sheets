use gitsheets::{
    core::{Snapshot, Table},
    diff::SnapshotDiff,
};

#[test]
fn test_snapshot_creation() {
    // Create a simple table with headers and rows
    let table = Table {
        headers: vec!["ID".to_string(), "Name".to_string(), "Amount".to_string()],
        rows: vec![
            vec!["1".to_string(), "Alice".to_string(), "100".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "200".to_string()],
        ],
        primary_key: None,
    };

    // Create a snapshot
    let snapshot = Snapshot::new(table, Some("Test snapshot".to_string()));

    // Should have a valid ID (timestamp-hash format)
    assert!(snapshot.id.len() > 10);
    assert_eq!(snapshot.message, Some("Test snapshot".to_string()));
    assert_eq!(snapshot.table.headers, vec!["ID", "Name", "Amount"]);
    assert_eq!(snapshot.table.rows.len(), 2);
}

#[test]
fn test_empty_table_handling() {
    // Create a table with headers but no rows - this should be allowed now
    let table = Table {
        headers: vec!["ID".to_string(), "Name".to_string()],
        rows: vec![],
        primary_key: None,
    };

    // Create snapshot - should not error
    let snapshot = Snapshot::new(table, Some("Empty table snapshot".to_string()));

    // Should have a valid ID (timestamp-hash format)
    assert!(snapshot.id.len() > 10);
    assert_eq!(snapshot.table.rows.len(), 0);
}

#[test]
fn test_diff_computation() {
    // Test that diff computation works without crashing
    let table1 = Table {
        headers: vec!["ID".to_string(), "Name".to_string(), "Amount".to_string()],
        rows: vec![vec![
            "1".to_string(),
            "Alice".to_string(),
            "100".to_string(),
        ]],
        primary_key: None,
    };

    let table2 = Table {
        headers: vec!["ID".to_string(), "Name".to_string(), "Amount".to_string()],
        rows: vec![vec!["2".to_string(), "Bob".to_string(), "200".to_string()]],
        primary_key: None,
    };

    let snapshot1 = Snapshot::new(table1, Some("Version 1".to_string()));
    let snapshot2 = Snapshot::new(table2, Some("Version 2".to_string()));

    // Compute diff - should not panic
    let diff = SnapshotDiff::compute(&snapshot1, &snapshot2).unwrap();

    // Should be able to compute a diff without error
    assert_eq!(diff.from_id, snapshot1.id);
    assert_eq!(diff.to_id, snapshot2.id);
}
