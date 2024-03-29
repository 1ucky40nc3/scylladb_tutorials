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

Create a new terminal and execute the following commands to create the seed node of our cluster:

```bash
# Create a new network for our cluster
docker network create scylla-network
# Create the seed node of our cluster
docker run --name scylla-seed --network scylla-network --rm scylladb/scylla --smp 1
```

In this command we create a new default Docker container network named `scylla-network`. This add the advantage of being able to find our Docker containers in the network easily. We can just use the container's name as the URL - instead of having to provide an IP address.

We can check if the single-node cluster is created by using `docker ps` to check if the container is running and showing the status of our cluster using nodetool:

```bash
docker exec -it scylla-seed nodetool status
```

Now it is time to add more nodes. We can add ScyllaDB nodes to our cluster by creating more ScyllaDB Docker containers. The difference is that we show the set the seed node of our cluster to our new nodes. This step adds the nodes to our existing cluster instead of creating new clusters. Create a new terminal and execute the following command to add a new node to our cluster:

```bash
docker run --name scylla-node --network scylla-network --rm scylladb/scylla --smp 1 --seeds="scylla-seed"
```

Using the noodetool command again we can see that another node has joined the ScyllaDB cluster. Here is the command:

```bash
docker exec -it scylla-seed nodetool status
```

Now we have a multi-node ScyllaDB cluster!

Don't forget to delete the `scylla-network` Docker container network at the end. Here is a command to delete the network:

```bash
docker network rm scylla-network
```

### 4. Creating a Multi-Node ScyllaDB Cluster with Docker Compose

The steps illustrated in the [section above](#3-creating-a-multi-node-scylladb-cluster) can also be automated using Docker Compose. Just start the provided [docker-compose.yml](./docker-compose.yml) file with the following command:

```bash
docker compose up
```

List the running Docker Compose containers to check if the `scylla-seed` and `scylla-node` containers are running:

```bash
docker compose ps
```

Inspect the multi-node ScyllaDB cluster using nodetools:

```bash
docker exec -it scylla-seed nodetool status
```

Now we have a multi-node ScyllaDB cluster!

Don't forget to delete the Docker Compose environment at the end. Here is a command to delete the network:

```bash
docker compose rm
```

## Sources

- [Run ScyllaDB in Docker](https://www.scylladb.com/download/?platform=docker#open-source)
- [scylladb/scylla](https://hub.docker.com/r/scylladb/scylla/)
- [Nodetool](https://opensource.docs.scylladb.com/stable/operating-scylla/nodetool.html)
- [CQLSh: the CQL shell](https://opensource.docs.scylladb.com/stable/cql/cqlsh.html)