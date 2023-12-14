# 使用一个基础镜像
FROM rust:1.74 as build

# 创建一个新的工作目录
WORKDIR /app

COPY . .

RUN  echo "[source.crates-io]\n\
replace-with = 'rsproxy-sparse'\n\
[source.rsproxy]\n\
registry = \"https://rsproxy.cn/crates.io-index\"\n\
[source.rsproxy-sparse]\n\
registry = \"sparse+https://rsproxy.cn/index/\"\n\
[registries.rsproxy]\n\
index = \"https://rsproxy.cn/crates.io-index\"\n\
[net]\n\
git-fetch-with-cli = true\n" >> $CARGO_HOME/config

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo build --release

EXPOSE 8080

RUN cp /app/target/release/select_course /usr/local/bin/select_course

RUN rm -rf target

RUN chmod +x /usr/local/bin/select_course

CMD ["/usr/local/bin/select_course"]
