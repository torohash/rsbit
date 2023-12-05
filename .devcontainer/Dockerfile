FROM rust:1.73.0

ARG USER_NAME

WORKDIR /home/${USER_NAME}/workspace

RUN useradd -u 1000 -m ${USER_NAME} && \
    mkdir -p /home/${USER_NAME}/workspace/app && \
    chown -R ${USER_NAME}:${USER_NAME} /home/${USER_NAME}

USER ${USER_NAME}
