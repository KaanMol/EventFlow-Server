use rusqlite::Connection;

struct User {
    id: String,
    name: String,
}

pub struct Ical {
    pub id: Option<i32>,
    pub user_id: String,
    pub name: String,
    pub url: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn connect() -> Self {
        let connection = Self {
            conn: Connection::open("calendar.db").unwrap(),
        };

        connection.setup().unwrap();
        connection
    }

    fn setup(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "create table if not exists user (
                    id UUID primary key,
                    name text not null
             )",
            [],
        )?;

        self.conn.execute(
            "create table if not exists ical (
                    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
                    name text not null,
                    url text not null,
                    user_id UUID not null
             )",
            [],
        )?;

        self.conn.execute(
            "create table if not exists calendar (
                    id UUID primary key,
                    name text not null,
                    user_id UUID not null
             )",
            [],
        )?;

        self.conn.execute(
            "create table if not exists filter (
                    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
                    field text not null,
                    filter text not null,
                    value text not null,
                    calendar_id int not null
             )",
            [],
        )?;

        self.conn.execute(
            "create table if not exists operation (
                    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
                    field       text        not null,
                    operation   text        not null,
                    value       text        not null,
                    new_value   text        not null,
                    calendar_id INTEGER     not null
             )",
            [],
        )?;

        Ok(())
    }

    pub fn create_user(&self, name: String) -> Result<String, rusqlite::Error> {
        let uuid = uuid::Uuid::new_v4().to_string();

        println!("uuid: {:?}. name: {:?}", uuid, name);

        let mut statement = self
            .conn
            .prepare("insert into user (id, name) values (?1, ?2)")?;
        statement.execute([uuid.clone(), name])?;

        Ok(uuid)
    }

    pub fn add_ical(&self, ical: Ical) -> Result<(), rusqlite::Error> {
        let mut statement = self
            .conn
            .prepare("insert into ical (name, url, user_id) values (?1, ?2, ?3)")?;
        statement.execute([ical.name, ical.url, ical.user_id])?;

        Ok(())
    }

    pub fn get_ical_urls(&self, user_id: String) -> Result<Vec<Ical>, rusqlite::Error> {
        let mut statement = self
            .conn
            .prepare("SELECT id, name, url, user_id FROM ical WHERE user_id = ?1")?;

        let ical_urls = statement.query_map([user_id], |row| {
            Ok(Ical {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                user_id: row.get(3)?,
            })
        })?;

        let mut ical_objects: Vec<Ical> = Vec::new();
        for ical_url in ical_urls {
            ical_objects.push(ical_url.unwrap());
        }

        Ok(ical_objects)
        // Ok(ical_urls.map(|url| url.unwrap()).collect::<Vec<String>>())
    }
}
