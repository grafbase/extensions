# Docker mTLS Setup

This directory contains files for the postgres-mtls container in compose.yaml for mTLS tests. Some small little nuggets for you to know so you don't need to ask ping me right away (spent a good five hours building this):

- The user to use is testuser, you should not try to use a password.
- the certs/ directory has everything for mTLS. If things stop working, delete all the files and execute the `./generate-certs.sh` from this directory. That will generate all the files again, with maybe newer timestamps so everybody's happy again.
- If things go wrong in the container, stop it with `docker compose down -v` to remove the volume and enable executing the init scripts again.
- You need a special psql command to connect:

```bash
psql "host=localhost \
      port=5433 \
      dbname=postgres \
      user=testuser \
      sslmode=verify-full \
      sslrootcert=./docker/postgres-mtls/certs/ca.crt \
      sslcert=./docker/postgres-mtls/certs/client.crt \
      sslkey=./docker/postgres-mtls/certs/client.key"
```

OR

```
psql postgresql://testuser@localhost:5433/postgres?sslmode=verify-full&sslrootcert=./docker/postgres-mtls/certs/ca.crt&sslcert=./docker/postgres-mtls/certs/client.crt&sslkey=./docker/postgres-mtls/certs/client.key
```

Notice the paths in the end. This command needs to be executed in the extensions root directory.

And finally, "have fun".
