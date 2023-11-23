FROM cimg/rust:1.67.1
RUN sudo apt-get update
RUN sudo apt-get install openssl libssl-dev psmisc gettext
RUN wget https://github.com/jwilder/dockerize/releases/download/v0.3.0/dockerize-linux-amd64-v0.3.0.tar.gz && sudo tar -C /usr/local/bin -xzvf dockerize-linux-amd64-v0.3.0.tar.gz && rm dockerize-linux-amd64-v0.3.0.tar.gz
RUN cargo install sqlx-cli
