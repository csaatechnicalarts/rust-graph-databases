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

As we go about specifying the graph database schema, we use the Cypher ```CREATE``` clause to provision tables that store graph nodes and 
the relationships between them. Here's an example where we create the ```User``` and ```City``` node tables. Immedately afterwards we create 
the ```Follows``` ```LivesIn``` and ```FriendshipCity``` tables to store relationships between users and cities.

```cypher
CREATE NODE TABLE User (name STRING, age INT64, PRIMARY KEY(name));
CREATE NODE TABLE City (name STRING, population INT64, PRIMARY KEY(name));
CREATE REL TABLE Follows (FROM User TO User, since DATE);
CREATE REL TABLE LivesIn (FROM User TO City, since DATE);
CREATE REL TABLE FriendshipCity (FROM City TO City, since DATE);
```
In the next section we'll see how  to use ```CREATE``` to insert nodes and relationships into the tables we've provisioned.  
## RETURN

The ```RETURN``` clause retrieves data from the graph database. There can only be one ```RETURN``` clause in a Cypher query, except for 
statements involving ```UNION``` and subqueries. We rely on curly brackets to denote node and relationship properties.


In the following example, our queries insert data into the ```User```, 
```City```, ```FriendshipCity``` and ```LivesIn``` tables. At the end of the statement, we use the ```RETURN``` to inspect everything we've loaded into the graph database.

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

The section below shows  query output as it appears on the command line. 

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
Note how Kuzu uses tuples to tag node, with tuple values representing table IDs and rows.
For example with the ```User``` table tagged as ```0```, and the first and second
rows tagged as ```0``` and ```1``` respectively, the tuple ```0:1``` maps to 
the second record in the User table, the entry for the node ```{name:Keinichi,age:47}```
(line 2).

These tuples are used in the same way for relationship data.
```(1:0)-[label:FriendshipCity, {since:1997-08-01}]->(1:1)``` matches a ```FriendshipCity```
relationship between Dallas and Sendai, nodes ```1:0``` and ```1:1``` respectively.

## REFERENCE VARIABLES and LABELS

You may have noticed that we use reference variables are placeholders for Cypher nodes. For example, ```u1``` and ```c2``` in ```u1: User``` and ```c2: City```. While we've used reference variable superficially in our examples here, you'll see more when we write our queries with the ```MATCH``` clause.

Note that because a ```CREATE``` statement doesn't perform any database lookup before inserting
new data, reference variables are only visible within the same Cypher query. In short,
reference variables are not visible between queries.

