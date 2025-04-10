+++
title = "Getting Started with Kuzu (Command Line)"
date = "2025-03-26"
draft = false

[taxonomies]
tags=["tutorial", "kuzudb"]

[extra]
comment = false
+++

## General Workflow

The quickest way to get started with KuzuDB is to use a Docker image. With the KuzuDB server running, you can then interact with it by means of the Kuzu Browser or your application code.

{% mermaid() %}
graph LR
    A[Start] --> B[Grab Docker image]
    B --> C[Run Kuzu with Kuzu Browser]
    C --> D[Interact with Kuzu Browser]
    C --> E[Run Kuzu app]
    
    style A fill:#f9f,stroke:#333
{% end %}

## Pull Docker Image for KuzuDB

```$> docker pull kuzudb/explorer```

## Start Docker with the Kuzu Server

```docker run -p 8000:8000 -v ./social_network_rs_db:/database  --rm kuzudb/explorer:latest```

If your running the Kuzu server from a remote network host, adjust the host firewall to open a port for Kuzu, 8000 in this case.

The social network database is stored in a directory local to the server host. The database schema and data is available in this [Kuzu tutorial](https://docs.kuzudb.com/tutorials/example-database/). The assumption here is that you've created the graph database in the host server filesystem beforehand. 
