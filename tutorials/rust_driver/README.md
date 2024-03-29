# Using the Rust Driver with ScyllaDB

This tutorial shows you how to use the [Scylla Rust Driver](https://github.com/scylladb/scylla-rust-driver) to interact with a ScyllaDB cluster. For the purposes of this tutorial we will develop a [Rust](https://www.rust-lang.org/) application and use a sing-node ScyllaDB cluster in a Docker container.

## Steps

### 1. Create a Single-Node ScyllaDB Cluster

In this tutorial we will use a single-node ScyllaDB cluster. This cluster is set up in a Docker Compose environment. You can start the cluster with the following command:

```bash
docker compose up
```

We start a service named `scylla` that implements the cluster. The cluster is started in development mode and restricted to use only one core.

You can inspect the ScyllaDB cluster using the `nodetool` CLI tool:

```bash
docker compose exec -it scylla nodetool status
```

### 2. Create the ScyllaDB Data Schema

After creating the ScyllaDB cluster we can create data schema for later use. In this tutorial we will a database for tutorials - quite creative 😅.

First you need to connect to your ScyllaDB cluster and start a [CQLSh](https://opensource.docs.scylladb.com/stable/cql/cqlsh.html) command line shell. Here is a command to do so:

```bash
docker compose exec -it scylla cqlsh
```

Next you need to create a keyspace. We create a keyspace called `tutorial`. Execute the following query in the CQLsh shell:

```
CREATE KEYSPACE IF NOT EXISTS tutorial
    WITH REPLICATION = {
        'class': 'SimpleStrategy',
        'replication_factor': 1
};
```

You can check if your keyspace was created with the following command:

```
DESC keyspaces;
```

After creating the keyspace we can create a table. In our database we want to save tutorials. Each tutorial has a unique ID, a name and timestamp for when it was last updated. For this purpose we create the `tutorial` table with the following query:

```
CREATE TABLE IF NOT EXISTS tutorial.tutorial (
    id UUID,
    name TEXT,
    time TIMESTAMP,
    PRIMARY KEY(id)
);
```

Optionally check if your table was created with this query:

```
DESC tables;
```

An example query to get tutorials from our database could be the following:

```
SELECT * FROM tutorial.tutorial
WHERE id = {id}
```

### 3. Connect to the ScyllaDB Cluster using the Rust Driver

We provide configuration for a VSCode Dev Container for development purposes. You can start the Dev Container using `Dev Containers: Reopen in Container` VSCode command. You can access the command using the command palette (with the keyboard shortcut `Ctr + Shift + P`).

Make sure that you install the [recommended extensions](.vscode/extensions.json) in the Dev Container:

- `rust-lang.rust-analyzer`
- `vadimcn.vscode-lldb`
- `streetsidesoftware.code-spell-checker`
- ...

Initialize the Rust project using the [cargo](https://github.com/rust-lang/cargo) package manager. You can use this command:

```bash
cargo init
```

## Sources

- [ScyllaDB University Lesson: Using the Rust Driver to Interact with a ScyllaDB Cluster](https://www.scylladb.com/2022/01/20/scylla-university-new-rust-lesson/)
- [scylladb/scylla](https://hub.docker.com/r/scylladb/scylla/)
- [Nodetool](https://opensource.docs.scylladb.com/stable/operating-scylla/nodetool.html)
- [CQL Reference - Data Types](https://opensource.docs.scylladb.com/stable/cql/types.html#data-types)
- [Cargo](https://github.com/rust-lang/cargo)
