FROM cimg/rust:1.76
RUN sudo apt update
RUN sudo apt install -y ca-certificates curl gnupg openssl libssl-dev psmisc gettext

RUN sudo mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
ENV NODE_MAJOR=18
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" /etc/apt/sources.list.d/nodesource.list

RUN sudo apt-get -qy update;
RUN sudo apt-get -qy install nodejs npm;
RUN wget https://github.com/jwilder/dockerize/releases/download/v0.3.0/dockerize-linux-amd64-v0.3.0.tar.gz && sudo tar -C /usr/local/bin -xzvf dockerize-linux-amd64-v0.3.0.tar.gz && rm dockerize-linux-amd64-v0.3.0.tar.gz

RUN sudo npm install --global yarn && sudo npm install --global vite
RUN cargo install sqlx-cli
ENV DATABASE_URL=postgres://asami:password@db_container:5432/asami_development
RUN sudo npm install ganache --global
