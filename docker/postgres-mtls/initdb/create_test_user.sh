#!/bin/bash
set -e

# Create a user 'testuser' that matches the CN of the client certificate.
# If you are not using a pg_ident.conf map, the username must match the client cert's CN.
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE USER testuser;
    ALTER ROLE testuser SUPERUSER;
    GRANT ALL PRIVILEGES ON DATABASE "$POSTGRES_DB" TO testuser;
EOSQL
