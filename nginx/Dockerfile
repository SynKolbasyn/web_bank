FROM nginx:latest

LABEL authors="Andrew Kozmin"

#RUN apt update && \
#    apt upgrade -y && \
#    apt install -y curl gnupg2 ca-certificates lsb-release ubuntu-keyring && \
#    curl https://nginx.org/keys/nginx_signing.key | gpg --dearmor | tee /usr/share/keyrings/nginx-archive-keyring.gpg >/dev/null && \
#    gpg --dry-run --quiet --no-keyring --import --import-options import-show /usr/share/keyrings/nginx-archive-keyring.gpg && \
#    echo "deb [signed-by=/usr/share/keyrings/nginx-archive-keyring.gpg] http://nginx.org/packages/mainline/ubuntu `lsb_release -cs` nginx" | tee /etc/apt/sources.list.d/nginx.list && \
#    apt update && \
#    apt install -y nginx && \
#    echo "" > /etc/nginx/nginx.conf

RUN rm /etc/nginx/conf.d/default.conf
COPY ./nginx/nginx.conf /etc/nginx/conf.d/default.conf

#CMD systemctl start nginx
