FROM bitnami/postgresql-repmgr:16

LABEL authors="Andrew Kozmin"

ARG country_name
ARG state_of_province_name
ARG locality_name
ARG organization_name
ARG organizational_unit_name
ARG common_name

RUN openssl req -new -utf8 -x509 -newkey rsa:4096 -noenc -keyout /tmp/key.pem -days 3653 -subj "/C=$country_name/ST=$state_of_province_name/L=$locality_name/O=$organization_name/OU=$organizational_unit_name/CN=$common_name" -out /tmp/cert.crt
