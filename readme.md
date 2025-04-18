# Fetcher
> A simple RUST Service that fetches Lambda Heatpump Data from IOBroker (Where the Modbus is in) 
> and Temperature Data from MQTT Devices to store it into a postgres sql database

## Environment Variables needed:
INFLUX_URL : the URL to the influxDB 2.0 \
POSTGRES_HOST: the token needed to authenticate against influx \
POSTGRES_PORT: bucketname where the data flows in \
POSTGRES_USER: The username for the postgres database \
POSTGRES_PASSWORD: Password for the postgres user \
POSTGRES_DATABASE: Database name \
POSTGRES_TABLENAME: Tabename where the data is stored in  \

## To run in docker:

`docker run -d --name fetcher -e IOBROKER_URL=value
-e POSTGRES_HOST=value
-e POSTGRES_PORT=value
-e POSTGRES_USER=value
-e POSTGRES_PASSWORD=value
-e POSTGRES_DATABASE=value
-e POSTGRES_TABLENAME=value` 


