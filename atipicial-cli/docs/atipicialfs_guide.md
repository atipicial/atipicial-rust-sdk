<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Working with AtipicialFs Decentralized Storage

This guide explains how to use the Atipicial CLI to interact with AtipicialFs, Atipicial's decentralized storage system.

## Overview

AtipicialFs is a distributed, decentralized object storage network built on the Atipicial blockchain. The `atipicial-cli` tool provides direct access to AtipicialFs through the `fs` command group, allowing you to:

1. Manage containers (storage buckets)
2. Store, retrieve, and manage objects (files)
3. Control access permissions
4. Query information about the AtipicialFs network

## Connection Management

### Checking Status

To check the status of your AtipicialFs connection:

```bash
atipicial-cli fs status
```

This displays information about the connected AtipicialFs network, including version, available storage, and online nodes.

### Managing Endpoints

To list all available AtipicialFs endpoints:

```bash
# List mainnet endpoints
atipicial-cli fs endpoints list --network mainnet

# List testnet endpoints
atipicial-cli fs endpoints list --network testnet
```

To test a connection to a specific endpoint:

```bash
# Test a gRPC endpoint
atipicial-cli fs endpoints test --endpoint grpc.mainnet.fs.atipicial.com:8082 --type grpc

# Test an HTTP gateway
atipicial-cli fs endpoints test --endpoint https://http.mainnet.fs.atipicial.com --type http
```

To get detailed information about a specific endpoint:

```bash
atipicial-cli fs endpoints info --endpoint grpc.mainnet.fs.atipicial.com:8082
```

## Container Management

Containers are the main storage units in AtipicialFs, similar to buckets in other object storage systems.

### Creating a Container

To create a new container:

```bash
atipicial-cli fs container create --config container-config.json
```

Example configuration file (`container-config.json`):
```json
{
  "name": "my-container",
  "basic_acl": 0644,
  "placement_policy": "REP 3",
  "attributes": [
    {"key": "CreatedBy", "value": "AtipicialRust CLI"},
    {"key": "Description", "value": "Test container for documents"}
  ]
}
```

### Listing Containers

To list all containers owned by your account:

```bash
atipicial-cli fs container list
```

### Getting Container Info

To get detailed information about a container:

```bash
atipicial-cli fs container get --id CID
```

Replace `CID` with the actual container ID.

### Deleting a Container

To delete a container (it must be empty):

```bash
atipicial-cli fs container delete --id CID
```

## Object Management

Objects are the files stored in AtipicialFs containers.

### Uploading an Object

To upload a file to AtipicialFs:

```bash
atipicial-cli fs object put --container CID --file path/to/file
```

You can also specify content type and custom attributes:

```bash
atipicial-cli fs object put --container CID --file document.pdf --content-type application/pdf --attributes "Author=JohnDoe" "Department=Research"
```

### Downloading an Object

To download a file from AtipicialFs:

```bash
atipicial-cli fs object get --container CID --id OID --output path/to/save
```

### Getting Object Information

To get metadata about an object:

```bash
atipicial-cli fs object info --container CID --id OID
```

### Listing Objects in a Container

To list all objects in a container:

```bash
atipicial-cli fs object list --container CID
```

### Deleting an Object

To delete an object:

```bash
atipicial-cli fs object delete --container CID --id OID
```

## Access Control

AtipicialFs provides flexible access control mechanisms for containers and objects.

### Setting ACL Rules

To set access control rules for a container:

```bash
atipicial-cli fs acl set --container CID --basic 0644
```

### Creating Extended ACL Rules

For more complex access control, you can create extended ACL tables:

```bash
atipicial-cli fs acl extend --container CID --config extended-acl.json
```

Example `extended-acl.json`:
```json
{
  "records": [
    {
      "operation": "PUT",
      "action": "ALLOW",
      "filters": [
        {"key": "address", "value": "NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz"}
      ]
    }
  ]
}
```

## Practical Examples

### Creating a Private Photo Storage

```bash
# Create a container for photos
atipicial-cli fs container create --config photo-container.json

# Upload photos
atipicial-cli fs object put --container CID --file vacation.jpg --content-type image/jpeg
atipicial-cli fs object put --container CID --file family.jpg --content-type image/jpeg

# Set private access permissions
atipicial-cli fs acl set --container CID --basic 0600
```

### Sharing Files with Collaborators

```bash
# Create a container for shared documents
atipicial-cli fs container create --config shared-docs.json

# Upload documents
atipicial-cli fs object put --container CID --file presentation.pdf --content-type application/pdf

# Set extended permissions for collaborators
atipicial-cli fs acl extend --container CID --config collaborators-acl.json
```

## Available Endpoints

### Mainnet

| Service Type | Endpoint URL |
|--------------|--------------|
| gRPC API     | grpc.mainnet.fs.atipicial.com:8082 |
| HTTP Gateway | https://http.mainnet.fs.atipicial.com |
| REST API     | https://rest.mainnet.fs.atipicial.com |

### Testnet

| Service Type | Endpoint URL |
|--------------|--------------|
| gRPC API     | grpc.testnet.fs.atipicial.com:8082 |
| HTTP Gateway | https://http.testnet.fs.atipicial.com |
| REST API     | https://rest.testnet.fs.atipicial.com |

## Tips and Best Practices

1. **Wallet Security**: Always keep your wallet secure - it contains the keys needed to access your AtipicialFs data.

2. **Container Organization**: Create separate containers for different types of data or different access patterns.

3. **Basic ACL**: Use the following permission values:
   - `0644`: Public read, owner write (like a public website)
   - `0640`: Group read, owner write (like team sharing)
   - `0600`: Owner only (private data)

4. **Metadata**: Use attributes to add searchable metadata to your objects.

5. **HTTP Gateway**: For public content, you can share links via the HTTP gateway:
   ```
   https://http.mainnet.fs.atipicial.com/container_id/object_id
   ```

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
