#!/bin/bash
set -e

# prepare config file for root CA generation
cat <<EOF >> root.cnf
[ req ]
distinguished_name = req_dn
[ req_dn ]
[ v3_ca ]
basicConstraints = CA:TRUE
keyUsage = digitalSignature, nonRepudiation, keyCertSign, cRLSign
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always
EOF

ROOT_CA_KEY=root-ca.key.pem
ROOT_CA=root-ca.pem
ROOT_CA_DER=root-ca.der

echo "Generate root CA key"
openssl genrsa -out $ROOT_CA_KEY 4096

echo "Generate root CA certificate"
openssl req -x509 -new -key $ROOT_CA_KEY -out $ROOT_CA -days 3650 -SHA256 -subj "/C=US/ST=Pennsylvania/O=Gwynedd Mercy University" -config root.cnf -extensions v3_ca
openssl x509 -outform der -in $ROOT_CA -out $ROOT_CA_DER

rm root.cnf

# prepare config file for server certificate generation
cat <<EOF >> server.cnf
extendedKeyUsage=serverAuth
subjectAltName = @alt_names
[alt_names]
DNS.1 = foobar.com
EOF


SERVER_KEY=server.key.pem
SERVER_KEY_RSA=server.key.rsa
SERVER_CERT=cert.pem
SERVER_CERT_DER=cert.der
IDENTITY=identity.p12
PASSPHRASE=mypass

echo "Generate server key"
openssl genrsa -out $SERVER_KEY 4096

echo "Generate server certificate"
openssl req -out server.csr -key $SERVER_KEY -new -SHA256 -subj "/C=US/ST=Pennsylvania/O=Gwynedd Mercy University/CN=home.thesheerans.com"
openssl x509 -req -days 3650 -SHA256 -in server.csr -CA $ROOT_CA -CAkey $ROOT_CA_KEY -CAcreateserial -out $SERVER_CERT -extfile server.cnf
openssl x509 -outform der -in $SERVER_CERT -out $SERVER_CERT_DER

echo "Convert server key to RSA"
openssl rsa -in $SERVER_KEY -out $SERVER_KEY_RSA -traditional

openssl pkcs12 -export -out $IDENTITY -inkey $SERVER_KEY -in $SERVER_CERT -passout pass:$PASSPHRASE

rm server.csr
rm server.cnf
