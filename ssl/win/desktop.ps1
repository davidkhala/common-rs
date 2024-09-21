$env:VCPKG_ROOT="C:\Program Files\Microsoft Visual Studio\2022\Community\VC\vcpkg"
choco install -y openssl --force
$env:OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
$env:OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD"
