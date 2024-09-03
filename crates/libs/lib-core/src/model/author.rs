// CREATE TABLE author (
//     id SERIAL PRIMARY KEY,
//     name VARCHAR(255) NOT NULL,
//     bio TEXT,
//     -- Timestamps
// cid BIGINT NOT NULL,
//     ctime TIMESTAMP WITH TIME ZONE NOT NULL,
//     mid BIGINT NOT NULL,
//         mtime TIMESTAMP WITH TIME ZONE NOT NULL
//
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
// region:    --- Author Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Author {
	pub id: i64,
	pub name: String,
	pub bio: Option<String>,

	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct AuthorForCreate {
	pub name: String,
	pub bio: String,
}

#[derive(Fields, Deserialize)]
pub struct AuthorForUpdate {
	pub name: Option<String>,
	pub bio: String,
}

#[derive(Fields)]
struct AuthorForCreateInner {
	pub name: String,
	pub bio: String,
	pub cid: i64,
	pub ctime: OffsetDateTime,
	pub mid: i64,
	pub mtime: OffsetDateTime,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct AuthorFilter {
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
	bio: Option<OpValsString>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

// endregion: --- Author Types
// region:    --- AuthorBmc
pub struct AuthorBmc;

impl DbBmc for AuthorBmc {
	const TABLE: &'static str = "author";
}

impl AuthorBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		author_c: AuthorForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, author_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Author> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<AuthorFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Author>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		author_u: AuthorForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, author_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// endregion: --- AuthorBmc

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use crate::model::author::AuthorBmc;
	use crate::model::Error;
	// use anyhow::Result;
	// use lib_utils::time::{format_time, now_utc};
	// use serde_json::json;
	use serial_test::serial;
	// use std::time::Duration;
	// use tokio::time::sleep;
	#[serial]
	#[tokio::test]
	async fn test_create_author_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let author_c = AuthorForCreate {
			name: "author1".to_string(),
			bio: "bio1".to_string(),
		};
		let fx_author_id = _dev_utils::seed_author(&ctx, &mm, author_c).await?;
		// -- Create
		// -- Get
		let author = AuthorBmc::get(&ctx, &mm, fx_author_id).await?;
		assert_eq!(author.name, "author1");
		assert_eq!(author.bio, Some("bio1".to_string()));
		// -- Tear-down
		AuthorBmc::delete(&ctx, &mm, fx_author_id).await?;
		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Exec
		let res = AuthorBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound {
					entity: "author",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_all_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let author_c1 = AuthorForCreate {
			name: "author1".to_string(),
			bio: "bio1".to_string(),
		};
		let author_c2 = AuthorForCreate {
			name: "author2".to_string(),
			bio: "bio2".to_string(),
		};
		let id_1 = _dev_utils::seed_author(&ctx, &mm, author_c1).await?;
		let id_2 = _dev_utils::seed_author(&ctx, &mm, author_c2).await?;

		let authors = AuthorBmc::list(&ctx, &mm, None, None).await?;
		assert_eq!(authors.len(), 2, "there should be 2 authors");

		AuthorBmc::delete(&ctx, &mm, id_1).await?;
		AuthorBmc::delete(&ctx, &mm, id_2).await?;

		Ok(())
	}
}
