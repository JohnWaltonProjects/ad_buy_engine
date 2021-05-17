FROM rust:slim-buster

WORKDIR /app

RUN mkdir /app/migrations && mkdir /app/static

RUN apt-get update -y && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev && \
    apt-get update -y && \
    apt-get install -y libsqlite3-dev && \
   apt-get update -y && \
   apt install pkg-config -y && \
   apt-get update -y && \
    apt install libssl-dev && \
    apt install curl -y

COPY migrations /app/migrations
COPY bin/campaign_server /app/campaign_server

#COPY bin/couch_app /app/couch_app
#CMD ./couch_app
#CMD ./campaign_server


COPY static /app/static
COPY GeoLite2-ASN.mmdb /app/GeoLite2-ASN.mmdb
COPY GeoLite2-City.mmdb /app/GeoLite2-City.mmdb
COPY .env /app

EXPOSE 80
EXPOSE 443
#EXPOSE 1488

ENTRYPOINT [ "/app/campaign_server" ]


#FROM rust:slim-buster AS base
#
#RUN apt-get update -y && \
#    apt-get -y upgrade && \
#    apt-get -y install libpq-dev && \
#    apt-get update -y && \
#    apt-get install -y libsqlite3-dev && \
##   apt-get update -y && \
##   apt-get install -y default-libmysqlclient-dev && \
#   apt-get update -y && \
#   apt install pkg-config -y && \
#   apt-get update -y && \
#    apt install libssl-dev
#
#RUN rustup default nightly
#
#WORKDIR /code
#COPY . /code
#
#RUN cargo build -p campaign_server --features=backend --release
#
#FROM rust:slim-buster
#
#RUN apt-get update -y && \
#    apt-get -y upgrade && \
#    apt-get -y install libpq-dev && \
#    apt-get update -y && \
#    apt-get install -y libsqlite3-dev && \
##   apt-get update -y && \
##   apt-get install -y default-libmysqlclient-dev && \
#   apt-get update -y && \
#   apt install pkg-config -y && \
#   apt-get update -y && \
#    apt install libssl-dev
#
#RUN  mkdir /usr/bin/static
#RUN  mkdir /usr/bin/migrations
#COPY --from=base /code/target/release/campaign_server /usr/bin/campaign_server
#
#
#EXPOSE 80
#EXPOSE 443
#
#ENTRYPOINT [ "/usr/bin/campaign_server" ]