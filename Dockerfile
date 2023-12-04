# 使用一个基础镜像
FROM rust:1.70 as build

# 创建一个新的工作目录
WORKDIR /app

COPY . /app/

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

RUN cargo build --release

FROM alpine

ENV TZ=Asia/Shanghai

RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

WORKDIR /app

EXPOSE 8959

COPY --from=build /app/target/release/select_course /usr/local/bin/

RUN chmod +x /usr/local/bin/select_course

CMD ["/usr/local/bin/select_course"]

