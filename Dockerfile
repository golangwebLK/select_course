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

ENV DATABASE_URL=mysql://root:wonderful123.@bj-cynosdbmysql-grp-34c8azma.sql.tencentcdb.com:27846/select_course

ENV SERVER_IP=0.0.0.0:8080

ENV MANAGER_BASEURL=http://127.0.0.1:25480/api
ENV COOKIE_NAME=_session_
ENV COOKIE_VALUE=eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxIiwiZXhwIjoxNzI5ODQzNDU3fQ.N_t_j8BUCaG3plBQonPXzFAN_t5R4-iwDOsRxCHMr7Lsm17Gd5bsrEyE5kOKSLcPdyYYK7HCU2jql4QC4iPc0zoN4BNAl4hWDhk51xfYB-n77FVlpIfR4IANaPqYYnj0qajw0okEoE57YclMNGIF2VUVFd372eIJrEVYWPuue6SBDRKklxobgaO1hqKe6fr3yalNteeOZvuJq9Rmg2QEgIbRvjyrbs9CZcLtTGXZQgRfXdVrEHSmGO4Ct_OBt97OnMQj6qfFLlHFwhl4fmIdqrVWrmcVE8f1xjbo2fdpWxNc1YtNC-0ciZvcGhiqxnnEJhd1NqVKMpmi6vUrxCgh9w

WORKDIR /apps

EXPOSE 8080

RUN chmod +x /app/target/release/select_course

CMD ["/app/target/release/select_course"]
