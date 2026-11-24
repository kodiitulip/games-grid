#![allow(dead_code)]

use postgres::{Client, NoTls, Row};

pub struct GamesDAO {
    pub id: i32,
    pub studio_id: i32,
    pub publisher_id: Option<i32>,
    pub name: String,
    pub description: String,
}

impl GamesDAO {
    pub fn new(
        id: i32,
        studio_id: i32,
        publisher_id: Option<i32>,
        name: String,
        description: String,
    ) -> Self {
        Self {
            id,
            studio_id,
            publisher_id,
            name,
            description,
        }
    }
}

impl From<&Row> for GamesDAO {
    fn from(value: &Row) -> Self {
        let id = value.get("id");
        let studio_id = value.get("studio_id");
        let publisher_id = value.get("publisher_id");
        let name = value.get("name");
        let description = value.get("description");
        GamesDAO::new(id, studio_id, publisher_id, name, description)
    }
}

impl From<Row> for GamesDAO {
    fn from(value: Row) -> Self {
        let id = value.get("id");
        let studio_id = value.get("studio_id");
        let publisher_id = value.get("publisher_id");
        let name = value.get("name");
        let description = value.get("description");
        GamesDAO::new(id, studio_id, publisher_id, name, description)
    }
}

pub struct GameArtDAO {
    pub game_id: i32,
    pub header: Option<String>,
    pub icon: Option<String>,
    pub capsule: Option<String>,
}

impl GameArtDAO {
    pub fn new(
        game_id: i32,
        header: Option<String>,
        icon: Option<String>,
        capsule: Option<String>,
    ) -> Self {
        Self {
            game_id,
            header,
            icon,
            capsule,
        }
    }
}

pub struct GameAchivementsDAO {
    pub id: i32,
    pub game_id: i32,
    pub icon: Option<String>,
    pub description: Option<String>,
}

impl GameAchivementsDAO {
    pub fn new(id: i32, game_id: i32, icon: Option<String>, description: Option<String>) -> Self {
        Self {
            id,
            game_id,
            icon,
            description,
        }
    }
}

pub struct DeveloperStudioDAO {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl DeveloperStudioDAO {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

impl From<Row> for DeveloperStudioDAO {
    fn from(value: Row) -> Self {
        let id = value.get("id");
        let name = value.get("name");
        let description = value.get("description");
        DeveloperStudioDAO::new(id, name, description)
    }
}

pub struct PublisherDAO {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl PublisherDAO {
    pub fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
        }
    }
}

impl From<Row> for PublisherDAO {
    fn from(value: Row) -> Self {
        let id = value.get("id");
        let name = value.get("name");
        let description = value.get("description");
        PublisherDAO::new(id, name, description)
    }
}

pub fn list_all_games() -> Result<Vec<GamesDAO>, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result: Vec<GamesDAO> = client
        .query(
            "select id,studio_id,publisher_id,name,description from games",
            &[],
        )?
        .iter()
        .map(|row| row.into())
        .collect();
    // drop(client);
    Ok(result)
}

pub fn list_game_by_id(id: i32) -> Result<GamesDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,studio_id,publisher_id,name,description from games where games.id == $1",
        &[&id],
    )?;
    // drop(client);
    Ok(result.into())
}

pub fn list_game_by_name(name: String) -> Result<GamesDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,studio_id,publisher_id,name,description from games where name ilike $1",
        &[&name],
    )?;
    // drop(client);
    Ok(result.into())
}

pub fn list_publisher_by_id(id: i32) -> Result<PublisherDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,name,description from publisher where id == $1",
        &[&id],
    )?;
    // drop(client);
    Ok(result.into())
}

pub fn list_publisher_by_name(name: String) -> Result<PublisherDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,name,description from publisher where name ilike $1",
        &[&name],
    )?;
    // drop(client);
    Ok(result.into())
}

pub fn list_studio_by_id(id: i32) -> Result<DeveloperStudioDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,name,description from developer_studio where id == $1",
        &[&id],
    )?;
    // drop(client);
    Ok(result.into())
}

pub fn list_studio_by_name(name: String) -> Result<DeveloperStudioDAO, postgres::Error> {
    let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

    let result = client.query_one(
        "select id,name,description from developer_studio where name ilike $1",
        &[&name],
    )?;
    // drop(client);
    Ok(result.into())
}
