use rusqlite::{params, Connection, Result};

// Struct to manage database operations
pub struct Database {
    conn: Connection,
}

impl Database {
    // Initialize the database
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS repositories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                owner TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE authorized_keys (
                username TEXT PRIMARY KEY,
                public_key TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                repo_id INTEGER NOT NULL,
                message TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                FOREIGN KEY(repo_id) REFERENCES repositories(id),
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    // Add a new repository
    pub fn create_repo(&self, name: &str, owner: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO repositories (name, owner) VALUES (?1, ?2)",
            params![name, owner],
        )?;
        Ok(())
    }

    // Add a commit to a repository
    pub fn add_commit(&self, repo_name: &str, message: &str) -> Result<()> {
        let repo_id: i64 = self.conn.query_row(
            "SELECT id FROM repositories WHERE name = ?1",
            params![repo_name],
            |row| row.get(0),
        )?;

        self.conn.execute(
            "INSERT INTO commits (repo_id, message, timestamp) VALUES (?1, ?2, strftime('%s', 'now'))",
            params![repo_id, message],
        )?;

        Ok(())
    }

    // Retrieve commits for a repository
    pub fn get_commits(&self, repo_name: &str) -> Result<Vec<(String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT c.message, c.timestamp 
             FROM commits c
             JOIN repositories r ON c.repo_id = r.id
             WHERE r.name = ?1",
        )?;

        let commits = stmt
            .query_map(params![repo_name], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(commits)
    }
}
