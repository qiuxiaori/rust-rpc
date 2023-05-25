FROM docker.eas.jc.in/eas-rpc-server:base 

COPY . .

RUN cargo build

EXPOSE 5000

CMD ["cargo", "run", "--bin", "server"]

