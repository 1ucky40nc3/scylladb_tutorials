# Getting Started with ScyllaDB

This tutorial will show you how to get started with ScyllaDB. We will use Docker in an effort to be independent of your current platform. Our sources and other resources are linked down below.

## Steps

### 1. Running a Single Instance

In this tutorial we will use the Open Source version of ScyllaDB. Luckily there is a [ScyllaDB](https://hub.docker.com/r/scylladb/scylla/) Docker container image on Docker Hub. ScyllaDB works by building Scylla clusters out of nodes. Using the provided Docker image we can run a single-node Scylla cluster. Here is the command:

```bash
docker run --name single-node-scylla --rm scylladb/scylla --smp 1
```

The command pulls the Open Source [scylladb/scylla](https://hub.docker.com/r/scylladb/scylla/) ScyllaDB image and tries to run a single Docker container. The container is named `single-node-scylla`. In this example the single-node cluster is limited to one core (by `--smp`) and started in development mode by default (see [here](https://hub.docker.com/r/scylladb/scylla/) for more details on the configuration).

### 2. Using ScyllaDB Tools

After starting the single-node ScyllaDB cluster we can use the [nodetool](https://opensource.docs.scylladb.com/stable/operating-scylla/nodetool.html) CLI utility to inspect the cluster. The following command shows the current status of your cluster.

```bash
docker exec -it single-node-scylla nodetool status
```

Next we can try to interact with our ScyllaDB using the [CQLSh](https://opensource.docs.scylladb.com/stable/cql/cqlsh.html) command line shell. Here is a command to open the shell:

```bash
docker exec -it single-node-scylla cqlsh
```

### 3. Creating a Multi-Node ScyllaDB Cluster

After successfully creating a single-node ScyllaDB cluster it is now time to add more nodes. We will now create a multi-node cluster.

Create a new terminal and execute the following command to create the seed node of our cluster:

```bash
docker run --name scylla-seed --hostname scylla-seed --rm scylladb/scylla --smp 1
```

In this command we add we set the hostname `single-node-scylla` in the Docker container network. Using the hostname we can later find the Docker container in the container network - this way we don't need to use it's IP address.

We can check if the single-node cluster is created by using `docker ps` to check if the container is running and showing the status of our cluster using nodetool:

```bash
docker exec -it scylla-seed nodetool status
```

Now it is time to add more nodes. We can add ScyllaDB nodes to our cluster by creating more ScyllaDB Docker containers. The difference is that we show the set the seed node of our cluster to our new nodes. This step adds the nodes to our existing cluster instead of creating new clusters. Create a new terminal and execute the following command to add a new node to our cluster:

```bash
docker run --name scylla-node --hostname scylla-node --rm scylladb/scylla --smp 1 --seeds="$(docker inspect --format='{{ .NetworkSettings.IPAddress }}' scylla-seed)"
```


## Sources

- [Run ScyllaDB in Docker](https://www.scylladb.com/download/?platform=docker#open-source)
- [scylladb/scylla](https://hub.docker.com/r/scylladb/scylla/)
- [Nodetool](https://opensource.docs.scylladb.com/stable/operating-scylla/nodetool.html)
- [CQLSh: the CQL shell](https://opensource.docs.scylladb.com/stable/cql/cqlsh.html)