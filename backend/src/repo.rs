use crate::db::DbPool;
use crate::models::{CreateTodo, Todo, UpdateTodo};
use rusqlite::params;

pub fn list_all(pool: &DbPool) -> Result<Vec<Todo>, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    let mut stmt = conn.prepare(
        "SELECT id, title, completed, display_order FROM todos ORDER BY display_order, id",
    )?;
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
                display_order: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(todos)
}

pub fn create(pool: &DbPool, input: &CreateTodo) -> Result<Todo, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    let id = uuid::Uuid::new_v4().to_string();
    let order = input.order.unwrap_or(0);
    conn.execute(
        "INSERT INTO todos (id, title, completed, display_order) VALUES (?1, ?2, ?3, ?4)",
        params![id, input.title, false, order],
    )?;
    Ok(Todo {
        id,
        title: input.title.clone(),
        completed: false,
        display_order: order,
    })
}

pub fn get_by_id(pool: &DbPool, id: &str) -> Result<Option<Todo>, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    let mut stmt =
        conn.prepare("SELECT id, title, completed, display_order FROM todos WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            display_order: row.get(3)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn update(
    pool: &DbPool,
    id: &str,
    input: &UpdateTodo,
) -> Result<Option<Todo>, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");

    // Build dynamic UPDATE
    let mut sets = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref title) = input.title {
        sets.push("title = ?");
        values.push(Box::new(title.clone()));
    }
    if let Some(completed) = input.completed {
        sets.push("completed = ?");
        values.push(Box::new(completed));
    }
    if let Some(order) = input.order {
        sets.push("display_order = ?");
        values.push(Box::new(order));
    }

    if sets.is_empty() {
        // Nothing to update, just return current
        drop(conn);
        return get_by_id(pool, id);
    }

    values.push(Box::new(id.to_string()));
    let sql = format!("UPDATE todos SET {} WHERE id = ?", sets.join(", "));
    let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let rows_affected = conn.execute(&sql, params.as_slice())?;

    if rows_affected == 0 {
        return Ok(None);
    }

    // Return updated row
    let mut stmt =
        conn.prepare("SELECT id, title, completed, display_order FROM todos WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            display_order: row.get(3)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn delete(pool: &DbPool, id: &str) -> Result<bool, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    let rows_affected = conn.execute("DELETE FROM todos WHERE id = ?1", params![id])?;
    Ok(rows_affected > 0)
}

pub fn delete_completed(pool: &DbPool) -> Result<usize, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    let rows_affected = conn.execute("DELETE FROM todos WHERE completed = 1", [])?;
    Ok(rows_affected)
}

pub fn toggle_all(pool: &DbPool, completed: bool) -> Result<Vec<Todo>, rusqlite::Error> {
    let conn = pool.lock().expect("lock db");
    conn.execute("UPDATE todos SET completed = ?1", params![completed])?;
    drop(conn);
    list_all(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn setup_pool() -> DbPool {
        std::env::set_var("DATABASE_URL", ":memory:");
        db::init_db().expect("init test db")
    }

    #[test]
    fn test_create_and_list() {
        let pool = setup_pool();
        let todo = create(
            &pool,
            &CreateTodo {
                title: "Test".into(),
                order: None,
            },
        )
        .expect("create");
        assert_eq!(todo.title, "Test");
        assert!(!todo.completed);

        let all = list_all(&pool).expect("list");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, todo.id);
    }

    #[test]
    fn test_get_by_id() {
        let pool = setup_pool();
        let todo = create(
            &pool,
            &CreateTodo {
                title: "Find me".into(),
                order: None,
            },
        )
        .expect("create");

        let found = get_by_id(&pool, &todo.id)
            .expect("get")
            .expect("should exist");
        assert_eq!(found.title, "Find me");

        let missing = get_by_id(&pool, "nonexistent").expect("get");
        assert!(missing.is_none());
    }

    #[test]
    fn test_update() {
        let pool = setup_pool();
        let todo = create(
            &pool,
            &CreateTodo {
                title: "Original".into(),
                order: None,
            },
        )
        .expect("create");

        let updated = update(
            &pool,
            &todo.id,
            &UpdateTodo {
                title: Some("Updated".into()),
                completed: Some(true),
                order: None,
            },
        )
        .expect("update")
        .expect("should exist");

        assert_eq!(updated.title, "Updated");
        assert!(updated.completed);
    }

    #[test]
    fn test_delete() {
        let pool = setup_pool();
        let todo = create(
            &pool,
            &CreateTodo {
                title: "Delete me".into(),
                order: None,
            },
        )
        .expect("create");
        assert!(delete(&pool, &todo.id).expect("delete"));
        assert!(!delete(&pool, &todo.id).expect("delete again"));
        assert!(list_all(&pool).expect("list").is_empty());
    }

    #[test]
    fn test_delete_completed() {
        let pool = setup_pool();
        create(
            &pool,
            &CreateTodo {
                title: "Keep".into(),
                order: None,
            },
        )
        .expect("create");
        let done = create(
            &pool,
            &CreateTodo {
                title: "Done".into(),
                order: None,
            },
        )
        .expect("create");
        update(
            &pool,
            &done.id,
            &UpdateTodo {
                completed: Some(true),
                ..Default::default()
            },
        )
        .expect("update");

        let deleted = delete_completed(&pool).expect("delete completed");
        assert_eq!(deleted, 1);
        let remaining = list_all(&pool).expect("list");
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].title, "Keep");
    }

    #[test]
    fn test_toggle_all() {
        let pool = setup_pool();
        create(
            &pool,
            &CreateTodo {
                title: "A".into(),
                order: None,
            },
        )
        .expect("create");
        create(
            &pool,
            &CreateTodo {
                title: "B".into(),
                order: None,
            },
        )
        .expect("create");

        let toggled = toggle_all(&pool, true).expect("toggle all");
        assert!(toggled.iter().all(|t| t.completed));

        let toggled = toggle_all(&pool, false).expect("toggle all");
        assert!(toggled.iter().all(|t| !t.completed));
    }
}
