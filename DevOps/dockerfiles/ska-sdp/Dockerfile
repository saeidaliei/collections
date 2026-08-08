FROM ubuntu:24.04

SHELL ["/bin/bash", "-c"]

WORKDIR /

ARG SPACK_BRANCH=releases/v0.23
ARG SPACK_ROOT=/opt/spack
ARG SPACK_ENV_NAME=sdp

ARG SPACK_CACHE_VERSION=v0.22.0
ARG SPACK_E4S_MIRROR=https://cache.e4s.io/24.11
ARG SPACK_E4S_MIRROR_NAME=E4S

ARG SPACK_ADD_CMD="spack -e ${SPACK_ENV_NAME} add"
ARG SPACK_CONCRETIZE_CMD="spack -e ${SPACK_ENV_NAME} concretize"
ARG SPACK_INSTALL_CMD="spack -e ${SPACK_ENV_NAME} install -j 4 --fail-fast"

COPY packages.deb.txt /opt/packages.deb.txt
RUN DEBIAN_FRONTEND=noninteractive apt-get update -y && apt-get upgrade -y
RUN xargs apt-get install -y < /opt/packages.deb.txt
RUN pipx ensurepath

RUN git clone --depth=2 --branch=$SPACK_BRANCH https://github.com/spack/spack.git $SPACK_ROOT
ENV PATH="/root/.local/bin:${SPACK_ROOT}/bin:${PATH}"
ENV SPACK_ROOT=$SPACK_ROOT
RUN spack compiler find

RUN spack mirror add $SPACK_CACHE_VERSION https://binaries.spack.io/$SPACK_CACHE_VERSION
RUN spack mirror add $SPACK_E4S_MIRROR_NAME $SPACK_E4S_MIRROR
RUN spack buildcache keys --install --trust

RUN git clone https://gitlab.com/ska-telescope/sdp/ska-sdp-spack.git $SPACK_ROOT/var/spack/repos/ska-sdp-spack
RUN spack repo add $SPACK_ROOT/var/spack/repos/ska-sdp-spack
RUN spack env create $SPACK_ENV_NAME

RUN $SPACK_ADD_CMD py-ska-sdp-batch-preprocess
RUN $SPACK_ADD_CMD py-ska-sdp-func-python
RUN $SPACK_ADD_CMD py-ska-sdp-datamodels
RUN $SPACK_ADD_CMD py-ska-sdp-func
RUN $SPACK_ADD_CMD py-ska-telmodel
RUN $SPACK_CONCRETIZE_CMD
RUN $SPACK_INSTALL_CMD
