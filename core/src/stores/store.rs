use crate::blocks::block::BlockType;
use crate::data_sources::data_source::{
    DataSource, DataSourceConfig, Document, DocumentVersion, SearchFilter,
};
use crate::databases::database::{Database, DatabaseRow, DatabaseTable};
use crate::databases::table_schema::TableSchema;
use crate::dataset::Dataset;
use crate::http::request::{HttpRequest, HttpResponse};
use crate::project::Project;
use crate::providers::embedder::{EmbedderRequest, EmbedderVector};
use crate::providers::llm::{LLMChatGeneration, LLMChatRequest, LLMGeneration, LLMRequest};
use crate::run::{Run, RunStatus, RunType};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Store {
    // Projects
    async fn create_project(&self) -> Result<Project>;
    async fn delete_project(&self, project: &Project) -> Result<()>;

    // Datasets
    async fn latest_dataset_hash(
        &self,
        project: &Project,
        dataset_id: &str,
    ) -> Result<Option<String>>;
    async fn register_dataset(&self, project: &Project, d: &Dataset) -> Result<()>;
    async fn load_dataset(
        &self,
        project: &Project,
        dataset_id: &str,
        hash: &str,
    ) -> Result<Option<Dataset>>;
    async fn list_datasets(&self, project: &Project)
        -> Result<HashMap<String, Vec<(String, u64)>>>;

    // Specifications
    async fn latest_specification_hash(&self, project: &Project) -> Result<Option<String>>;
    async fn register_specification(&self, project: &Project, hash: &str, spec: &str)
        -> Result<()>;
    async fn load_specification(
        &self,
        project: &Project,
        hash: &str,
    ) -> Result<Option<(u64, String)>>;

    // Runs
    async fn latest_run_id(&self, project: &Project, run_type: RunType) -> Result<Option<String>>;
    async fn list_runs(
        &self,
        project: &Project,
        run_type: RunType,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<(Vec<Run>, usize)>;
    async fn load_runs(
        &self,
        project: &Project,
        run_ids: Vec<String>,
    ) -> Result<HashMap<String, Run>>;

    async fn create_run_empty(&self, project: &Project, run: &Run) -> Result<()>;
    async fn update_run_status(
        &self,
        project: &Project,
        run_id: &str,
        run_status: &RunStatus,
    ) -> Result<()>;
    async fn append_run_block(
        &self,
        project: &Project,
        run: &Run,
        block_idx: usize,
        block_type: &BlockType,
        block_name: &String,
    ) -> Result<()>;

    async fn load_run(
        &self,
        project: &Project,
        run_id: &str,
        // None return all, Some(None), return none, Some(Some(_)) return that block.
        block: Option<Option<(BlockType, String)>>,
    ) -> Result<Option<Run>>;
    async fn delete_run(&self, project: &Project, run_id: &str) -> Result<()>;

    // DataSources
    async fn has_data_sources(&self, project: &Project) -> Result<bool>;
    async fn register_data_source(&self, project: &Project, ds: &DataSource) -> Result<()>;
    async fn load_data_source(
        &self,
        project: &Project,
        data_source_id: &str,
    ) -> Result<Option<DataSource>>;
    async fn update_data_source_config(
        &self,
        project: &Project,
        data_source_id: &str,
        config: &DataSourceConfig,
    ) -> Result<()>;
    async fn load_data_source_document(
        &self,
        project: &Project,
        data_source_id: &str,
        document_id: &str,
        version_hash: &Option<String>,
    ) -> Result<Option<Document>>;
    async fn find_data_source_document_ids(
        &self,
        project: &Project,
        data_source_id: &str,
        filter: &Option<SearchFilter>,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<(Vec<String>, usize)>;
    async fn upsert_data_source_document(
        &self,
        project: &Project,
        data_source_id: &str,
        document: &Document,
    ) -> Result<()>;
    async fn update_data_source_document_tags(
        &self,
        project: &Project,
        data_source_id: &str,
        document_id: &str,
        add_tags: &Vec<String>,
        remove_tags: &Vec<String>,
    ) -> Result<Vec<String>>;
    async fn update_data_source_document_parents(
        &self,
        project: &Project,
        data_source_id: &str,
        document_id: &str,
        parents: &Vec<String>,
    ) -> Result<()>;
    async fn list_data_source_document_versions(
        &self,
        project: &Project,
        data_source_id: &str,
        document_id: &str,
        limit_offset: Option<(usize, usize)>,
        latest_hash: &Option<String>,
    ) -> Result<(Vec<DocumentVersion>, usize)>;
    async fn list_data_source_documents(
        &self,
        project: &Project,
        data_source_id: &str,
        limit_offset: Option<(usize, usize)>,
        remove_system_tags: bool,
    ) -> Result<(Vec<Document>, usize)>;
    async fn delete_data_source_document(
        &self,
        project: &Project,
        data_source_id: &str,
        document_id: &str,
    ) -> Result<()>;
    async fn delete_data_source(&self, project: &Project, data_source_id: &str) -> Result<()>;
    // Databases
    async fn register_database(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        name: &str,
    ) -> Result<Database>;
    async fn load_database(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
    ) -> Result<Option<Database>>;
    async fn list_databases(
        &self,
        project: &Project,
        data_source_id: &str,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<Vec<Database>>;
    async fn upsert_database_table(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
        name: &str,
        description: &str,
    ) -> Result<DatabaseTable>;
    async fn update_database_table_schema(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
        schema: &TableSchema,
    ) -> Result<()>;
    async fn load_database_table(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
    ) -> Result<Option<DatabaseTable>>;
    async fn list_databases_tables(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<(Vec<DatabaseTable>, usize)>;
    async fn batch_upsert_database_rows(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
        rows: &Vec<DatabaseRow>,
        truncate: bool,
    ) -> Result<()>;
    async fn load_database_row(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
        row_id: &str,
    ) -> Result<Option<DatabaseRow>>;
    async fn list_database_rows(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
        table_id: &str,
        limit_offset: Option<(usize, usize)>,
    ) -> Result<(Vec<DatabaseRow>, usize)>;
    async fn delete_database(
        &self,
        project: &Project,
        data_source_id: &str,
        database_id: &str,
    ) -> Result<()>;
    // LLM Cache
    async fn llm_cache_get(
        &self,
        project: &Project,
        request: &LLMRequest,
    ) -> Result<Vec<LLMGeneration>>;
    async fn llm_cache_store(
        &self,
        project: &Project,
        request: &LLMRequest,
        generation: &LLMGeneration,
    ) -> Result<()>;

    // LLM Chat Cache
    async fn llm_chat_cache_get(
        &self,
        project: &Project,
        request: &LLMChatRequest,
    ) -> Result<Vec<LLMChatGeneration>>;
    async fn llm_chat_cache_store(
        &self,
        project: &Project,
        request: &LLMChatRequest,
        generation: &LLMChatGeneration,
    ) -> Result<()>;

    // Embedder Cache
    async fn embedder_cache_get(
        &self,
        project: &Project,
        request: &EmbedderRequest,
    ) -> Result<Vec<EmbedderVector>>;
    async fn embedder_cache_store(
        &self,
        project: &Project,
        request: &EmbedderRequest,
        embedding: &EmbedderVector,
    ) -> Result<()>;

    // HTTP Cache
    async fn http_cache_get(
        &self,
        project: &Project,
        request: &HttpRequest,
    ) -> Result<Vec<HttpResponse>>;
    async fn http_cache_store(
        &self,
        project: &Project,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Result<()>;

    // Cloning
    fn clone_box(&self) -> Box<dyn Store + Sync + Send>;
}

impl Clone for Box<dyn Store + Sync + Send> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub const POSTGRES_TABLES: [&'static str; 14] = [
    "-- projects
     CREATE TABLE IF NOT EXISTS projects (
        id BIGSERIAL PRIMARY KEY
    );",
    "-- app specifications
    CREATE TABLE IF NOT EXISTS specifications (
       id                   BIGSERIAL PRIMARY KEY,
       project              BIGINT NOT NULL,
       created              BIGINT NOT NULL,
       hash                 TEXT NOT NULL,
       specification        TEXT NOT NULL,
       FOREIGN KEY(project) REFERENCES projects(id)
    );",
    "-- datasets
    CREATE TABLE IF NOT EXISTS datasets (
       id                   BIGSERIAL PRIMARY KEY,
       project              BIGINT NOT NULL,
       created              BIGINT NOT NULL,
       dataset_id           TEXT NOT NULL,
       hash                 TEXT NOT NULL,
       FOREIGN KEY(project) REFERENCES projects(id)
    );",
    "-- datasets raw hashed data points
    CREATE TABLE IF NOT EXISTS datasets_points (
       id   BIGSERIAL PRIMARY KEY,
       hash TEXT NOT NULL,
       json TEXT NOT NULL
    );",
    "-- datasets to data association (avoid duplication)
    CREATE TABLE IF NOT EXISTS datasets_joins (
       id                   BIGSERIAL PRIMARY KEY,
       dataset              BIGINT NOT NULL,
       point                BIGINT NOT NULL,
       point_idx            BIGINT NOT NULL,
       FOREIGN KEY(dataset) REFERENCES datasets(id),
       FOREIGN KEY(point)   REFERENCES datasets_points(id)
    );",
    "-- runs
    CREATE TABLE IF NOT EXISTS runs (
       id                   BIGSERIAL PRIMARY KEY,
       project              BIGINT NOT NULL,
       created              BIGINT NOT NULL,
       run_id               TEXT NOT NULL,
       run_type             TEXT NOT NULL,
       app_hash             TEXT NOT NULL,
       config_json          TEXT NOT NULL,
       status_json          TEXT NOT NULL,
       FOREIGN KEY(project) REFERENCES projects(id)
    );",
    "-- block executions
    CREATE TABLE IF NOT EXISTS block_executions (
       id        BIGSERIAL PRIMARY KEY,
       hash      TEXT NOT NULL,
       execution TEXT NOT NULL
    );",
    "-- runs to block_executions association (avoid duplication)
    CREATE TABLE IF NOT EXISTS runs_joins (
       id                           BIGSERIAL PRIMARY KEY,
       run                          BIGINT NOT NULL,
       block_idx                    BIGINT NOT NULL,
       block_type                   TEXT NOT NULL,
       block_name                   TEXT NOT NULL,
       input_idx                    BIGINT NOT NULL,
       map_idx                      BIGINT NOT NULL,
       block_execution              BIGINT NOT NULL,
       FOREIGN KEY(run)             REFERENCES runs(id),
       FOREIGN KEY(block_execution) REFERENCES block_executions(id)
    );",
    "-- Cache (non unique hash index)
    CREATE TABLE IF NOT EXISTS cache (
       id                   BIGSERIAL PRIMARY KEY,
       project              BIGINT NOT NULL,
       created              BIGINT NOT NULL,
       hash                 TEXT NOT NULL,
       request              TEXT NOT NULL,
       response             TEXT NOT NULL,
       FOREIGN KEY(project) REFERENCES projects(id)
    );",
    "-- data sources
    CREATE TABLE IF NOT EXISTS data_sources (
       id                   BIGSERIAL PRIMARY KEY,
       project              BIGINT NOT NULL,
       created              BIGINT NOT NULL,
       data_source_id       TEXT NOT NULL,
       internal_id          TEXT NOT NULL,
       config_json          TEXT NOT NULL,
       FOREIGN KEY(project) REFERENCES projects(id)
    );",
    "-- data sources documents
    CREATE TABLE IF NOT EXISTS data_sources_documents (
       id                       BIGSERIAL PRIMARY KEY,
       data_source              BIGINT NOT NULL,
       created                  BIGINT NOT NULL,
       document_id              TEXT NOT NULL,
       timestamp                BIGINT NOT NULL,
       tags_array               TEXT[] NOT NULL,
       parents                  TEXT[] NOT NULL,
       source_url               TEXT,
       hash                     TEXT NOT NULL,
       text_size                BIGINT NOT NULL,
       chunk_count              BIGINT NOT NULL,
       status                   TEXT NOT NULL,
       FOREIGN KEY(data_source) REFERENCES data_sources(id)
    );",
    "-- database
    CREATE TABLE IF NOT EXISTS databases (
       id                   BIGSERIAL PRIMARY KEY,
       created              BIGINT NOT NULL,
       data_source          BIGINT NOT NULL,
       database_id          TEXT NOT NULL, -- unique within data source. Used as the external id.
       name                 TEXT NOT NULL, -- unique within data source
       FOREIGN KEY(data_source) REFERENCES data_sources(id)
    );",
    "-- databases tables
    CREATE TABLE IF NOT EXISTS databases_tables (
       id                   BIGSERIAL PRIMARY KEY,
       created              BIGINT NOT NULL,
       database             BIGINT NOT NULL,
       table_id             TEXT NOT NULL, -- unique within database
       name                 TEXT NOT NULL, -- unique within database
       description          TEXT NOT NULL,
       schema               TEXT, -- json, kept up-to-date automatically with the last insert
       FOREIGN KEY(database) REFERENCES databases(id)
    );",
    "-- databases row
    CREATE TABLE IF NOT EXISTS databases_rows (
       id                   BIGSERIAL PRIMARY KEY,
       created              BIGINT NOT NULL,
       database_table       BIGINT NOT NULL,
       content              TEXT NOT NULL, -- json
       row_id               TEXT NOT NULL, -- unique within table
       FOREIGN KEY(database_table) REFERENCES databases_tables(id)
    );",
];

pub const SQL_INDEXES: [&'static str; 23] = [
    "CREATE INDEX IF NOT EXISTS
       idx_specifications_project_created ON specifications (project, created);",
    "CREATE INDEX IF NOT EXISTS
       idx_specifications_project_hash ON specifications (project, hash);",
    "CREATE INDEX IF NOT EXISTS
       idx_datasets_project_dataset_id_created
       ON datasets (project, dataset_id, created);",
    "CREATE INDEX IF NOT EXISTS
       idx_runs_project_run_type_created ON runs (project, run_type, created);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
       idx_runs_id ON runs (run_id);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
       idx_block_executions_hash ON block_executions (hash);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
       idx_datasets_points_hash ON datasets_points (hash);",
    "CREATE INDEX IF NOT EXISTS
       idx_datasets_joins ON datasets_joins (dataset, point);",
    "CREATE INDEX IF NOT EXISTS
       idx_runs_joins ON runs_joins (run, block_execution);",
    "CREATE INDEX IF NOT EXISTS
       idx_cache_project_hash ON cache (project, hash);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
       idx_data_sources_project_data_source_id ON data_sources (project, data_source_id);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_data_source_document_id
       ON data_sources_documents (data_source, document_id);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_data_source_status_timestamp
       ON data_sources_documents (data_source, status, timestamp);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_data_source_document_id_hash
       ON data_sources_documents (data_source, document_id, hash);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_data_source_document_id_status
       ON data_sources_documents (data_source, document_id, status);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_data_source_document_id_created
       ON data_sources_documents (data_source, document_id, created DESC);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_tags_array ON data_sources_documents USING GIN (tags_array);",
    "CREATE INDEX IF NOT EXISTS
       idx_data_sources_documents_parents_array ON data_sources_documents USING GIN (parents);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
        idx_databases_database_id_data_source ON databases (database_id, data_source);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
        idx_databases_data_source_database_name ON databases (data_source, name);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
        idx_databases_tables_table_id_database ON databases_tables (table_id, database);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
        idx_databases_tables_database_table_name ON databases_tables (database, name);",
    "CREATE UNIQUE INDEX IF NOT EXISTS
        idx_databases_rows_row_id_database_table ON databases_rows (row_id, database_table);",
];

pub const SQL_FUNCTIONS: [&'static str; 3] = [
    // SQL function to delete the project runs / runs_joins / block_executions
    r#"
        CREATE OR REPLACE FUNCTION delete_project_runs(v_project_id BIGINT)
        RETURNS void AS $$
        DECLARE
            run_ids BIGINT[];
            block_exec_ids BIGINT[];
        BEGIN
            -- Store run IDs in an array for the specified project
            SELECT array_agg(id) INTO run_ids FROM runs WHERE project = v_project_id;

            -- Store block_execution IDs in an array
            SELECT array_agg(block_execution) INTO block_exec_ids
            FROM runs_joins
            WHERE run = ANY(run_ids);

            -- Delete from runs_joins where run IDs match those in the project
            DELETE FROM runs_joins WHERE block_execution = ANY(block_exec_ids);

            -- Now delete from block_executions using the stored IDs
            DELETE FROM block_executions WHERE id = ANY(block_exec_ids);

            -- Finally, delete from runs where run IDs match those in the project
            DELETE FROM runs WHERE id = ANY(run_ids);
        END;
        $$ LANGUAGE plpgsql;
    "#,
    // SQL function to delete the project datasets / datasets_joins / datasets_points
    r#"
        CREATE OR REPLACE FUNCTION delete_project_datasets(v_project_id BIGINT)
        RETURNS void AS $$
        DECLARE
            datasets_ids BIGINT[];
            datasets_points_ids BIGINT[];
        BEGIN
            -- Store datasets_ids IDs in an array for the specified project
            SELECT array_agg(id) INTO datasets_ids FROM datasets WHERE project = v_project_id;

            -- Store datasets_points IDs in an array
            SELECT array_agg(point) INTO datasets_points_ids
            FROM datasets_joins
            WHERE dataset = ANY(datasets_ids);

            -- Delete from datasets_joins where point IDs match those in datasets_points
            DELETE FROM datasets_joins WHERE point = ANY(datasets_points_ids);

            -- Now delete from datasets_points using the stored IDs
            DELETE FROM datasets_points WHERE id = ANY(datasets_points_ids);

            -- Finally, delete from datasets where datasets IDs match those in the project
            DELETE FROM datasets WHERE id = ANY(datasets_ids);
        END;
        $$ LANGUAGE plpgsql;
    "#,
    // SQL function to delete a given run + its block_executions / runs_joins
    r#"
        CREATE OR REPLACE FUNCTION delete_run(v_project_id BIGINT, v_run_run_id TEXT)
        RETURNS void AS $$
        DECLARE
            block_exec_ids BIGINT[];
        BEGIN
            -- Store block_execution IDs in an array
            SELECT array_agg(rj.block_execution) INTO block_exec_ids
            FROM runs_joins rj
            JOIN runs r ON rj.run = r.id WHERE r.project = v_project_id AND r.run_id = v_run_run_id;
            -- Delete from runs_joins where run IDs match those in the project
            DELETE FROM runs_joins WHERE block_execution = ANY(block_exec_ids);
            -- Now delete from block_executions using the stored IDs
            DELETE FROM block_executions WHERE id = ANY(block_exec_ids);
            -- Finally, delete from runs where run IDs match those in the project
            DELETE FROM runs WHERE run_id = v_run_run_id;
        END;
        $$ LANGUAGE plpgsql;
    "#,
];
