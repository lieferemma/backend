# https://github.com/GUI/postgis-docker
FROM nickblah/postgis:12.2-postgis-3

ADD enable-extension-uuid-ossp.sql /docker-entrypoint-initdb.d
ADD enable-extension-postgis.sql /docker-entrypoint-initdb.d
