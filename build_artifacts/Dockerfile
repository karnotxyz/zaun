# =========================================================================== #
#                               PLEASE READ THIS                              #
#                                                                             #
# The build system for this repo is an absolute mess, with many external      #
# dependencies, each scattered across different versions, but it WORKS.       #
#                                                                             #
# Here are a few things you should NOT do:                                    #
#                                                                             #
# - Commiting artifacts to vcs is impractical and not flexible at all, what   #
#   if we want to update versions? We should add this as submodules instead   #
#   and just build from source each time!                                     #
#                                                                             #
# Don't. First of all submodules are a pain to work with. Second of all this  #
# adds many external dependencies aside from `cargo` to build this crate.     #
# This is especially bad when this crate is imported into another project,    #
# since cargo will fail the build if all required dependencies are not        #
# installed globally. IE: your Rust project is now a Python project and a     #
# Solidity project, and a Cairo project... this is a pain to work with.       #
#                                                                             #
# - This Dockerfile is a mess, I should refactor it!                          #
#                                                                             #
# Don't. This file is a mess because building all the required dependencies,  #
# sometimes event the same dependency across multiple versions and multiple   #
# build systems, is a mess. It really isn't worth trying to refactor this so  #
# long as it works.                                                           #
#                                                                             #
# - I really need some new functionality. We should update the dependencies.  #
#                                                                             #
# Don't (if you can). You might be lucky and the build scripts for the new    #
# version will not have changed. Otherwise keep in mind that you will have to #
# track down build scripts across multiple repos, debug missing dependencies  #
# and file for the coming days to get everything to work. If you do REALLY    #
# need to update the dependencies, here are a few tips which might help you:  #
#                                                                             #
# 1. Don't try and guess how each project is built, and don't just assume the #
#    current commands will work. Check out each dep at the updated rev, read  #
#    their build scripts and inline them into this file.                      #
#                                                                             #
# 2. Use `docker build ... --progresss plain`. This will print out all the    #
#    commands which are run as the image is being built and allow you to use  #
#    `RUN echo` and `RUN ls` for debug statments without them being erased.   #
#                                                                             #
# 3. Docker likes to cache build steps. This means that your `RUN echo`       #
#    statements and the like will only be run on their first build. You can   #
#    run `docker build ... --no-cache` but this will rebuild the entire image.#
#    Otherwise, you can force a cache invalidation by just adding an          #
#    `ENV foo=bazz` above the statement which you want to re-run.             #
#                                                                             #
# 4. THIS REPO DOES NOT HAVE ANY UNIT TESTS. You should not rely on cargo test#
#    to check if you're change haven't broken the artifacts. Madara           #
#                                                                             #
#                  https://github.com/madara-alliance/madara                  #
#                                                                             #
#    has some test in its bootstrapper code which uses zaun in an e2e context.#
#    Update the deps there to point to your fixes, run the tests and make sure#
#    they pass. If that is the case you should be good.                       #
#                                                                             #
# 5. REMEMBER TO COMMIT THE NEWLY GENERATED ARTIFACTS TO VCS. There is nothing#
#    in the CI which enforces that changes to the build Dockerfile should     #
#    coincide with changes to the artifacts. This is something you need to    #
#    remember to do manually.                                                 #
#                                                                             #
# =========================================================================== #

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

# clone repository
RUN git clone https://github.com/starknet-io/starkgate-contracts.git
WORKDIR starkgate-contracts
RUN git checkout $STARKGATE_REV

# setup python
RUN python3 -m venv venv
ENV PATH="$STARKGATE_DIR/venv/bin:$PATH"
RUN pip install -r requirements.txt

# build
RUN scripts/setup.sh && scripts/build-solidity.sh

COPY build_artifacts/foundry-starkgate-4594188.toml starkware/solidity/foundry.toml
COPY build_artifacts/ERC20.sol starkware/solidity/tokens/ERC20/ERC20_1.sol
RUN cd starkware/solidity && forge build

# Extract
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                         STARKGATE CONTRACTS (c08863a)                       #
# =========================================================================== #

FROM ciimage/python:3.7 AS starkgate-c08863a
ENV STARKGATE_REV="c08863a1f08226c09f1d0748124192e848d73db9"
ENV SOLC_VERSION="0.6.12"
ENV SOLC_URL="https://binaries.soliditylang.org/linux-amd64/solc-linux-amd64-v$SOLC_VERSION+commit.27d51765"

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
RUN curl $SOLC_URL -o /usr/local/bin/solc-$SOLC_VERSION
RUN chmod +x /usr/local/bin/solc-$SOLC_VERSION
RUN npm install -g --unsafe-perm ganache-cli@6.12.2

# Build.
RUN ./build.sh

# Extract
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                         STARKGATE CONTRACTS (82e651f)                       #
# =========================================================================== #

FROM builder AS starkgate-82e651f
ENV STARKGATE_REV="82e651f5f642508577b6310f47d0d457f5f9a9bb"

# Clone repository
RUN git clone https://github.com/starknet-io/starkgate-contracts.git
WORKDIR starkgate-contracts
RUN git checkout $STARKGATE_REV

# Build
WORKDIR src/starkware
COPY build_artifacts/foundry-starkgate-82e651f.toml foundry.toml
RUN forge build

# Extract
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                                   CAIRO-LANG                                #
# =========================================================================== #

FROM builder AS cairo-lang
ENV CAIRO_LANG_REV="8e11b8cc65ae1d0959328b1b4a40b92df8b58595"

# Clone the repo
RUN git clone https://github.com/starkware-libs/cairo-lang.git
WORKDIR cairo-lang
RUN git checkout $CAIRO_LANG_REV

# Build
COPY build_artifacts/foundry-cairo-lang.toml foundry.toml
RUN forge build

# Extract
WORKDIR out
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                                  PILTOVER                                   #
# =========================================================================== #

FROM builder AS piltover
ENV PILTOVER_REV="ff8853fde770fac31ea071892242e12cd386a3f1"
ENV SCARB_VERSION="2.5.3"
ENV SCARB_RELEASES="https://github.com/software-mansion/scarb/releases/download"
ENV SCARB_URL="$SCARB_RELEASES/v$SCARB_VERSION/scarb-v$SCARB_VERSION-x86_64-unknown-linux-gnu.tar.gz"

# Clone the repo
RUN git clone https://github.com/keep-starknet-strange/piltover.git
WORKDIR piltover
RUN git checkout $PILTOVER_REV

# Setup scarb
RUN mkdir scarb && wget -c $SCARB_URL -O - | tar -xz -C scarb --strip-components=1

# Build
RUN ./scarb/bin/scarb build

# Extract
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                               LOCAL CONTRACTS                               #
# =========================================================================== #

FROM builder AS local-contracts
ENV CAIRO_LANG_REV="8e11b8cc65ae1d0959328b1b4a40b92df8b58595"
ENV STARKNET_TOKEN_REV="ffdef4e66b0c02627cbed7629373019d4c1e137b"

WORKDIR local-contracts

COPY src src
COPY foundry.toml remappings.txt .

RUN mkdir -p lib
RUN cd lib && \
    git clone https://github.com/starkware-libs/cairo-lang.git && \
    cd cairo-lang && \
    git checkout $CAIRO_LANG_REV
RUN cd lib && \
    git clone https://github.com/starknet-io/starknet-token.git && \
    cd starknet-token && \
    git checkout $STARKNET_TOKEN_REV

# Buld
RUN forge build

# Extract
WORKDIR out
COPY build_artifacts/convert.py .
RUN python3 convert.py

# =========================================================================== #
#                                  ARTIFACTS                                  #
# =========================================================================== #

FROM scratch
WORKDIR artifacts

WORKDIR starkgate_4594188

ENV STARKGATE_DIR="/contracts/starkgate-contracts"
ENV ARTIFACTS_SOLID_4594188="$STARKGATE_DIR/artifacts"

COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarkgateManager.json StarkgateManager.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarkgateRegistry.json StarkgateRegistry.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/Proxy.json Proxy_5_0_0.json
COPY --from=starkgate-4594188 $ARTIFACTS_SOLID_4594188/StarknetTokenBridge.json StarknetTokenBridge.json
COPY --from=starkgate-4594188 $STARKGATE_DIR/starkware/solidity/out/ERC20_1.sol/ERC20_1.json ERC20.json

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
ENV ARTIFACTS_CAIRO_LANG="$CAIRO_LANG_DIR/out"

COPY --from=cairo-lang $ARTIFACTS_CAIRO_LANG/Starknet.sol/Starknet.json Starknet.json

WORKDIR ../local_contracts

ENV LOCAL_CONTRACTS_DIR="/contracts/local-contracts"
ENV ARTIFACTS_LOCAL="$LOCAL_CONTRACTS_DIR/out"

COPY --from=local-contracts $ARTIFACTS_LOCAL/StarknetDevCoreContract.sol/Starknet.json StarknetDevCoreContract.json
COPY --from=local-contracts $ARTIFACTS_LOCAL/UnsafeProxy.sol/UnsafeProxy.json UnsafeProxy.json

WORKDIR ../piltover

ENV PILTOVER_DIR="/contracts/piltover"
ENV ARTIFACTS_PILTOVER="$PILTOVER_DIR/target"

COPY --from=piltover $ARTIFACTS_PILTOVER/dev/. .
