//      $> clear; rm -rf /tmp/kuzu_db/; cargo run

use kuzu::{Database, SystemConfig, Connection};

const DEFAULT_DB: &str = "/tmp/kuzu_db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(DEFAULT_DB, SystemConfig::default())?;
    let conn = Connection::new(&db)?;

    let _ = conn.query("
        CREATE NODE TABLE User (name STRING, age INT64, PRIMARY KEY(name));
        CREATE NODE TABLE City (name STRING, population INT64, PRIMARY KEY(name));
        CREATE REL TABLE Follows (FROM User TO User, since DATE);
        CREATE REL TABLE LivesIn (FROM User TO City, since DATE);
        CREATE REL TABLE FriendshipCity (FROM City TO City, since DATE);
    ")?;

    let result = conn.query("
        CREATE (u1: User {name: 'Carly', age: 31}), 
            (u2: User {name: 'Keinichi', age: 47}),
            (u3: User {name: 'Akila', age: 25}),
            (u4: User {name: 'Klaus', age: 38})
        CREATE (u1)-[f1: Follows {since: DATE('2025-03-25')}]->(u2)
        CREATE (u1)-[f2: Follows {since: DATE('2025-04-08')}]->(u3)
        CREATE (u2)-[f3: Follows {since: DATE('2024-01-25')}]->(u3)
        CREATE (u4)-[f4: Follows {since: DATE('2024-01-25')}]->(u1)
        CREATE (u2)-[f5: Follows {since: DATE('2024-01-25')}]->(u4)
        CREATE (c1: City {name: 'Dallas', population: 1302638})
        CREATE (c2: City {name: 'Sendai', population: 2341000})
        CREATE (c3: City {name: 'Cairo', population: 4493410})
        CREATE (c4: City {name: 'Stuttgart', population: 632865})
        CREATE (c1)-[fc01: FriendshipCity {since: DATE('1997-08-01')}]->(c2)
        CREATE (c3)-[fc02: FriendshipCity {since: DATE('1997-08-01')}]->(c4)
        CREATE (u1)-[l01: LivesIn]->(c1)
        CREATE (u2)-[l02: LivesIn]->(c2)
        CREATE (u3)-[l03: LivesIn]->(c3)
        CREATE (u4)-[l04: LivesIn]->(c4)
        RETURN *;    
    ")?;

    // kuzu::result is a vector; kuzu::value is a enum wrapper for Rust types.
    for item in result {
        for value in item {
            println!("{}", value);
        }
    }

    Ok(())
}
