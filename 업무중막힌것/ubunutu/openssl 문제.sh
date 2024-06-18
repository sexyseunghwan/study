##################################################################### 
###### 1. RUST 를 Ubuntu 에서 컴파일 하는 도중에 아래와 같은 에러가 발생함 ######
##################################################################### 
# [ 에러메시지 ]

"
Compiling openssl-sys v0.9.102
Compiling num_cpus v1.16.0
error: failed to run custom build command for openssl-sys v0.9.102

Caused by:
process didn't exit successfully: /home/wmp-user/index_clear_program/elastic_index_clear/target/debug/build/openssl-sys-934b51f7854c899f/build-script-main (exit status: 101)
--- stdout
cargo
=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR unset
cargo
=OPENSSL_LIB_DIR
OPENSSL_LIB_DIR unset
cargo
=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR unset
cargo
=OPENSSL_INCLUDE_DIR
OPENSSL_INCLUDE_DIR unset
cargo
=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR unset
cargo
=OPENSSL_DIR
OPENSSL_DIR = /usr/include/openssl

--- stderr
thread 'main' panicked at /root/.cargo/registry/src/index.crates.io-6f17d22bba15001f/openssl-sys-0.9.102/build/main.rs:96:9:
OpenSSL include directory does not exist: /usr/include/openssl/include
note: run with RUST_BACKTRACE=1 environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
"

# 해결방법 -> libssl-dev 를 install
sudo apt update
sudo apt install libssl-dev   

# 사실 이부분에서도 해결되지 않음
# openssl 의 헤더파일과 라이브러리 파일 경로를 환경변수로 설정하는 작업이 필요함.
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include/openssl

# OPENSSL_LIB_DIR -> 애를 환경변수로 설정하는 이유는 아래와 같다.
# openssl.so -> SSL 통신을 처리하는 데 사용되는 공유 라이브러리. 해당 라이브러리를 심볼릭 링크처리하려고 하는것.

root@xxxxx:/usr/lib/x86_64-linux-gnu
# ls -l | grep open
lrwxrwxrwx  1 root root       28 Jun 29  2019 libxmlsec1-openssl.so.1 -> libxmlsec1-openssl.so.1.2.28
-rw-r--r--  1 root root   283272 Jun 29  2019 libxmlsec1-openssl.so.1.2.28
drwxr-xr-x  3 root root     4096 Jul 27  2022 open-vm-tools

