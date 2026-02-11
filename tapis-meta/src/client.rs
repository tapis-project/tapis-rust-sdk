use crate::apis::{configuration, Error, aggregation_api, collection_api, db_api, document_api, general_api, index_api, root_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisMeta {
    config: Arc<configuration::Configuration>,
    pub aggregation: AggregationClient,
    pub collection: CollectionClient,
    pub db: DbClient,
    pub document: DocumentClient,
    pub general: GeneralClient,
    pub index: IndexClient,
    pub root: RootClient,
}

impl TapisMeta {
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(Self {
            config: config.clone(),
            aggregation: AggregationClient { config: config.clone() },
            collection: CollectionClient { config: config.clone() },
            db: DbClient { config: config.clone() },
            document: DocumentClient { config: config.clone() },
            general: GeneralClient { config: config.clone() },
            index: IndexClient { config: config.clone() },
            root: RootClient { config: config.clone() },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct AggregationClient {
    config: Arc<configuration::Configuration>,
}

impl AggregationClient {
    pub async fn add_aggregation(&self, db: &str, collection: &str, aggregation: &str, body: Option<serde_json::Value>) -> Result<(), Error<aggregation_api::AddAggregationError>> {
        aggregation_api::add_aggregation(&self.config, db, collection, aggregation, body).await
    }

    pub async fn delete_aggregation(&self, db: &str, collection: &str, aggregation: &str) -> Result<(), Error<aggregation_api::DeleteAggregationError>> {
        aggregation_api::delete_aggregation(&self.config, db, collection, aggregation).await
    }

    pub async fn submit_large_aggregation(&self, db: &str, collection: &str, aggregation: &str, page: Option<i32>, pagesize: Option<i32>, keys: Option<Vec<String>>, body: Option<serde_json::Value>) -> Result<serde_json::Value, Error<aggregation_api::SubmitLargeAggregationError>> {
        aggregation_api::submit_large_aggregation(&self.config, db, collection, aggregation, page, pagesize, keys, body).await
    }

    pub async fn use_aggregation(&self, db: &str, collection: &str, aggregation: &str) -> Result<(), Error<aggregation_api::UseAggregationError>> {
        aggregation_api::use_aggregation(&self.config, db, collection, aggregation).await
    }

}

#[derive(Clone)]
pub struct CollectionClient {
    config: Arc<configuration::Configuration>,
}

impl CollectionClient {
    pub async fn create_collection(&self, db: &str, collection: &str) -> Result<(), Error<collection_api::CreateCollectionError>> {
        collection_api::create_collection(&self.config, db, collection).await
    }

    pub async fn delete_collection(&self, if_match: &str, db: &str, collection: &str) -> Result<(), Error<collection_api::DeleteCollectionError>> {
        collection_api::delete_collection(&self.config, if_match, db, collection).await
    }

    pub async fn get_collection_metadata(&self, db: &str, collection: &str) -> Result<serde_json::Value, Error<collection_api::GetCollectionMetadataError>> {
        collection_api::get_collection_metadata(&self.config, db, collection).await
    }

    pub async fn get_collection_size(&self, db: &str, collection: &str) -> Result<String, Error<collection_api::GetCollectionSizeError>> {
        collection_api::get_collection_size(&self.config, db, collection).await
    }

    pub async fn list_documents(&self, db: &str, collection: &str, page: Option<i32>, pagesize: Option<i32>, filter: Option<serde_json::Value>, sort: Option<serde_json::Value>, keys: Option<Vec<String>>) -> Result<Vec<serde_json::Value>, Error<collection_api::ListDocumentsError>> {
        collection_api::list_documents(&self.config, db, collection, page, pagesize, filter, sort, keys).await
    }

    pub async fn submit_large_query(&self, db: &str, collection: &str, page: Option<i32>, pagesize: Option<i32>, sort: Option<serde_json::Value>, keys: Option<Vec<String>>, body: Option<serde_json::Value>) -> Result<Vec<serde_json::Value>, Error<collection_api::SubmitLargeQueryError>> {
        collection_api::submit_large_query(&self.config, db, collection, page, pagesize, sort, keys, body).await
    }

}

#[derive(Clone)]
pub struct DbClient {
    config: Arc<configuration::Configuration>,
}

impl DbClient {
    pub async fn create_db(&self, db: &str) -> Result<(), Error<db_api::CreateDbError>> {
        db_api::create_db(&self.config, db).await
    }

    pub async fn delete_db(&self, if_match: &str, db: &str) -> Result<(), Error<db_api::DeleteDbError>> {
        db_api::delete_db(&self.config, if_match, db).await
    }

    pub async fn get_db_metadata(&self, db: &str) -> Result<serde_json::Value, Error<db_api::GetDbMetadataError>> {
        db_api::get_db_metadata(&self.config, db).await
    }

    pub async fn list_collection_names(&self, db: &str) -> Result<Vec<String>, Error<db_api::ListCollectionNamesError>> {
        db_api::list_collection_names(&self.config, db).await
    }

}

#[derive(Clone)]
pub struct DocumentClient {
    config: Arc<configuration::Configuration>,
}

impl DocumentClient {
    pub async fn create_document(&self, db: &str, collection: &str, basic: Option<bool>, body: Option<serde_json::Value>) -> Result<serde_json::Value, Error<document_api::CreateDocumentError>> {
        document_api::create_document(&self.config, db, collection, basic, body).await
    }

    pub async fn delete_document(&self, db: &str, collection: &str, doc_id: &str) -> Result<(), Error<document_api::DeleteDocumentError>> {
        document_api::delete_document(&self.config, db, collection, doc_id).await
    }

    pub async fn get_document(&self, db: &str, collection: &str, doc_id: &str) -> Result<serde_json::Value, Error<document_api::GetDocumentError>> {
        document_api::get_document(&self.config, db, collection, doc_id).await
    }

    pub async fn modify_document(&self, db: &str, collection: &str, doc_id: &str, np: Option<bool>, body: Option<serde_json::Value>) -> Result<(), Error<document_api::ModifyDocumentError>> {
        document_api::modify_document(&self.config, db, collection, doc_id, np, body).await
    }

    pub async fn replace_document(&self, db: &str, collection: &str, doc_id: &str, body: Option<serde_json::Value>) -> Result<(), Error<document_api::ReplaceDocumentError>> {
        document_api::replace_document(&self.config, db, collection, doc_id, body).await
    }

}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn healthcheck(&self) -> Result<serde_json::Value, Error<general_api::HealthcheckError>> {
        general_api::healthcheck(&self.config).await
    }

}

#[derive(Clone)]
pub struct IndexClient {
    config: Arc<configuration::Configuration>,
}

impl IndexClient {
    pub async fn create_index(&self, db: &str, collection: &str, index_name: &str, body: Option<serde_json::Value>) -> Result<(), Error<index_api::CreateIndexError>> {
        index_api::create_index(&self.config, db, collection, index_name, body).await
    }

    pub async fn delete_index(&self, db: &str, collection: &str, index_name: &str) -> Result<(), Error<index_api::DeleteIndexError>> {
        index_api::delete_index(&self.config, db, collection, index_name).await
    }

    pub async fn list_indexes(&self, db: &str, collection: &str) -> Result<Vec<serde_json::Value>, Error<index_api::ListIndexesError>> {
        index_api::list_indexes(&self.config, db, collection).await
    }

}

#[derive(Clone)]
pub struct RootClient {
    config: Arc<configuration::Configuration>,
}

impl RootClient {
    pub async fn list_db_names(&self) -> Result<Vec<String>, Error<root_api::ListDbNamesError>> {
        root_api::list_db_names(&self.config).await
    }

}

