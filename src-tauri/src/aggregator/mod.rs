use std::collections::HashMap;

use rusqlite::{params, Connection};

use crate::models::errors::FastDiskResult;

#[derive(Debug)]
struct EntryAggregate {
    id: i64,
    parent_id: Option<i64>,
    size: i64,
    is_directory: bool,
    child_count: i64,
    descendant_count: i64,
}

pub fn aggregate_folder_sizes(connection: &Connection, scan_session_id: i64) -> FastDiskResult<i64> {
    let mut statement = connection.prepare(
        "SELECT id, parent_id, size, is_directory
         FROM file_entries
         WHERE scan_session_id = ?1
         ORDER BY depth DESC",
    )?;
    let rows = statement.query_map(params![scan_session_id], |row| {
        Ok(EntryAggregate {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            size: row.get(2)?,
            is_directory: row.get::<_, i64>(3)? == 1,
            child_count: 0,
            descendant_count: 0,
        })
    })?;

    let mut totals = HashMap::<i64, EntryAggregate>::new();
    let mut root_total = 0;
    for row in rows {
        let entry = row?;
        let size = if entry.is_directory {
            totals.get(&entry.id).map(|item| item.size).unwrap_or(0)
        } else {
            entry.size
        };
        let entry_id = entry.id;
        let parent_id = entry.parent_id;
        let is_directory = entry.is_directory;

        totals
            .entry(entry_id)
            .and_modify(|item| {
                item.parent_id = parent_id;
                item.is_directory = is_directory;
                if !is_directory {
                    item.size = size;
                }
            })
            .or_insert(EntryAggregate {
                size,
                ..entry
            });

        if let Some(parent_id) = totals.get(&entry_id).and_then(|item| item.parent_id) {
            let descendant_count = totals
                .get(&entry_id)
                .map(|item| item.descendant_count + 1)
                .unwrap_or(1);
            let parent = totals.entry(parent_id).or_insert(EntryAggregate {
                id: parent_id,
                parent_id: None,
                size: 0,
                is_directory: true,
                child_count: 0,
                descendant_count: 0,
            });
            parent.size += size;
            parent.child_count += 1;
            parent.descendant_count += descendant_count;
        } else {
            root_total += size;
        }
    }

    for aggregate in totals.values() {
        if aggregate.is_directory {
            connection.execute(
                "UPDATE file_entries
                 SET size = ?1, child_count = ?2, descendant_count = ?3
                 WHERE id = ?4",
                params![
                    aggregate.size,
                    aggregate.child_count,
                    aggregate.descendant_count,
                    aggregate.id
                ],
            )?;
        }
    }

    Ok(root_total)
}
