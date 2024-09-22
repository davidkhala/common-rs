# $env:VCPKG_ROOT="C:\Program Files\Microsoft Visual Studio\2022\Community\VC\vcpkg"
[Environment]::SetEnvironmentVariable("VCPKG_ROOT", "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\vcpkg", "User")

choco install -y openssl --force

# $env:OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
[Environment]::SetEnvironmentVariable("OPENSSL_DIR", "C:\Program Files\OpenSSL-Win64", "User")

# $env:OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD"
[Environment]::SetEnvironmentVariable("OPENSSL_LIB_DIR", "C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD", "User")