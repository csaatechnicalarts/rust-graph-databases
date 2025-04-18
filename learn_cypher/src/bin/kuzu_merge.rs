use kuzu::{Database, SystemConfig, Connection};

const DEFAULT_DB: &str = "/tmp/kuzu_db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(DEFAULT_DB, SystemConfig::default())?;
    let conn = Connection::new(&db)?;
    
    let q_statement = "MATCH (u1: User {name: 'Klaus'})
        MATCH (u2: User {name: 'Akila'}) 
        MERGE (u1)-[f1 :Follows {since: DATE('2024-01-25')}]->(u2) 
        MERGE (u2)-[f2 :Follows {since: DATE('2024-01-25')}]->(u1) 
        RETURN u1, u2, f1, f2";

    let mut result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    let q_statement = "MATCH (c1: City {name: 'Dallas'}) 
        MATCH (u1: User {name: 'Carly'}) 
        MERGE (u2: User {name: 'Tom', age: 22}) 
        MERGE (u2)-[l1: LivesIn]->(c1) 
        MERGE (u2)-[f1: Follows]->(u1) 
        MERGE (u1)-[f2: Follows]->(u2) 
        RETURN u1, u2, c1, l1, f1, f2";

    result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    Ok(())
}
