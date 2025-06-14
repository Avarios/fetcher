import { InfluxDB, Point } from '@influxdata/influxdb-client';
import { Client } from 'pg';
import * as dotenv from 'dotenv';

// Load environment variables
dotenv.config();

interface HeatpumpData {
  event_timestamp: Date;
  Ambient_State: string;
  Ambient_TemperatureCalculated: number;
  Boiler_HighTemp: number;
  Boiler_LowTemp: number;
  Boiler_MaxTemp: number;
  Boiler_State: string;
  Buffer_HighTemp: number;
  Buffer_LowTemp: number;
  Buffer_MaxTemp: number;
  Buffer_State: string;
  HeatingCircuit_1_FlowTemp: number;
  HeatingCircuit_1_State: string;
  HeatingCircuit_2_FlowTemp: number;
  HeatingCircuit_2_State: string;
  Heatpump_ActualHeatingCapacity: number;
  Heatpump_CompressorRating: number;
  Heatpump_CurrentCop: number;
  Heatpump_ElectricEnergy: number;
  Heatpump_EnergySourceInletTemp: number;
  Heatpump_ErrorNumber: number;
  Heatpump_ErrorState: string;
  Heatpump_FlowlineTemp: number;
  Heatpump_HeatEnergy: number;
  Heatpump_InverterActualPower: number;
  Heatpump_OperatingState: string;
  Heatpump_RequestFlowTemp: number;
  Heatpump_RequestReturnTemp: number;
  Heatpump_RequestTempDiff: number;
  Heatpump_RequestType: string;
  Heatpump_ReturnLineTemp: number;
  Heatpump_State: string;
  Heatpump_VolumeSink: number;
  Heatpump_VolumeSourceFlow: number;
}

class InfluxToPostgresSync {
  private influxClient: InfluxDB;
  private pgClient: Client;
  private queryApi: any;

  constructor() {
    this.influxClient = new InfluxDB({
      url: process.env.INFLUXDB_URL || 'http://adfnas.local:8086',
      token: process.env.INFLUXDB_TOKEN || '',
      timeout: 1000000
    });

    this.queryApi = this.influxClient.getQueryApi(process.env.INFLUXDB_ORG || 'adf');

    this.pgClient = new Client({
      host: process.env.POSTGRES_HOST || 'adfnas.local',
      port: parseInt(process.env.POSTGRES_PORT || '5432'),
      user: process.env.POSTGRES_USER || 'smarthome',
      password: process.env.POSTGRES_PASSWORD || '',
      database: process.env.POSTGRES_DATABASE || '',
    });
  }

  async connect(): Promise<void> {
    try {
      await this.pgClient.connect();
      console.log('Connected to PostgreSQL');
    } catch (error) {
      console.error('Failed to connect to PostgreSQL:', error);
      throw error;
    }
  }

  async disconnect(): Promise<void> {
    await this.pgClient.end();
    console.log('Disconnected from databases');
  }

  async getLastTimestamp(): Promise<Date | null> {
    try {
      const result = await this.pgClient.query(
        'SELECT MAX(event_timestamp) as last_timestamp FROM heatpump'
      );
      return result.rows[0].last_timestamp || null;
    } catch (error) {
      console.error('Error getting last timestamp:', error);
      return null;
    }
  }

  async fetchInfluxData(startTime?: Date): Promise<HeatpumpData[]> {
    const timeFilter = `|> range(start: -365d)`; // Default to last hour if no start time

    const fluxQuery = `
      from(bucket: "smarthome")
        ${timeFilter}
        |> filter(fn: (r) => r["_measurement"] == "Heating")
        |> pivot(rowKey:["_time"], columnKey: ["_field"], valueColumn: "_value")
        |> sort(columns: ["_time"])
    `;

    console.log('Executing InfluxDB query...');
    
    const data: HeatpumpData[] = [];
    
    return new Promise((resolve, reject) => {
      this.queryApi.queryRows(fluxQuery, {
        next(row: string[], tableMeta: any) {
          const o = tableMeta.toObject(row);
          console.log('Processing row:', o);
          const record: HeatpumpData = {
            event_timestamp: new Date(o._time),
            Ambient_State: o.Ambient_State || '',
            Ambient_TemperatureCalculated: parseFloat(o.Ambient_TemperatureCalculated) || 0,
            Boiler_HighTemp: parseFloat(o.Boiler_HighTemp) || 0,
            Boiler_LowTemp: parseFloat(o.Boiler_LowTemp) || 0,
            Boiler_MaxTemp: parseFloat(o.Boiler_MaxTemp) || 0,
            Boiler_State: o.Boiler_State || '',
            Buffer_HighTemp: parseFloat(o.Buffer_HighTemp) || 0,
            Buffer_LowTemp: parseFloat(o.Buffer_LowTemp) || 0,
            Buffer_MaxTemp: parseFloat(o.Buffer_MaxTemp) || 0,
            Buffer_State: o.Buffer_State || '',
            HeatingCircuit_1_FlowTemp: parseFloat(o.HeatingCircuit_1_FlowTemp) || 0,
            HeatingCircuit_1_State: o.HeatingCircuit_1_State || '',
            HeatingCircuit_2_FlowTemp: parseFloat(o.HeatingCircuit_2_FlowTemp) || 0,
            HeatingCircuit_2_State: o.HeatingCircuit_2_State || '',
            Heatpump_ActualHeatingCapacity: parseFloat(o.Heatpump_ActualHeatingCapacity) || 0,
            Heatpump_CompressorRating: parseFloat(o.Heatpump_CompressorRating) || 0,
            Heatpump_CurrentCop: parseFloat(o.Heatpump_CurrentCop) || 0,
            Heatpump_ElectricEnergy: parseFloat(o.Heatpump_ElectricEnergy) || 0,
            Heatpump_EnergySourceInletTemp: parseFloat(o.Heatpump_EnergySourceInletTemp) || 0,
            Heatpump_ErrorNumber: parseFloat(o.Heatpump_ErrorNumber) || 0,
            Heatpump_ErrorState: o.Heatpump_ErrorState || '',
            Heatpump_FlowlineTemp: parseFloat(o.Heatpump_FlowlineTemp) || 0,
            Heatpump_HeatEnergy: parseFloat(o.Heatpump_HeatEnergy) || 0,
            Heatpump_InverterActualPower: parseFloat(o.Heatpump_InverterActualPower) || 0,
            Heatpump_OperatingState: o.Heatpump_OperatingState || '',
            Heatpump_RequestFlowTemp: parseFloat(o.Heatpump_RequestFlowTemp) || 0,
            Heatpump_RequestReturnTemp: parseFloat(o.Heatpump_RequestReturnTemp) || 0,
            Heatpump_RequestTempDiff: parseFloat(o.Heatpump_RequestTempDiff) || 0,
            Heatpump_RequestType: o.Heatpump_RequestType || '',
            Heatpump_ReturnLineTemp: parseFloat(o.Heatpump_ReturnLineTemp) || 0,
            Heatpump_State: o.Heatpump_State || '',
            Heatpump_VolumeSink: parseFloat(o.Heatpump_VolumeSink) || 0,
            Heatpump_VolumeSourceFlow: parseFloat(o.Heatpump_VolumeSourceFlow) || 0,
          };
          
          data.push(record);
        },
        error(error: Error) {
          console.error('InfluxDB query error:', error);
          reject(error);
        },
        complete() {
          console.log(`Fetched ${data.length} records from InfluxDB`);
          resolve(data);
        },
      });
    });
  }

  async insertDataToPostgres(data: HeatpumpData[]): Promise<void> {
    if (data.length === 0) {
      console.log('No data to insert');
      return;
    }

    const insertQuery = `
      INSERT INTO heatpump (
        event_timestamp, Ambient_State, Ambient_TemperatureCalculated,
        Boiler_HighTemp, Boiler_LowTemp, Boiler_MaxTemp, Boiler_State,
        Buffer_HighTemp, Buffer_LowTemp, Buffer_MaxTemp, Buffer_State,
        HeatingCircuit_1_FlowTemp, HeatingCircuit_1_State,
        HeatingCircuit_2_FlowTemp, HeatingCircuit_2_State,
        Heatpump_ActualHeatingCapacity, Heatpump_CompressorRating,
        Heatpump_CurrentCop, Heatpump_ElectricEnergy,
        Heatpump_EnergySourceInletTemp, Heatpump_ErrorNumber,
        Heatpump_ErrorState, Heatpump_FlowlineTemp, Heatpump_HeatEnergy,
        Heatpump_InverterActualPower, Heatpump_OperatingState,
        Heatpump_RequestFlowTemp, Heatpump_RequestReturnTemp,
        Heatpump_RequestTempDiff, Heatpump_RequestType,
        Heatpump_ReturnLineTemp, Heatpump_State, Heatpump_VolumeSink,
        Heatpump_VolumeSourceFlow
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
              $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28,
              $29, $30, $31, $32, $33, $34)
      ON CONFLICT (event_timestamp) DO UPDATE SET
        Ambient_State = EXCLUDED.Ambient_State,
        Ambient_TemperatureCalculated = EXCLUDED.Ambient_TemperatureCalculated,
        Boiler_HighTemp = EXCLUDED.Boiler_HighTemp,
        Boiler_LowTemp = EXCLUDED.Boiler_LowTemp,
        Boiler_MaxTemp = EXCLUDED.Boiler_MaxTemp,
        Boiler_State = EXCLUDED.Boiler_State,
        Buffer_HighTemp = EXCLUDED.Buffer_HighTemp,
        Buffer_LowTemp = EXCLUDED.Buffer_LowTemp,
        Buffer_MaxTemp = EXCLUDED.Buffer_MaxTemp,
        Buffer_State = EXCLUDED.Buffer_State,
        HeatingCircuit_1_FlowTemp = EXCLUDED.HeatingCircuit_1_FlowTemp,
        HeatingCircuit_1_State = EXCLUDED.HeatingCircuit_1_State,
        HeatingCircuit_2_FlowTemp = EXCLUDED.HeatingCircuit_2_FlowTemp,
        HeatingCircuit_2_State = EXCLUDED.HeatingCircuit_2_State,
        Heatpump_ActualHeatingCapacity = EXCLUDED.Heatpump_ActualHeatingCapacity,
        Heatpump_CompressorRating = EXCLUDED.Heatpump_CompressorRating,
        Heatpump_CurrentCop = EXCLUDED.Heatpump_CurrentCop,
        Heatpump_ElectricEnergy = EXCLUDED.Heatpump_ElectricEnergy,
        Heatpump_EnergySourceInletTemp = EXCLUDED.Heatpump_EnergySourceInletTemp,
        Heatpump_ErrorNumber = EXCLUDED.Heatpump_ErrorNumber,
        Heatpump_ErrorState = EXCLUDED.Heatpump_ErrorState,
        Heatpump_FlowlineTemp = EXCLUDED.Heatpump_FlowlineTemp,
        Heatpump_HeatEnergy = EXCLUDED.Heatpump_HeatEnergy,
        Heatpump_InverterActualPower = EXCLUDED.Heatpump_InverterActualPower,
        Heatpump_OperatingState = EXCLUDED.Heatpump_OperatingState,
        Heatpump_RequestFlowTemp = EXCLUDED.Heatpump_RequestFlowTemp,
        Heatpump_RequestReturnTemp = EXCLUDED.Heatpump_RequestReturnTemp,
        Heatpump_RequestTempDiff = EXCLUDED.Heatpump_RequestTempDiff,
        Heatpump_RequestType = EXCLUDED.Heatpump_RequestType,
        Heatpump_ReturnLineTemp = EXCLUDED.Heatpump_ReturnLineTemp,
        Heatpump_State = EXCLUDED.Heatpump_State,
        Heatpump_VolumeSink = EXCLUDED.Heatpump_VolumeSink,
        Heatpump_VolumeSourceFlow = EXCLUDED.Heatpump_VolumeSourceFlow
    `;

    let insertedCount = 0;
    let updatedCount = 0;

    try {
      await this.pgClient.query('BEGIN');

      for (const record of data) {
        const values = [
          record.event_timestamp,
          record.Ambient_State,
          record.Ambient_TemperatureCalculated,
          record.Boiler_HighTemp,
          record.Boiler_LowTemp,
          record.Boiler_MaxTemp,
          record.Boiler_State,
          record.Buffer_HighTemp,
          record.Buffer_LowTemp,
          record.Buffer_MaxTemp,
          record.Buffer_State,
          record.HeatingCircuit_1_FlowTemp,
          record.HeatingCircuit_1_State,
          record.HeatingCircuit_2_FlowTemp,
          record.HeatingCircuit_2_State,
          record.Heatpump_ActualHeatingCapacity,
          record.Heatpump_CompressorRating,
          record.Heatpump_CurrentCop,
          record.Heatpump_ElectricEnergy,
          record.Heatpump_EnergySourceInletTemp,
          record.Heatpump_ErrorNumber,
          record.Heatpump_ErrorState,
          record.Heatpump_FlowlineTemp,
          record.Heatpump_HeatEnergy,
          record.Heatpump_InverterActualPower,
          record.Heatpump_OperatingState,
          record.Heatpump_RequestFlowTemp,
          record.Heatpump_RequestReturnTemp,
          record.Heatpump_RequestTempDiff,
          record.Heatpump_RequestType,
          record.Heatpump_ReturnLineTemp,
          record.Heatpump_State,
          record.Heatpump_VolumeSink,
          record.Heatpump_VolumeSourceFlow,
        ];
        console.log(`Inserting record with timestamp: ${record.event_timestamp}`);
        await this.pgClient.query(insertQuery, values);
        insertedCount++;
        console.log(`Inserted record: ${record.event_timestamp}`);
      }

      await this.pgClient.query('COMMIT');
      console.log(`Successfully processed ${insertedCount} records`);
    } catch (error) {
      await this.pgClient.query('ROLLBACK');
      console.error('Error inserting data:', error);
      throw error;
    }
  }

  async syncData(): Promise<void> {
    try {
      console.log('Starting data synchronization...');
      
      // Get the last timestamp from PostgreSQL
      const lastTimestamp = await this.getLastTimestamp();
      console.log('Last timestamp in PostgreSQL:', lastTimestamp);

      // Fetch data from InfluxDB
      const influxData = await this.fetchInfluxData(lastTimestamp ?? undefined);

      if (influxData.length === 0) {
        console.log('No new data to sync');
        return;
      }

      // Insert data into PostgreSQL
      await this.insertDataToPostgres(influxData);

      console.log('Data synchronization completed successfully');
    } catch (error) {
      console.error('Error during data synchronization:', error);
      throw error;
    }
  }

  async runContinuousSync(intervalMinutes: number = 5): Promise<void> {
    console.log(`Starting continuous sync with ${intervalMinutes} minute intervals`);
    
    const sync = async () => {
      try {
        await this.syncData();
      } catch (error) {
        console.error('Sync error:', error);
      }
    };

    // Run initial sync
    await sync();

    // Set up interval for continuous syncing
    setInterval(sync, intervalMinutes * 60 * 1000);
  }
}

// Main execution
async function main() {
  const syncer = new InfluxToPostgresSync();

  try {
    await syncer.connect();
    
    // Check if we want continuous sync or one-time sync
    const continuous = process.env.CONTINUOUS_SYNC === 'true';
    const intervalMinutes = parseInt(process.env.SYNC_INTERVAL_MINUTES || '5');

    if (continuous) {
      await syncer.runContinuousSync(intervalMinutes);
    } else {
      await syncer.syncData();
      await syncer.disconnect();
    }
  } catch (error) {
    console.error('Application error:', error);
    await syncer.disconnect();
    process.exit(1);
  }
}

// Handle graceful shutdown
process.on('SIGINT', async () => {
  console.log('Received SIGINT, shutting down gracefully...');
  process.exit(0);
});

process.on('SIGTERM', async () => {
  console.log('Received SIGTERM, shutting down gracefully...');
  process.exit(0);
});

if (require.main === module) {
  main();
}

export { InfluxToPostgresSync, HeatpumpData };