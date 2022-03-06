# simple-rust-psktls-server
this project show how to create a simple psk-tls(tls1.2) server based on openssl

# system info
- Tested in Windows11 WSL2 Ubuntu-18.04 
- Openssl 1.1.1 and libssl-dev is required
- Use ```openssl s_client -connect localhost:8443 -tls1_2 -psk 123456 -cipher PSK-AES128-CBC-SHA256 -psk_identity client``` to communicate with this psktls-server
