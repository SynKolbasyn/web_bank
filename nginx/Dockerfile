FROM nginx:latest

LABEL authors="Andrew Kozmin"

ARG country_name
ARG state_of_province_name
ARG locality_name
ARG organization_name
ARG organizational_unit_name
ARG common_name

RUN mkdir /etc/nginx/certs/ && \
    openssl req -new -utf8 -x509 -newkey rsa:4096 -noenc -keyout /etc/nginx/certs/key.pem -days 3653 -subj "/C=$country_name/ST=$state_of_province_name/L=$locality_name/O=$organization_name/OU=$organizational_unit_name/CN=$common_name" -out /etc/nginx/certs/cert.crt && \
    rm /etc/nginx/conf.d/default.conf

COPY ./nginx/nginx.conf /etc/nginx/conf.d/default.conf
