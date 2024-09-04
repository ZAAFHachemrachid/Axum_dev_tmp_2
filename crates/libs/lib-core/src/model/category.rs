// CREATE TABLE categorie (
//     id SERIAL PRIMARY KEY,
//     name VARCHAR(255) NOT NULL,
//     description TEXT,
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

// region:    --- Category Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Category {
	pub id: i64,
	pub name: String,
	pub description: Option<String>,

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
pub struct CategoryForCreate {
	pub name: String,
	pub description: Option<String>,
}

#[derive(Fields, Deserialize)]
pub struct CategoryForUpdate {
	pub name: Option<String>,
	pub description: Option<String>,
}

#[derive(FilterNodes, Deserialize, Default)]
pub struct CategoryFilter {
	pub id: Option<OpValsInt64>,
	pub name: Option<OpValsString>,
	pub description: Option<OpValsString>,
	pub cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub ctime: Option<OpValsValue>,
	pub mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	pub mtime: Option<OpValsValue>,
}

// endregion: --- Category Types

// region:    --- Category DbBmc
pub struct CategoryBmc {}

impl DbBmc for CategoryBmc {
	const TABLE: &'static str = "categorie";
}

impl CategoryBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		category_c: CategoryForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, category_c).await
	}
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Category> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<CategoryFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Category>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		category_u: CategoryForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, category_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
