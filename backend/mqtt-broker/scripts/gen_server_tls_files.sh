if [[ -z $1 ]] then
	echo Must specify the path to the CA
	exit 1
fi

openssl genrsa -out broker.key 2048
openssl req -out broker.csr -key broker.key -new
openssl x509 -req -in broker.csr -CA $1/ca.crt -CAkey $1/ca.key -CAcreateserial -out broker.crt -days 365
