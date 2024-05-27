if [[ -z $1 ]] then
	echo Must specify the path to the CA
	exit 1
fi

openssl genrsa -aes256 -out client.key 2048
openssl req -out client.csr -key client.key -new
openssl x509 -req -in client.csr -CA $1/ca.crt -CAkey $1/ca.key -CAcreateserial -out client.crt -days 365
