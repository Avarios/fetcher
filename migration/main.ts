import { InfluxDB, FluxTableMetaData } from "@influxdata/influxdb-client";
import { Client } from "pg";
import dotenv from "dotenv";
import fs from "fs";
import path from "path";

dotenv.config();

// InfluxDB configuration
const influxUrl = process.env.INFLUX_URL || "http://adfnas.local:8086";
const influxToken =
  process.env.INFLUX_TOKEN ||
  "YOUR_INFLUX_TOKEN"; // Replace with your InfluxDB token
const influxOrg = process.env.INFLUX_ORG || "adf";
const influxBucket = process.env.INFLUX_BUCKET || "smarthome";

const PG_HOST = process.env.PG_HOST || "adfnas.local";
const PG_PORT = process.env.PG_PORT || 5432;
const PG_USER = process.env.PG_USER || "YOUR_PG_USER"; // Replace with your PostgreSQL user
const PG_PASSWORD = process.env.PG_PASSWORD || "YOUR_PG_PASSWORD"; // Replace with your PostgreSQL password
const PG_DATABASE = process.env.PG_DATABASE || "YOUR_PG_DATABASE"; // Replace with your PostgreSQL database name

const influxClient = new InfluxDB({ url: influxUrl, token: influxToken });
const newline = "\r\n";
const folderPath = "sqlInserts";

async function migrateData() {
  const queryApi = influxClient.getQueryApi(influxOrg);
  const fluxQuery = `
    from(bucket: "${influxBucket}")
  |> range(start: -365d)
  |> filter(fn: (r) => r["_measurement"] == "Heating")
  `;

  const pgClient = new Client({
    host: PG_HOST,
    port: typeof PG_PORT === "string" ? parseInt(PG_PORT, 10) : PG_PORT,
    user: PG_USER,
    password: PG_PASSWORD,
    database: PG_DATABASE,
  });

  await pgClient.connect();

  console.log("Querying InfluxDB...");

  let data: Record<string, Record<string, number | string>> = {};

  for await (const { values, tableMeta } of queryApi.iterateRows(fluxQuery)) {
    const { _time, _value, _field } = tableMeta.toObject(values);

    if (!data[_time]) data[_time] = {};

    data[_time][_field] = _value;

    //console.log(`${_time} FIELD: ${_field} VALUE : ${_value}`);
  }
  console.log("found " + Object.keys(data).length + " records");
  pgClient.query("DROP TABLE IF EXISTS heatpump;");
  pgClient.query("commit;");
  console.log("Data migration started...");

  try {
    for (let key in data) {
      let createTableString = `CREATE TABLE heatpump (event_timestamp TIMESTAMP with time zone PRIMARY KEY NOT NULL,`;
      for (let field in data[key]) {
        if (!isNaN(+data[key][field])) {
          createTableString += `${field} real NOT NULL, `;
        } else {
          createTableString += `${field} varchar(50) NOT NULL,`;
        }
      }
      createTableString = createTableString.substring(
        0,
        createTableString.lastIndexOf(",")
      );
      createTableString += ")";
      console.log(createTableString);

      await pgClient.query(createTableString);
      break;
    }
  } catch (error) {
    console.error(error);
  }

  let dataCounter = 0;
  let dayCounter = 0;
  let lastKey = "";

  if (fs.existsSync(folderPath)) {
    fs.rmSync(folderPath, { recursive: true, force: true });
    console.log("Folder deleted successfully");
  } else {
    console.log("Folder does not exist");
  }
  fs.mkdirSync(folderPath, { recursive: true });
  console.log("Folder created successfully");

  if(fs.existsSync("error.sql")){
    fs.rmSync("error.sql", { recursive: true, force: true }); 
  }

  for (let key in data) {
    let queryString = `INSERT INTO heatpump VALUES ('${new Date(
      key
    ).toISOString()}',`;
    for (let field in data[key]) {
      if (isNaN(+data[key][field])) {
        queryString += `'${data[key][field]}',`;
      } else {
        queryString += `${data[key][field]},`;
      }
    }
    queryString = queryString.substring(0, queryString.lastIndexOf(","));
    queryString += ")";
    fs.appendFileSync(
      path.join(folderPath, `${dayCounter}.sql`),
      queryString + newline
    );

    try {
      await pgClient.query(queryString);
    } catch (error) {
      console.log(error);
      console.error(`Last Date inserted is ${lastKey}`);
      console.error(`Last Query is ${queryString}`);
      fs.appendFileSync("error.sql", queryString + newline);
    }
    

    if (dataCounter % 2880 == 0) {
      console.log(`${new Date().toTimeString()}: Inserted ${dayCounter} days`);
      dayCounter++;
    }
    dataCounter++;
    lastKey = key;
  }

  console.log(`Data migration completed. ${dataCounter} records inserted.`);
}

migrateData()
