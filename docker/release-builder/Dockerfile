FROM ubuntu:22.04

RUN apt update && apt install curl wget git build-essential libssl-dev pkg-config ca-certificates rsync -y

RUN set -uex; \
    apt-get update; \
    apt-get install -y ca-certificates curl gnupg; \
    mkdir -p /etc/apt/keyrings; \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
     | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg; \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_22.x nodistro main" \
     > /etc/apt/sources.list.d/nodesource.list; \
    apt-get -qy update; \
    apt-get -qy install nodejs;

RUN npm install --global yarn \ && npm install --global vite

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain=1.87.0 -y && cargo install sqlx-cli --version 0.8.3

RUN wget https://github.com/jwilder/dockerize/releases/download/v0.3.0/dockerize-linux-amd64-v0.3.0.tar.gz && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-v0.3.0.tar.gz && rm dockerize-linux-amd64-v0.3.0.tar.gz

ENV DATABASE_URL=postgres://asami:password@db_container:5432/asami_development

RUN mkdir /work

SHELL ["/bin/bash", "-c"]

CMD /bin/bash -c "\
  rm -rf /work/* \
  && rsync -av --progress /host_workspace/ /work --exclude-from /host_workspace/.gitignore \
  && chown -R root:root /work \
  && mkdir -p /host_workspace/release \
  && rm -rf /host_workspace/release/pwa/assets/index-*.js \
  && cd /work/pwa \
  && yarn cache clean \
  && yarn \
  && yarn build \
	&& cp -r /work/pwa/dist /host_workspace/release/pwa \
  && cd /work/api && sqlx database reset -y \
  && cd /work/api && cargo build --release --bin migrator --bin api --bin worker \
  && cp /work/target/release/{migrator,api,worker} /host_workspace/release \
  && chown -R $HOST_UID:$HOST_UID /host_workspace/release \
"


