+++
title = "Cypher CREATE and RETURN"
date = "2025-03-26"
draft = false

[taxonomies]
tags=["Cypher", "KuzuDB"]

[extra]
comment = false
+++

## CREATE

The ```CREATE``` clause is used to tables that store nodes and relationships in a graph database.
It does not check for existing data in the datastore; instead, it blindly folows the command
to create a new pattern in the graph network. 
```cypher
CREATE NODE TABLE User (name STRING, age INT64, PRIMARY KEY(name));
CREATE NODE TABLE City (name STRING, population INT64, PRIMARY KEY(name));
CREATE REL TABLE Follows (FROM User TO User, since DATE);
```
In the example below, we also use ```CREATE``` to insert graph nodes into our tables.
Within a node, we use curly brackets to enclose node properties.

## RETURN

The ```RETURN``` clause retrieves graph elements from the database. There can only be
one ```RETURN``` clause in a Cypher query, except for statements involving ```UNION```
and subqueries.

In the following example, the query returns all the nodes and the relationships we've added to the graph tables.

```cypher, linenos
CREATE (u1: User {name: 'Carly', age: 31}), (u2: User {name: 'Keinichi', age: 47})
CREATE (u1)-[f: Follows {since: DATE('2025-03-25')}]->(u2)
CREATE (c1: City {name: 'Dallas', population: 1302638})
CREATE (c2: City {name: 'Sendai', population: 2341000})
CREATE (c1)-[fc01: FriendshipCity {since: DATE('1997-08-01')}]->(c2)
CREATE (u1)-[l01: LivesIn]->(c1)
CREATE (u2)-[l02: LivesIn]->(c2)
RETURN *;    
```

Looking at the command line output for the statement above, Kuzu returns the data tagged with tuples indicating 
node table IDs and rows.

```KuzuDB, linenos
(label:User, 0:0, {name:Carly,age:31})
(label:User, 0:1, {name:Keinichi,age:47})
(0:0)-[label:Follows, {since:2025-03-25}]->(0:1)
(label:City, 1:0, {name:Dallas,population:1302638})
(label:City, 1:1, {name:Sendai,population:2341000})
(1:0)-[label:FriendshipCity, {since:1997-08-01}]->(1:1)
(0:0)-[label:LivesIn, {since:}]->(1:0)
(0:1)-[label:LivesIn, {since:}]->(1:1)
```

For example with the ```User``` table tagged as ```0```, and the first and second
rows tagged as ```0``` and ```1``` respectively, the tuple ```0:1``` maps to 
the second record in the User table, the entry for the node ```{name:Keinichi,age:47}```
(line 2).

These tuples are used in the same way for relationships.
```(1:0)-[label:FriendshipCity, {since:1997-08-01}]->(1:1)``` matches a ```FriendshipCity```
relationship between Dallas and Sendai, nodes ```1:0``` and ```1:1``` respectively.

## REFERENCE VARIABLES and LABELS

In Cypher queries, reference variables are placeholders for data that we declare before 
node types. For example, ```u1``` and ```c2``` in ```u1: User``` and ```c2: City```.

Because a ```CREATE``` statement doesn't perform any database lookup before inserting
new data, reference variables are only visible within the same Cypher query. In short,
reference variables are not visible between queries.

