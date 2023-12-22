# 使用一个基础镜像
FROM rust:1.74
# 创建一个新的工作目录
WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features mysql

RUN cargo build --release

EXPOSE 8080

RUN cp /app/target/release/select_course /usr/local/bin/select_course

RUN cargo clean

RUN chmod +x /usr/local/bin/select_course

CMD ["/usr/local/bin/select_course"]
