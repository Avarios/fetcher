CREATE TABLE heatpump (
    event_timestamp TIMESTAMP WITH TIME ZONE PRIMARY KEY NOT NULL,
    Ambient_State varchar(50) NOT NULL,
    Ambient_TemperatureCalculated double precision NOT NULL,
    Boiler_HighTemp double precision NOT NULL,
    Boiler_LowTemp double precision NOT NULL,
    Boiler_MaxTemp double precision NOT NULL,
    Boiler_State varchar(50) NOT NULL,
    Buffer_HighTemp double precision NOT NULL,
    Buffer_LowTemp double precision NOT NULL,
    Buffer_MaxTemp double precision NOT NULL,
    Buffer_State varchar(50) NOT NULL,
    EManager_ActualPower double precision NOT NULL,
    EManager_OperatingState varchar(50) NOT NULL,
    EManager_PVPower double precision NOT NULL,
    EManager_PowerSetpoint double precision NOT NULL,
    HeatingCircuit_1_FlowTemp double precision NOT NULL,
    HeatingCircuit_1_State varchar(50) NOT NULL,
    HeatingCircuit_2_FlowTemp double precision NOT NULL,
    HeatingCircuit_2_State varchar(50) NOT NULL,
    Heatpump_ActualHeatingCapacity double precision NOT NULL,
    Heatpump_CompressorRating double precision NOT NULL,
    Heatpump_CurrentCop double precision NOT NULL,
    Heatpump_ElectricEnergy double precision NOT NULL,
    Heatpump_EnergySourceInletTemp double precision NOT NULL,
    Heatpump_ErrorNumber double precision NOT NULL,
    Heatpump_ErrorState varchar(50) NOT NULL,
    Heatpump_FlowlineTemp double precision NOT NULL,
    Heatpump_HeatEnergy double precision NOT NULL,
    Heatpump_InverterActualPower double precision NOT NULL,
    Heatpump_OperatingState varchar(50) NOT NULL,
    Heatpump_RequestFlowTemp double precision NOT NULL,
    Heatpump_RequestReturnTemp double precision NOT NULL,
    Heatpump_RequestTempDiff double precision NOT NULL,
    Heatpump_RequestType varchar(50) NOT NULL,
    Heatpump_ReturnLineTemp double precision NOT NULL,
    Heatpump_State varchar(50) NOT NULL,
    Heatpump_VolumeSink double precision NOT NULL,
    Heatpump_VolumeSourceFlow double precision NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_eventtime
    ON public.heatpump USING btree
    (event_timestamp ASC NULLS LAST)
    INCLUDE(event_timestamp)
    WITH (deduplicate_items=True)
    TABLESPACE pg_default;

DO
$$
BEGIN
   IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_roles
      WHERE rolname = 'fetcher'
   ) THEN
      CREATE ROLE fetcher LOGIN PASSWORD 'ADDPASSWORDHERE';
   END IF;
END
$$;

GRANT INSERT ON heatpump TO fetcher;
