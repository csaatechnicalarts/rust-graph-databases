use kuzu::{Database, SystemConfig, Connection};

const DEFAULT_DB: &str = "/tmp/kuzu_db";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new(DEFAULT_DB, SystemConfig::default())?;
    let conn = Connection::new(&db)?;

    let q_statement = "MATCH (u: User {name: 'Carly'}) 
        RETURN u";
    let mut result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    let q_statement =  "MATCH (u: User) 
        WHERE u.name = 'Keinichi' 
        RETURN u";

    result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    let q_statement = "MATCH (u: User) 
        WHERE u.name = 'Keinichi' OR u.name = 'Carly' 
        RETURN u.name AS PERSON";
    result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    let q_statement = "MATCH (u1: User) 
        WHERE u1.name = 'Carly'
        MATCH (u2: User) WHERE u2.name = 'Keinichi'
        RETURN *";
    let result = conn.query(q_statement)?;
    println!("{}", result.to_string());

    Ok(())
}
