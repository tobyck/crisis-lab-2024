mkdir -p tls/ca && cd $_
bash ../../scripts/gen_ca.sh

mkdir ../broker && cd $_
bash ../../scripts/gen_server_tls_files.sh ../ca

mkdir ../client1 && cd $_
bash ../../scripts/gen_client_tls_files.sh ../ca

mkdir ../client2 && cd $_
bash ../../scripts/gen_client_tls_files.sh ../ca
