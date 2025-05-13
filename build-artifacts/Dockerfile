FROM debian:bookworm AS builder
WORKDIR /contracts

RUN apt-get -y update && apt-get install -y \
  python3-full \
  python3-pip \
  python3-venv \
  npm \
  wget \
  curl \
  git

RUN curl -L https://foundry.paradigm.xyz | bash 
ENV PATH="$PATH:/root/.foundry/bin"
RUN foundryup

# =========================================================================== #
#                         STARKGATE CONTRACTS (4594188)                       #
# =========================================================================== #

FROM builder AS starkgate-4594188
ENV STARKGATE_REV="45941888479663ac93e898cd7f8504fa9066c54c"
ENV STARKGATE_DIR="/contracts/starkgate-contracts"
ENV COMPILE_SIERRA=".downloads/cairo/bin/starknet-sierra-compile"

# clone repository
RUN git clone https://github.com/starknet-io/starkgate-contracts.git
WORKDIR starkgate-contracts
RUN git checkout $STARKGATE_REV

# setup python
RUN python3 -m venv venv
ENV PATH="$STARKGATE_DIR/venv/bin:$PATH"
RUN pip install -r requirements.txt

# Download cairo and solc
RUN scripts/setup.sh

RUN scripts/build-cairo.sh && scripts/build-solidity.sh

# =========================================================================== #
#                         STARKGATE CONTRACTS (c08863a)                       #
# =========================================================================== #

FROM ciimage/python:3.7 AS starkgate-c08863a
ENV STARKGATE_REV="c08863a1f08226c09f1d0748124192e848d73db9"

WORKDIR /contracts

RUN apt-get update -y
RUN apt -y -o Dpkg::Options::="--force-overwrite" install python3.7-dev
RUN apt-get install -y \
  libgmp3-dev \
  make \
  g++ \
  npm \
  unzip \
  git

# Installing cmake via apt doesn't bring the most up-to-date version.
RUN pip install cmake==3.22

RUN git clone https://github.com/starknet-io/starkgate-contracts.git
WORKDIR starkgate-contracts
RUN git checkout $STARKGATE_REV

# Install solc and ganache
RUN curl https://binaries.soliditylang.org/linux-amd64/solc-linux-amd64-v0.6.12+commit.27d51765 -o /usr/local/bin/solc-0.6.12
RUN echo 'f6cb519b01dabc61cab4c184a3db11aa591d18151e362fcae850e42cffdfb09a /usr/local/bin/solc-0.6.12' | sha256sum --check
RUN chmod +x /usr/local/bin/solc-0.6.12
RUN npm install -g --unsafe-perm ganache-cli@6.12.2

# Build.
RUN ./build.sh

# =========================================================================== #
#                         STARKGATE CONTRACTS (82e651f)                       #
# =========================================================================== #

FROM builder AS starkgate-82e651f
ENV STARKGATE_REV="82e651f5f642508577b6310f47d0d457f5f9a9bb"
ENV STARKGATE_DIR="/contracts/starkgate-contracts"

# Clone repository
RUN git clone https://github.com/starknet-io/starkgate-contracts.git
WORKDIR starkgate-contracts
RUN git checkout $STARKGATE_REV

# Build
WORKDIR src/starkware
COPY build-artifacts/foundry-starkgate.toml foundry.toml
RUN forge build

# =========================================================================== #
#                                   CAIRO-LANG                                #
# =========================================================================== #

FROM builder as cairo-lang
ENV CAIRO_LANG_REV="8e11b8cc65ae1d0959328b1b4a40b92df8b58595"
ENV CAIRO_LANG_DIR="/contracts/cairo-lang"

# Clone the repo
RUN git clone https://github.com/starkware-libs/cairo-lang.git
WORKDIR cairo-lang
RUN git checkout $CAIRO_LANG_REV

# Build
COPY build-artifacts/foundry-cairo-lang.toml foundry.toml
RUN forge build

# =========================================================================== #
#                               LOCAL CONTRACTS                               #
# =========================================================================== #

FROM builder AS local-contracts
WORKDIR local-contracts

COPY src src
COPY foundry.toml .

RUN mkdir build-artifacts && \
    cd build-artifacts && \
    git clone https://github.com/starkware-libs/cairo-lang.git

# Build
RUN forge build

# =========================================================================== #
#                                  ARTIFACTS                                  #
# =========================================================================== #

FROM scratch
WORKDIR artifacts

WORKDIR starkgate_4594188

ENV STARKGATE_DIR="/contracts/starkgate-contracts"
ENV ARTIFACTS_CAIRO_4594188="$STARKGATE_DIR/cairo_contracts"
ENV ARTIFACTS_SOLID_4594188="$STARKGATE_DIR/artifacts"

COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarkgateManager.json StarkgateManager.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarkgateRegistry.json StarkgateRegistry.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/Proxy.json Proxy_5_0_0.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarknetTokenBridge.json StarknetTokenBridge.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/TestERC20.json ERC20.json

WORKDIR ../starkgate_c08863a

ENV STARKGATE_DIR="/contracts/starkgate-contracts/build/Release/src"
ENV ARTIFACTS_SOLID_c08863a="$STARKGATE_DIR/starkware/solidity/upgrade"

COPY --from=starkgate-c08863a $ARTIFACTS_SOLID_c08863a/Proxy.json Proxy_3_0_2.json

WORKDIR ../starkgate_82e651f

ENV STARKGATE_DIR="/contracts/starkgate-contracts"
ENV ARTIFACTS_SOLID_82e651f="$STARKGATE_DIR/src/starkware/out"

COPY --from=starkgate-82e651f $ARTIFACTS_SOLID_82e651f/StarknetEthBridge.sol/StarknetEthBridge.json StarknetLegacyBridge.json

WORKDIR ../cairo_lang

ENV CAIRO_LANG_DIR="/contracts/cairo-lang"

COPY --from=cairo-lang $CAIRO_LANG_DIR/out/Starknet.sol/Starknet.json Starknet.json

WORKDIR ../local-contracts

ENV LOCAL_CONTRACTS_DIR="/contracts/local-contracts"
ENV ARTIFACTS_LOCAL="$LOCAL_CONTRACTS_DIR/out"

COPY --from=local-contracts $ARTIFACTS_LOCAL/StarknetDevCoreContract.sol/Starknet.json .
COPY --from=local-contracts $ARTIFACTS_LOCAL/UnsafeProxy.sol/UnsafeProxy.json .
