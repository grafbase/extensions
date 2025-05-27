#!/bin/bash
# Script to generate SSL certificates for Kafka TLS container

set -e

# Set working directory to the location of this script
cd "$(dirname "$0")/ssl"

# Configuration
PASSWORD=keystorepassword
TRUSTSTORE_PASSWORD=truststorepassword
KEY_PASSWORD=keystorepassword
VALIDITY_DAYS=3650
CA_CERT="ca-cert"
SERVER_KEYSTORE="kafka.keystore.jks"
SERVER_TRUSTSTORE="kafka.truststore.jks"
CLIENT_KEYSTORE="kafka.client.keystore.jks"
CLIENT_TRUSTSTORE="kafka.client.truststore.jks"

echo "=== Generating SSL certificates for Kafka TLS ==="
echo "Certificates will be generated in the ssl directory"

# Clean up existing certificates
rm -f *.crt *.key *.jks *.csr *.srl *.conf

# Create a key pair for the CA
echo "Generating CA key pair..."
openssl req -new -x509 -keyout $CA_CERT.key -out $CA_CERT \
  -days $VALIDITY_DAYS -nodes \
  -subj "/C=US/ST=Test/L=Test/O=Kafka/OU=Kafka/CN=ca.kafka.test"

# Create a key pair for the Kafka server
echo "Generating Kafka server key pair..."
keytool -genkey -keystore $SERVER_KEYSTORE -alias kafka -validity $VALIDITY_DAYS \
  -keyalg RSA -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS \
  -dname "CN=kafka-tls, OU=Kafka, O=Kafka, L=Test, ST=Test, C=US"

# Create a CSR for the Kafka server
echo "Creating CSR for Kafka server..."
keytool -keystore $SERVER_KEYSTORE -alias kafka -certreq -file kafka.csr \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS

# Create extension file for server certificate with SAN
cat > server_ext.conf <<EOF
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = kafka-tls
DNS.2 = kafka-mtls
DNS.3 = localhost
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

# Sign the Kafka server certificate with the CA
echo "Signing Kafka server certificate with CA..."
openssl x509 -req -CA $CA_CERT -CAkey $CA_CERT.key -in kafka.csr -out kafka.crt \
  -days $VALIDITY_DAYS -CAcreateserial -passin pass:$PASSWORD \
  -extensions v3_req -extfile server_ext.conf

# Import the CA and signed certificate into the Kafka server keystore
echo "Importing CA into server keystore..."
keytool -keystore $SERVER_KEYSTORE -alias CARoot -import -file $CA_CERT \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

echo "Importing signed server certificate into server keystore..."
keytool -keystore $SERVER_KEYSTORE -alias kafka -import -file kafka.crt \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

# Create a truststore for the Kafka server
echo "Creating truststore for Kafka server..."
keytool -keystore $SERVER_TRUSTSTORE -alias CARoot -import -file $CA_CERT \
  -storepass $TRUSTSTORE_PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

# Create client keystore and truststore for testing
echo "Creating client keystore and truststore..."

# Generate client keys
keytool -genkey -keystore $CLIENT_KEYSTORE -alias client -validity $VALIDITY_DAYS \
  -keyalg RSA -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS \
  -dname "CN=kafka-client, OU=Kafka, O=Kafka, L=Test, ST=Test, C=US"

# Create a CSR for the client
keytool -keystore $CLIENT_KEYSTORE -alias client -certreq -file client.csr \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS

# Create extension file for client certificate with SAN
cat > client_ext.conf <<EOF
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
extendedKeyUsage = clientAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = kafka-client
DNS.2 = localhost
IP.1 = 127.0.0.1
IP.2 = ::1
EOF

# Sign the client certificate with the CA
openssl x509 -req -CA $CA_CERT -CAkey $CA_CERT.key -in client.csr -out client.crt \
  -days $VALIDITY_DAYS -CAcreateserial -passin pass:$PASSWORD \
  -extensions v3_req -extfile client_ext.conf

# Import the CA and signed certificate into the client keystore
keytool -keystore $CLIENT_KEYSTORE -alias CARoot -import -file $CA_CERT \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

keytool -keystore $CLIENT_KEYSTORE -alias client -import -file client.crt \
  -storepass $PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

# Create a truststore for the client
keytool -keystore $CLIENT_TRUSTSTORE -alias CARoot -import -file $CA_CERT \
  -storepass $TRUSTSTORE_PASSWORD -keypass $KEY_PASSWORD -storetype JKS -noprompt

# Extract client private key for use with rustls (PEM format)
echo "Extracting client private key to PEM format..."
keytool -importkeystore -srckeystore $CLIENT_KEYSTORE -destkeystore client.p12 \
  -deststoretype PKCS12 -srcalias client -destalias client \
  -srcstorepass $PASSWORD -deststorepass $PASSWORD -noprompt

openssl pkcs12 -in client.p12 -nodes -nocerts -out client.key \
  -passin pass:$PASSWORD

echo "=== SSL certificates generated successfully ==="
echo "You may need to install the keytool and openssl commands if they're not already available."
echo "Server keystore:   $SERVER_KEYSTORE"
echo "Server truststore: $SERVER_TRUSTSTORE"
echo "Client keystore:   $CLIENT_KEYSTORE"
echo "Client truststore: $CLIENT_TRUSTSTORE"

# Clean up extension and temporary files
rm -f server_ext.conf client_ext.conf client.p12

# Set appropriate permissions
chmod 600 *.key *.jks
