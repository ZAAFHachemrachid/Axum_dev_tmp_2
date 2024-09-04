// CREATE TABLE publisher (
//     id SERIAL PRIMARY KEY,
//     name VARCHAR(255) NOT NULL,
//     address TEXT,
//     -- Timestamps
//  -- Timestamps
//   cid bigint NOT NULL,
//   ctime timestamp with time zone NOT NULL,
//   mid bigint NOT NULL,
//   mtime timestamp with time zone NOT NULL
// );

use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// region:    --- Publisher Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Publisher {
	pub id: i64,
	pub name: String,
	pub address: Option<String>,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct PublisherForCreate {
	pub name: String,
	pub address: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct PublisherForUpdate {
	pub name: Option<String>,
	pub address: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct PublisherFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub address: Option<OpValsString>,
	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

// region:    --- Publisher DbBmc

// region:    --- Publisher DbBmc

pub struct PublisherBmc;

impl DbBmc for PublisherBmc {
	const TABLE: &'static str = "publisher";
}

impl PublisherBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		publisher_c: PublisherForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, publisher_c).await
	}
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Publisher> {
		base::get::<Self, _>(ctx, mm, id).await
	}
	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<PublisherFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Publisher>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		publisher_u: PublisherForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, publisher_u).await
	}
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// Endregion: --- Publisher DbBmc
