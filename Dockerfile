FROM swipl:stable
WORKDIR /usr/lib/swipl/pack/terminus_store_prolog
COPY . .
RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
        git \
	build-essential \
        curl \
    make
# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./make
