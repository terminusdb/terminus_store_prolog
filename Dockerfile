# First build with rust and c libs
FROM terminusdb/swipl:8.0.2
WORKDIR /usr/lib/swi-prolog/pack/terminus_store_prolog
COPY . .
RUN apt-get update \
	&& apt-get install -y --no-install-recommends \
        git \
	build-essential \
        curl \
        ca-certificates \
    make
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN ./make

FROM terminusdb/swipl:8.0.2
WORKDIR /usr/lib/swi-prolog/pack/terminus_store_prolog
COPY --from=0 /usr/lib/swi-prolog/pack/terminus_store_prolog /usr/lib/swi-prolog/pack/terminus_store_prolog
