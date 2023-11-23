FROM ubuntu:22.04
RUN apt update && apt install curl wget xvfb unzip git build-essential libssl-dev pkg-config ca-certificates -y

# Set up the Chrome PPA
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -
RUN echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list

# Update the package list and install chrome
RUN apt-get update -y
RUN apt-get install -y google-chrome-stable

# Set up Chromedriver Environment variables
ENV CHROMEDRIVER_VERSION 114.0.5735.90
ENV CHROMEDRIVER_DIR /chromedriver
RUN mkdir $CHROMEDRIVER_DIR

# Download and install Chromedriver
RUN wget -q --continue -P $CHROMEDRIVER_DIR "http://chromedriver.storage.googleapis.com/$CHROMEDRIVER_VERSION/chromedriver_linux64.zip"
RUN unzip $CHROMEDRIVER_DIR/chromedriver* -d $CHROMEDRIVER_DIR

# Put Chromedriver into the PATH
ENV PATH $CHROMEDRIVER_DIR:$PATH

RUN curl -sL https://deb.nodesource.com/setup_18.x -o /tmp/nodesource_setup.sh \
  && chmod +x /tmp/nodesource_setup.sh \
  && /tmp/nodesource_setup.sh \
  && apt install nodejs -y \
  && npm install --global yarn

ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain=1.67.1 -y && cargo install sqlx-cli

RUN wget https://github.com/jwilder/dockerize/releases/download/v0.3.0/dockerize-linux-amd64-v0.3.0.tar.gz && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-v0.3.0.tar.gz && rm dockerize-linux-amd64-v0.3.0.tar.gz

ENV DATABASE_URL=postgres://asami:password@db_container:5432/asami_development

RUN mkdir /work

CMD /bin/bash -c "\
  rm -rf /work/* \
  && rsync -av --progress /host_workspace/ /work --exclude-from /host_workspace/.gitignore \
  && chown -R root:root /work \
  && cd /work/api && sqlx database reset -y \
  && cd /work/integration_tests \
  && ./run
"
