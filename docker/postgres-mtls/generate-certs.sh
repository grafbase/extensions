#!/bin/bash

# 1. Generate CA Key and Certificate
openssl genrsa -out certs/ca.key 4096
openssl req -new -x509 -days 3650 -key certs/ca.key -out certs/ca.crt \
  -subj "/C=US/ST=California/L=SanFrancisco/O=MyTestOrg/OU=CA/CN=MyTestCA"

# 2. Generate Server Key and Certificate Signing Request (CSR)
# IMPORTANT: Set CN (Common Name) to 'postgres' or the hostname your client will use to connect to the DB service.
# For Docker Compose, 'postgres' is often the service name and hostname.
openssl genrsa -out certs/server.key 4096
openssl req -new -key certs/server.key -out certs/server.csr \
  -subj "/C=US/ST=California/L=SanFrancisco/O=MyTestOrg/OU=Server/CN=postgres"

# 3. Sign the Server Certificate with the CA
openssl x509 -req -days 365 -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/server.crt \
  -extfile <(printf "subjectAltName=DNS:postgres,DNS:localhost,IP:127.0.0.1") # Add SANs if needed

# 4. Generate Client Key and Certificate Signing Request (CSR)
# IMPORTANT: The CN for the client certificate will be used to map to a PostgreSQL user.
# Let's use 'testuser' as the CN for the client certificate.
openssl genrsa -out certs/client.key 4096
openssl req -new -key certs/client.key -out certs/client.csr \
  -subj "/C=US/ST=California/L=SanFrancisco/O=MyTestOrg/OU=Client/CN=testuser"

# 5. Sign the Client Certificate with the CA
openssl x509 -req -days 365 -in certs/client.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client.crt

# 6. Set appropriate permissions (especially for keys)
chmod 600 certs/*.key
