### RUST SETUP ###
##################
# start from rust official image
FROM rust:latest

### PYTHON SETUP ###
####################
# update debian packages
RUN apt-get update
# install python, pip and venv
RUN apt-get install -y python3 python3-pip python3-venv
# set up virtual environment pyenv
RUN python3 -m venv /pyenv
# add pyenv binaries to path
ENV PATH="/pyenv/bin:$PATH"
# install packages
RUN pip install --upgrade pip && pip install regex && pip install tiktoken && pip install pytest