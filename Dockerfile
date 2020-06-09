# First build with rust and c libs
FROM terminusdb/swipl:v8.2.0
WORKDIR /usr/lib/swipl/pack/terminus_store_prolog
COPY . .
RUN BUILD_DEPS="git build-essential curl" && apt-get update \
	&& apt-get install -y --no-install-recommends $BUILD_DEPS \
        ca-certificates \
    make
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./make && apt-get purge -y --auto-remove $BUILD_DEPS

FROM terminusdb/swipl:v8.0.3
WORKDIR /usr/lib/swipl/pack/terminus_store_prolog
COPY --from=0 /usr/lib/swipl/pack/terminus_store_prolog /usr/lib/swipl/pack/terminus_store_prolog
