--
-- PostgreSQL database dump
--

\restrict WppEjd9NEh7WYUwQSmCLt3axAB43BHlhSFS9Zzf8ZGmhqeEAYOaIpzZ4AIAFZCC

-- Dumped from database version 17.7 (Ubuntu 17.7-0ubuntu0.25.10.1)
-- Dumped by pg_dump version 17.7 (Ubuntu 17.7-0ubuntu0.25.10.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: _sqlx_test; Type: SCHEMA; Schema: -; Owner: fernando
--

CREATE SCHEMA _sqlx_test;


ALTER SCHEMA _sqlx_test OWNER TO fernando;

--
-- Name: pgroll; Type: SCHEMA; Schema: -; Owner: fernando
--

CREATE SCHEMA pgroll;


ALTER SCHEMA pgroll OWNER TO fernando;

--
-- Name: public_15.rename_ask_members; Type: SCHEMA; Schema: -; Owner: fernando
--

CREATE SCHEMA "public_15.rename_ask_members";


ALTER SCHEMA "public_15.rename_ask_members" OWNER TO fernando;

--
-- Name: find_version_schema(name, integer); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.find_version_schema(p_schema_name name, p_depth integer DEFAULT 0) RETURNS text
    LANGUAGE sql STABLE
    AS $$
    WITH RECURSIVE ancestors AS (
        SELECT
            name,
            COALESCE(migration ->> 'version_schema', name) AS version_schema,
            schema,
            parent,
            0 AS depth
        FROM
            "pgroll".migrations
        WHERE
            name = "pgroll".latest_migration (p_schema_name)
            AND SCHEMA = p_schema_name
        UNION ALL
        SELECT
            m.name,
            COALESCE(m.migration ->> 'version_schema', m.name) AS version_schema,
            m.schema,
            m.parent,
            a.depth + 1
        FROM
            "pgroll".migrations m
            JOIN ancestors a ON m.name = a.parent
                AND m.schema = a.schema
)
        SELECT
            a.version_schema
        FROM
            ancestors a
    WHERE
        EXISTS (
            SELECT
                1
            FROM
                information_schema.schemata s
            WHERE
                s.schema_name = p_schema_name || '_' || a.version_schema)
    ORDER BY
        a.depth ASC OFFSET p_depth
    LIMIT 1;
$$;


ALTER FUNCTION pgroll.find_version_schema(p_schema_name name, p_depth integer) OWNER TO fernando;

--
-- Name: is_active_migration_period(name); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.is_active_migration_period(schemaname name) RETURNS boolean
    LANGUAGE sql STABLE
    AS $$
    SELECT
        EXISTS (
            SELECT
                1
            FROM
                "pgroll".migrations
            WHERE
                SCHEMA = schemaname
                AND done = FALSE)
$$;


ALTER FUNCTION pgroll.is_active_migration_period(schemaname name) OWNER TO fernando;

--
-- Name: latest_migration(name); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.latest_migration(schemaname name) RETURNS text
    LANGUAGE sql STABLE SECURITY DEFINER
    SET search_path TO 'pgroll', 'pg_catalog', 'pg_temp'
    AS $$
    SELECT
        p.name
    FROM
        "pgroll".migrations p
    WHERE
        NOT EXISTS (
            SELECT
                1
            FROM
                "pgroll".migrations c
            WHERE
                SCHEMA = schemaname
                AND c.parent = p.name)
        AND SCHEMA = schemaname
$$;


ALTER FUNCTION pgroll.latest_migration(schemaname name) OWNER TO fernando;

--
-- Name: latest_version(name); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.latest_version(schemaname name) RETURNS text
    LANGUAGE sql STABLE
    AS $$
    SELECT
        "pgroll".find_version_schema (schemaname, 0);
$$;


ALTER FUNCTION pgroll.latest_version(schemaname name) OWNER TO fernando;

--
-- Name: previous_migration(name); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.previous_migration(schemaname name) RETURNS text
    LANGUAGE sql
    AS $$
    SELECT
        parent
    FROM
        "pgroll".migrations
    WHERE
        SCHEMA = schemaname
        AND name = "pgroll".latest_migration (schemaname);
$$;


ALTER FUNCTION pgroll.previous_migration(schemaname name) OWNER TO fernando;

--
-- Name: previous_version(name); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.previous_version(schemaname name) RETURNS text
    LANGUAGE sql STABLE
    AS $$
    SELECT
        "pgroll".find_version_schema (schemaname, 1);
$$;


ALTER FUNCTION pgroll.previous_version(schemaname name) OWNER TO fernando;

--
-- Name: raw_migration(); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.raw_migration() RETURNS event_trigger
    LANGUAGE plpgsql SECURITY DEFINER
    SET search_path TO 'pgroll', 'pg_catalog', 'pg_temp'
    AS $$
DECLARE
    schemaname text;
    migration_id text;
BEGIN
    -- Ignore schema changes made by pgroll
    IF (pg_catalog.current_setting('pgroll.no_inferred_migrations', TRUE) = 'TRUE') THEN
        RETURN;
    END IF;
    IF tg_event = 'sql_drop' AND tg_tag = 'DROP SCHEMA' THEN
        -- Take the schema name from the drop schema command
        SELECT
            object_identity INTO schemaname
        FROM
            pg_event_trigger_dropped_objects ();
    ELSIF tg_event = 'sql_drop'
            AND tg_tag != 'ALTER TABLE' THEN
            -- Guess the schema from drop commands
            SELECT
                schema_name INTO schemaname
            FROM
                pg_catalog.pg_event_trigger_dropped_objects ()
            WHERE
                schema_name IS NOT NULL;
    ELSIF tg_event = 'ddl_command_end' THEN
        -- Guess the schema from ddl commands, ignore migrations that touch several schemas
        IF (
            SELECT
                pg_catalog.count(DISTINCT schema_name)
            FROM
                pg_catalog.pg_event_trigger_ddl_commands ()
            WHERE
                schema_name IS NOT NULL) > 1 THEN
            RETURN;
        END IF;
        IF tg_tag = 'CREATE SCHEMA' THEN
            SELECT
                object_identity INTO schemaname
            FROM
                pg_event_trigger_ddl_commands ();
        ELSE
            SELECT
                schema_name INTO schemaname
            FROM
                pg_catalog.pg_event_trigger_ddl_commands ()
            WHERE
                schema_name IS NOT NULL;
        END IF;
    END IF;
    IF schemaname IS NULL THEN
        RETURN;
    END IF;
    -- Ignore migrations done during a migration period
    IF "pgroll".is_active_migration_period (schemaname) THEN
        RETURN;
    END IF;
    -- Remove any duplicate inferred migrations with the same timestamp for this
    -- schema. We assume such migrations are multi-statement batched migrations
    -- and we are only interested in the last one in the batch.
    DELETE FROM "pgroll".migrations
    WHERE SCHEMA = schemaname
        AND created_at = CURRENT_TIMESTAMP
        AND migration_type = 'inferred'
        AND migration -> 'operations' -> 0 -> 'sql' ->> 'up' = current_query();
    -- Someone did a schema change without pgroll, include it in the history
    -- Get the latest non-inferred migration name with microsecond timestamp for ordering
    WITH latest_non_inferred AS (
        SELECT
            name
        FROM
            "pgroll".migrations
        WHERE
            SCHEMA = schemaname
            AND migration_type != 'inferred'
        ORDER BY
            created_at DESC
        LIMIT 1
)
SELECT
    INTO migration_id CASE WHEN EXISTS (
        SELECT
            1
        FROM
            latest_non_inferred) THEN
        pg_catalog.format('%s_%s', (
                SELECT
                    name
                FROM latest_non_inferred), pg_catalog.to_char(pg_catalog.clock_timestamp(), 'YYYYMMDDHH24MISSUS'))
    ELSE
        pg_catalog.format('00000_initial_%s', pg_catalog.to_char(pg_catalog.clock_timestamp(), 'YYYYMMDDHH24MISSUS'))
    END;
    INSERT INTO "pgroll".migrations (schema, name, migration, resulting_schema, done, parent, migration_type, created_at, updated_at)
        VALUES (schemaname, migration_id, pg_catalog.json_build_object('version_schema', 'sql_' || substring(md5(random()::text), 1, 8), 'operations', (
                SELECT
                    pg_catalog.json_agg(pg_catalog.json_build_object('sql', pg_catalog.json_build_object('up', pg_catalog.current_query()))))),
            "pgroll".read_schema (schemaname),
            TRUE,
            "pgroll".latest_migration (schemaname),
            'inferred',
            statement_timestamp(),
            statement_timestamp());
END;
$$;


ALTER FUNCTION pgroll.raw_migration() OWNER TO fernando;

--
-- Name: read_schema(text); Type: FUNCTION; Schema: pgroll; Owner: fernando
--

CREATE FUNCTION pgroll.read_schema(schemaname text) RETURNS jsonb
    LANGUAGE plpgsql
    AS $$
DECLARE
    tables jsonb;
BEGIN
    SELECT
        json_build_object('name', schemaname, 'tables', (
                SELECT
                    COALESCE(json_object_agg(t.relname, jsonb_strip_nulls (jsonb_build_object('name', t.relname, 'oid', t.oid, 'comment', descr.description, 'columns', (
                                        SELECT
                                            json_object_agg(name, c)
                                    FROM (
                                        SELECT
                                            attr.attname AS name, CASE WHEN attr.attgenerated = '' THEN
                                                pg_get_expr(def.adbin, def.adrelid)
                                            ELSE
                                                NULL
                                            END AS default, NOT (attr.attnotnull
                                                OR tp.typtype = 'd'
                                                AND tp.typnotnull) AS nullable, CASE WHEN 'character varying'::regtype = ANY (ARRAY[attr.atttypid, tp.typelem]) THEN
                                            REPLACE(format_type(attr.atttypid, attr.atttypmod), 'character varying', 'varchar')
                                        WHEN 'timestamp with time zone'::regtype = ANY (ARRAY[attr.atttypid, tp.typelem]) THEN
                                            REPLACE(format_type(attr.atttypid, attr.atttypmod), 'timestamp with time zone', 'timestamptz')
                                        ELSE
                                            format_type(attr.atttypid, attr.atttypmod)
                                        END AS type, descr.description AS comment, (EXISTS (
                                                SELECT
                                                    1
                                                FROM pg_constraint
                                            WHERE
                                                conrelid = attr.attrelid
                                                AND ARRAY[attr.attnum::int] @> conkey::int[]
                                                AND contype = 'u')
                                        OR EXISTS (
                                            SELECT
                                                1
                                            FROM pg_index
                                            JOIN pg_class ON pg_class.oid = pg_index.indexrelid
                                        WHERE
                                            indrelid = attr.attrelid
                                            AND indisunique
                                            AND ARRAY[attr.attnum::int] @> pg_index.indkey::int[])) AS unique, (
                                    SELECT
                                        array_agg(e.enumlabel ORDER BY e.enumsortorder)
                                    FROM pg_enum AS e
                                WHERE
                                    e.enumtypid = tp.oid) AS enumValues, CASE WHEN tp.typtype = 'b' THEN
                                    'base'
                                WHEN tp.typtype = 'c' THEN
                                    'composite'
                                WHEN tp.typtype = 'd' THEN
                                    'domain'
                                WHEN tp.typtype = 'e' THEN
                                    'enum'
                                WHEN tp.typtype = 'p' THEN
                                    'pseudo'
                                WHEN tp.typtype = 'r' THEN
                                    'range'
                                WHEN tp.typtype = 'm' THEN
                                    'multirange'
                                END AS postgresType FROM pg_attribute AS attr
                                INNER JOIN pg_type AS tp ON attr.atttypid = tp.oid
                                LEFT JOIN pg_attrdef AS def ON attr.attrelid = def.adrelid
                                    AND attr.attnum = def.adnum
                            LEFT JOIN pg_description AS descr ON attr.attrelid = descr.objoid
                                AND attr.attnum = descr.objsubid
                        WHERE
                            attr.attnum > 0
                            AND NOT attr.attisdropped
                            AND attr.attrelid = t.oid ORDER BY attr.attnum) c), 'primaryKey', (
                        SELECT
                            json_agg(pg_attribute.attname) AS primary_key_columns
                        FROM pg_index, pg_attribute
                    WHERE
                        indrelid = t.oid
                        AND nspname = schemaname
                        AND pg_attribute.attrelid = t.oid
                        AND pg_attribute.attnum = ANY (pg_index.indkey)
                        AND indisprimary), 'indexes', (
                        SELECT
                            json_object_agg(ix_details.name, json_build_object('name', ix_details.name, 'unique', ix_details.indisunique, 'exclusion', ix_details.indisexclusion, 'columns', ix_details.columns, 'predicate', ix_details.predicate, 'method', ix_details.method, 'definition', ix_details.definition))
                    FROM (
                        SELECT
                            replace(reverse(split_part(reverse(pi.indexrelid::regclass::text), '.', 1)), '"', '') AS name, pi.indisunique, pi.indisexclusion, array_agg(a.attname) AS columns, pg_get_expr(pi.indpred, t.oid) AS predicate, am.amname AS method, pg_get_indexdef(pi.indexrelid) AS definition
                        FROM pg_index pi
                        JOIN pg_attribute a ON a.attrelid = pi.indrelid
                            AND a.attnum = ANY (pi.indkey)
                        JOIN pg_class cls ON cls.oid = pi.indexrelid
                        JOIN pg_am am ON am.oid = cls.relam
                        WHERE
                            indrelid = t.oid::regclass GROUP BY pi.indexrelid, pi.indisunique, pi.indpred, am.amname) AS ix_details), 'checkConstraints', (
                SELECT
                    json_object_agg(cc_details.conname, json_build_object('name', cc_details.conname, 'columns', cc_details.columns, 'definition', cc_details.definition, 'noInherit', cc_details.connoinherit))
                FROM (
                    SELECT
                        cc_constraint.conname, array_agg(cc_attr.attname ORDER BY cc_constraint.conkey::int[]) AS columns, pg_get_constraintdef(cc_constraint.oid) AS definition, cc_constraint.connoinherit FROM pg_constraint AS cc_constraint
                    INNER JOIN pg_attribute cc_attr ON cc_attr.attrelid = cc_constraint.conrelid
                        AND cc_attr.attnum = ANY (cc_constraint.conkey)
                    WHERE
                        cc_constraint.conrelid = t.oid
                        AND cc_constraint.contype = 'c' GROUP BY cc_constraint.oid, cc_constraint.conname) AS cc_details), 'uniqueConstraints', (
                    SELECT
                        json_object_agg(uc_details.conname, json_build_object('name', uc_details.conname, 'columns', uc_details.columns))
                    FROM (
                        SELECT
                            uc_constraint.conname, array_agg(uc_attr.attname ORDER BY uc_constraint.conkey::int[]) AS columns, pg_get_constraintdef(uc_constraint.oid) AS definition FROM pg_constraint AS uc_constraint
                        INNER JOIN pg_attribute uc_attr ON uc_attr.attrelid = uc_constraint.conrelid
                            AND uc_attr.attnum = ANY (uc_constraint.conkey)
                        WHERE
                            uc_constraint.conrelid = t.oid
                            AND uc_constraint.contype = 'u' GROUP BY uc_constraint.oid, uc_constraint.conname) AS uc_details), 'excludeConstraints', (
                        SELECT
                            json_object_agg(xc_details.conname, json_build_object('name', xc_details.conname, 'columns', xc_details.columns, 'definition', xc_details.definition, 'predicate', xc_details.predicate, 'method', xc_details.method))
                        FROM (
                            SELECT
                                xc_constraint.conname, array_agg(xc_attr.attname ORDER BY xc_constraint.conkey::int[]) AS columns, pg_get_expr(pi.indpred, t.oid) AS predicate, am.amname AS method, pg_get_constraintdef(xc_constraint.oid) AS definition FROM pg_constraint AS xc_constraint
                            INNER JOIN pg_attribute xc_attr ON xc_attr.attrelid = xc_constraint.conrelid
                                AND xc_attr.attnum = ANY (xc_constraint.conkey)
                            JOIN pg_index pi ON pi.indexrelid = xc_constraint.conindid
                            JOIN pg_class cls ON cls.oid = pi.indexrelid
                            JOIN pg_am am ON am.oid = cls.relam
                            WHERE
                                xc_constraint.conrelid = t.oid
                                AND xc_constraint.contype = 'x' GROUP BY xc_constraint.oid, xc_constraint.conname, pi.indpred, pi.indexrelid, am.amname) AS xc_details), 'foreignKeys', (
                            SELECT
                                json_object_agg(fk_details.conname, json_build_object('name', fk_details.conname, 'columns', fk_details.columns, 'referencedTable', fk_details.referencedTable, 'referencedColumns', fk_details.referencedColumns, 'matchType', fk_details.matchType, 'onDelete', fk_details.onDelete, 'onUpdate', fk_details.onUpdate))
                            FROM (
                                SELECT
                                    fk_info.conname AS conname, fk_info.columns AS columns, fk_info.relname AS referencedTable, array_agg(ref_attr.attname ORDER BY ref_attr.attname) AS referencedColumns, CASE WHEN fk_info.confmatchtype = 'f' THEN
                                    'FULL'
                                WHEN fk_info.confmatchtype = 'p' THEN
                                    'PARTIAL'
                                WHEN fk_info.confmatchtype = 's' THEN
                                    'SIMPLE'
                                END AS matchType, CASE WHEN fk_info.confdeltype = 'a' THEN
                                    'NO ACTION'
                                WHEN fk_info.confdeltype = 'r' THEN
                                    'RESTRICT'
                                WHEN fk_info.confdeltype = 'c' THEN
                                    'CASCADE'
                                WHEN fk_info.confdeltype = 'd' THEN
                                    'SET DEFAULT'
                                WHEN fk_info.confdeltype = 'n' THEN
                                    'SET NULL'
                                END AS onDelete, CASE WHEN fk_info.confupdtype = 'a' THEN
                                    'NO ACTION'
                                WHEN fk_info.confupdtype = 'r' THEN
                                    'RESTRICT'
                                WHEN fk_info.confupdtype = 'c' THEN
                                    'CASCADE'
                                WHEN fk_info.confupdtype = 'd' THEN
                                    'SET DEFAULT'
                                WHEN fk_info.confupdtype = 'n' THEN
                                    'SET NULL'
                                END AS onUpdate FROM (
                                    SELECT
                                        fk_constraint.conname, fk_constraint.conrelid, fk_constraint.confrelid, fk_constraint.confkey, fk_cl.relname, fk_constraint.confmatchtype, fk_constraint.confdeltype, fk_constraint.confupdtype, array_agg(fk_attr.attname ORDER BY fk_attr.attname) AS columns FROM pg_constraint AS fk_constraint
                                    INNER JOIN pg_class fk_cl ON fk_constraint.confrelid = fk_cl.oid -- join the referenced table
                                    INNER JOIN pg_attribute fk_attr ON fk_attr.attrelid = fk_constraint.conrelid
                                        AND fk_attr.attnum = ANY (fk_constraint.conkey) -- join the columns of the referencing table
                                    WHERE
                                        fk_constraint.conrelid = t.oid
                                        AND fk_constraint.contype = 'f' GROUP BY fk_constraint.conrelid, fk_constraint.conname, fk_constraint.confrelid, fk_cl.relname, fk_constraint.confkey, fk_constraint.confmatchtype, fk_constraint.confdeltype, fk_constraint.confupdtype) AS fk_info
                                    INNER JOIN pg_attribute ref_attr ON ref_attr.attrelid = fk_info.confrelid
                                        AND ref_attr.attnum = ANY (fk_info.confkey) -- join the columns of the referenced table
                                GROUP BY fk_info.conname, fk_info.conrelid, fk_info.columns, fk_info.confrelid, fk_info.confmatchtype, fk_info.confdeltype, fk_info.confupdtype, fk_info.relname) AS fk_details)))), '{}'::json)
            FROM pg_class AS t
            INNER JOIN pg_namespace AS ns ON t.relnamespace = ns.oid
            LEFT JOIN pg_description AS descr ON t.oid = descr.objoid
                AND descr.objsubid = 0
            WHERE
                ns.nspname = schemaname
                AND t.relkind IN ('r', 'p') -- tables only (ignores views, materialized views & foreign tables)
)) INTO tables;
    RETURN tables;
END;
$$;


ALTER FUNCTION pgroll.read_schema(schemaname text) OWNER TO fernando;

--
-- Name: database_ids; Type: SEQUENCE; Schema: _sqlx_test; Owner: fernando
--

CREATE SEQUENCE _sqlx_test.database_ids
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE _sqlx_test.database_ids OWNER TO fernando;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: databases; Type: TABLE; Schema: _sqlx_test; Owner: fernando
--

CREATE TABLE _sqlx_test.databases (
    db_name text NOT NULL,
    test_path text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE _sqlx_test.databases OWNER TO fernando;

--
-- Name: migrations; Type: TABLE; Schema: pgroll; Owner: fernando
--

CREATE TABLE pgroll.migrations (
    schema name NOT NULL,
    name text NOT NULL,
    migration jsonb NOT NULL,
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    parent text,
    done boolean DEFAULT false NOT NULL,
    resulting_schema jsonb DEFAULT '{}'::jsonb NOT NULL,
    migration_type character varying(32) DEFAULT 'pgroll'::character varying,
    CONSTRAINT migration_type_check CHECK (((migration_type)::text = ANY ((ARRAY['pgroll'::character varying, 'inferred'::character varying, 'baseline'::character varying])::text[])))
);


ALTER TABLE pgroll.migrations OWNER TO fernando;

--
-- Name: pgroll_version; Type: TABLE; Schema: pgroll; Owner: fernando
--

CREATE TABLE pgroll.pgroll_version (
    version text NOT NULL,
    initialized_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE pgroll.pgroll_version OWNER TO fernando;

--
-- Name: approval; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.approval (
    ask boolean NOT NULL,
    bid boolean NOT NULL,
    candidate uuid NOT NULL
);


ALTER TABLE public.approval OWNER TO fernando;

--
-- Name: ask; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.ask (
    id uuid NOT NULL,
    seller uuid NOT NULL,
    not_below real NOT NULL
);


ALTER TABLE public.ask OWNER TO fernando;

--
-- Name: bid; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.bid (
    id uuid NOT NULL,
    buyer uuid NOT NULL,
    not_above real NOT NULL
);


ALTER TABLE public.bid OWNER TO fernando;

--
-- Name: candidate; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.candidate (
    id uuid NOT NULL,
    ask uuid NOT NULL,
    bid uuid NOT NULL
);


ALTER TABLE public.candidate OWNER TO fernando;

--
-- Name: candidate_archive; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.candidate_archive (
    ask uuid NOT NULL,
    bid uuid NOT NULL
);


ALTER TABLE public.candidate_archive OWNER TO fernando;

--
-- Name: deal; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public.deal (
    id uuid NOT NULL,
    buyer uuid NOT NULL,
    seller uuid NOT NULL,
    price real NOT NULL
);


ALTER TABLE public.deal OWNER TO fernando;

--
-- Name: user; Type: TABLE; Schema: public; Owner: fernando
--

CREATE TABLE public."user" (
    id uuid NOT NULL
);


ALTER TABLE public."user" OWNER TO fernando;

--
-- Name: approval; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".approval WITH (security_invoker='true') AS
 SELECT ask,
    bid,
    candidate
   FROM public.approval;


ALTER VIEW "public_15.rename_ask_members".approval OWNER TO fernando;

--
-- Name: ask; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".ask WITH (security_invoker='true') AS
 SELECT id,
    not_below,
    seller
   FROM public.ask;


ALTER VIEW "public_15.rename_ask_members".ask OWNER TO fernando;

--
-- Name: bid; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".bid WITH (security_invoker='true') AS
 SELECT buyer,
    id,
    not_above
   FROM public.bid;


ALTER VIEW "public_15.rename_ask_members".bid OWNER TO fernando;

--
-- Name: candidate; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".candidate WITH (security_invoker='true') AS
 SELECT id,
    ask,
    bid
   FROM public.candidate;


ALTER VIEW "public_15.rename_ask_members".candidate OWNER TO fernando;

--
-- Name: candidate_archive; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".candidate_archive WITH (security_invoker='true') AS
 SELECT ask,
    bid
   FROM public.candidate_archive;


ALTER VIEW "public_15.rename_ask_members".candidate_archive OWNER TO fernando;

--
-- Name: deal; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members".deal WITH (security_invoker='true') AS
 SELECT seller,
    id,
    buyer,
    price
   FROM public.deal;


ALTER VIEW "public_15.rename_ask_members".deal OWNER TO fernando;

--
-- Name: user; Type: VIEW; Schema: public_15.rename_ask_members; Owner: fernando
--

CREATE VIEW "public_15.rename_ask_members"."user" WITH (security_invoker='true') AS
 SELECT id
   FROM public."user";


ALTER VIEW "public_15.rename_ask_members"."user" OWNER TO fernando;

--
-- Name: databases databases_pkey; Type: CONSTRAINT; Schema: _sqlx_test; Owner: fernando
--

ALTER TABLE ONLY _sqlx_test.databases
    ADD CONSTRAINT databases_pkey PRIMARY KEY (db_name);


--
-- Name: migrations migrations_pkey; Type: CONSTRAINT; Schema: pgroll; Owner: fernando
--

ALTER TABLE ONLY pgroll.migrations
    ADD CONSTRAINT migrations_pkey PRIMARY KEY (schema, name);


--
-- Name: pgroll_version pgroll_version_pkey; Type: CONSTRAINT; Schema: pgroll; Owner: fernando
--

ALTER TABLE ONLY pgroll.pgroll_version
    ADD CONSTRAINT pgroll_version_pkey PRIMARY KEY (version);


--
-- Name: ask ask_pkey; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.ask
    ADD CONSTRAINT ask_pkey PRIMARY KEY (id);


--
-- Name: bid bid_pkey; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.bid
    ADD CONSTRAINT bid_pkey PRIMARY KEY (id);


--
-- Name: candidate candidate_pkey1; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.candidate
    ADD CONSTRAINT candidate_pkey1 PRIMARY KEY (id);


--
-- Name: approval candidate_unique; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.approval
    ADD CONSTRAINT candidate_unique UNIQUE (candidate);


--
-- Name: deal deal_pkey; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.deal
    ADD CONSTRAINT deal_pkey PRIMARY KEY (id);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: databases_created_at; Type: INDEX; Schema: _sqlx_test; Owner: fernando
--

CREATE INDEX databases_created_at ON _sqlx_test.databases USING btree (created_at);


--
-- Name: history_is_linear; Type: INDEX; Schema: pgroll; Owner: fernando
--

CREATE UNIQUE INDEX history_is_linear ON pgroll.migrations USING btree (schema, parent);


--
-- Name: only_first_migration_without_parent; Type: INDEX; Schema: pgroll; Owner: fernando
--

CREATE UNIQUE INDEX only_first_migration_without_parent ON pgroll.migrations USING btree (schema) WHERE (parent IS NULL);


--
-- Name: only_one_active; Type: INDEX; Schema: pgroll; Owner: fernando
--

CREATE UNIQUE INDEX only_one_active ON pgroll.migrations USING btree (schema, name, done) WHERE (done = false);


--
-- Name: idx_ask_price; Type: INDEX; Schema: public; Owner: fernando
--

CREATE INDEX idx_ask_price ON public.ask USING btree (not_below);


--
-- Name: idx_bid_price; Type: INDEX; Schema: public; Owner: fernando
--

CREATE INDEX idx_bid_price ON public.bid USING btree (not_above);


--
-- Name: migrations migrations_schema_parent_fkey; Type: FK CONSTRAINT; Schema: pgroll; Owner: fernando
--

ALTER TABLE ONLY pgroll.migrations
    ADD CONSTRAINT migrations_schema_parent_fkey FOREIGN KEY (schema, parent) REFERENCES pgroll.migrations(schema, name);


--
-- Name: candidate fk_ask_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.candidate
    ADD CONSTRAINT fk_ask_id FOREIGN KEY (ask) REFERENCES public.ask(id);


--
-- Name: candidate_archive fk_ask_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.candidate_archive
    ADD CONSTRAINT fk_ask_id FOREIGN KEY (ask) REFERENCES public.ask(id);


--
-- Name: candidate fk_bid_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.candidate
    ADD CONSTRAINT fk_bid_id FOREIGN KEY (bid) REFERENCES public.bid(id);


--
-- Name: candidate_archive fk_bid_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.candidate_archive
    ADD CONSTRAINT fk_bid_id FOREIGN KEY (bid) REFERENCES public.bid(id);


--
-- Name: deal fk_buyer_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.deal
    ADD CONSTRAINT fk_buyer_id FOREIGN KEY (buyer) REFERENCES public."user"(id);


--
-- Name: approval fk_candidate_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.approval
    ADD CONSTRAINT fk_candidate_id FOREIGN KEY (candidate) REFERENCES public.candidate(id);


--
-- Name: deal fk_seller_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.deal
    ADD CONSTRAINT fk_seller_id FOREIGN KEY (seller) REFERENCES public."user"(id);


--
-- Name: ask fk_user_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.ask
    ADD CONSTRAINT fk_user_id FOREIGN KEY (seller) REFERENCES public."user"(id);


--
-- Name: bid fk_user_id; Type: FK CONSTRAINT; Schema: public; Owner: fernando
--

ALTER TABLE ONLY public.bid
    ADD CONSTRAINT fk_user_id FOREIGN KEY (buyer) REFERENCES public."user"(id);


--
-- Name: pg_roll_handle_ddl; Type: EVENT TRIGGER; Schema: -; Owner: fernando
--

CREATE EVENT TRIGGER pg_roll_handle_ddl ON ddl_command_end
   EXECUTE FUNCTION pgroll.raw_migration();


ALTER EVENT TRIGGER pg_roll_handle_ddl OWNER TO fernando;

--
-- Name: pg_roll_handle_drop; Type: EVENT TRIGGER; Schema: -; Owner: fernando
--

CREATE EVENT TRIGGER pg_roll_handle_drop ON sql_drop
   EXECUTE FUNCTION pgroll.raw_migration();


ALTER EVENT TRIGGER pg_roll_handle_drop OWNER TO fernando;

--
-- PostgreSQL database dump complete
--

\unrestrict WppEjd9NEh7WYUwQSmCLt3axAB43BHlhSFS9Zzf8ZGmhqeEAYOaIpzZ4AIAFZCC

