# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh 
curl https://sh.rustup.rs -sSf | sh -s -- -y
cargo install --path .

datagen create -h
