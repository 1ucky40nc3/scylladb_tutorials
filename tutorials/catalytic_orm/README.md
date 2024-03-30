# Using the ScyllaDB Catalytic ORM for Rust

This tutorial shows how the ScyllaDB [Catalytic](https://github.com/Jasperav/Catalytic) ORM for Rust.

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

## Sources

- [Catalytic: ScyllaDB and Cassandra Object-Relation Mapper](https://github.com/Jasperav/Catalytic)
- [Introducing Catalytic: An ORM designed for ScyllaDB and Cassandra written in Rust](https://www.scylladb.com/2022/02/15/introducing-catalytic-an-orm-designed-for-scylladb-and-cassandra-written-in-rust/)
- [Catalytic Scylla blog](https://github.com/Jasperav/scylla_mapping)