# 使用一个基础镜像
FROM rust:1.71 as build

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


RUN cargo build --release


FROM rust:1.71

ENV DATABASE_URL=mysql://root:wonderful123.@bj-cynosdbmysql-grp-34c8azma.sql.tencentcdb.com:27846/student_manager_data

ENV SERVER_IP=0.0.0.0:8080

WORKDIR /apps

EXPOSE 8080

RUN cargo install diesel_cli
RUN cargo install diesel_cli --no-default-features --features mysql

COPY --from=build /app/target/release/select_course /usr/local/bin/

RUN chmod +x /usr/local/bin/select_course

CMD ["/usr/local/bin/select_course"]

