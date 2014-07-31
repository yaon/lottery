make && killall main && RUST_LOG=4 ./main& sleep 1 && nc -U ./socket-unix-test <<EOF
ADD titi toto
DEL titi
ADD thomas mariaux
GET thimas
GET thomas
DEL thomas
EOF
