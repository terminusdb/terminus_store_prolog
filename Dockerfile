# First build with rust and c libs
FROM terminusdb/swipl:v8.2.3
WORKDIR /usr/share/swi-prolog/pack/terminus_store_prolog
COPY . .
RUN BUILD_DEPS="git build-essential curl" && apt-get update \
	&& apt-get install -y --no-install-recommends $BUILD_DEPS \
        ca-certificates \
    make
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./make.sh && apt-get purge -y --auto-remove $BUILD_DEPS \
       && rm -rf rust/target/release/build && rm -rf rust/target/release/deps

FROM terminusdb/swipl:v8.2.3
WORKDIR /usr/share/swi-prolog/pack/terminus_store_prolog
COPY --from=0 /usr/share/swi-prolog/pack/terminus_store_prolog /usr/share/swi-prolog/pack/terminus_store_prolog
