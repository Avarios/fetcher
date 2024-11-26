# Fetcher
> A simple RUST Service that fetches Lambda Heatpump Data from IOBroker (Where the Modbus is in) 
> and Temperature Data from MQTT Devices to store it in influxdb 2.0

## Environment Variables needed:
INFLUX_URL : the URL to the influxDB 2.0 \
INFLUX_AUTH_TOKEN: the token needed to authenticate against influx \
INFLUX_BUCKETNAME: bucketname where the data flows in \
INFLUX_ORG: Org name where the bucket is in \

## To run in docker:

`docker run -e IOBROKER_URL=value
-e INFLUX_URL=value
-e INFLUX_AUTH_TOKEN=value
-e INFLUX_BUCKETNAME=value
-e INFLUX_ORG=value` 


