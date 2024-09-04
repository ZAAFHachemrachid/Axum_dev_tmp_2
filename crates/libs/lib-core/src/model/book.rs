// CREATE TABLE Books (
//     id SERIAL PRIMARY KEY,
//     title VARCHAR(255) NOT NULL,
//     author_id INT REFERENCES Authors(id),
//     publisher_id INT REFERENCES Publishers(id),
//     category_id INT REFERENCES Categories(id),
//     published_date DATE,
//     isbn VARCHAR(13) UNIQUE
//     cid bigint NOT NULL,
//   ctime timestamp with time zone NOT NULL,
//   mid bigint NOT NULL,
//   mtime timestamp with time zone NOT NULL
// );
// bmc for Books
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
use time::format_description::FormatItem;
use time::serde::format_description;

// region:    --- Book Types
#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Book {
	pub id: i64,
	pub title: String,
	pub author_id: i64,
	pub publisher_id: i64,
	pub category_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub published_date: OffsetDateTime,
	pub isbn: String,
	pub description: String,
	// -- Timestamps
	//    (creator and last modified user_id/time)
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}
#[serde_as]
#[derive(Fields, Deserialize)]
pub struct BookForCreate {
	pub title: String,
	pub author_id: i64,
	pub publisher_id: i64,
	pub category_id: i64,
	pub description: String,
	#[serde_as(as = "Rfc3339")]
	pub published_date: OffsetDateTime,
	pub isbn: String,
}
#[serde_as]
#[derive(Fields, Deserialize)]
pub struct BookForUpdate {
	pub title: Option<String>,
	pub author_id: Option<i64>,
	pub publisher_id: Option<i64>,
	pub category_id: Option<i64>,

	pub description: String,
	#[serde_as(as = "Rfc3339")]
	pub published_date: OffsetDateTime,
	pub isbn: Option<String>,
}

#[derive(Fields)]
struct BookForCreateInner {
	pub title: String,
	pub author_id: i64,
	pub publisher_id: i64,
	pub category_id: i64,
	pub description_o: String,

	pub published_date: String,
	pub isbn: String,
}
#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct BookFilter {
	id: Option<OpValsInt64>,
	title: Option<OpValsString>,
	author_id: Option<OpValsInt64>,
	publisher_id: Option<OpValsInt64>,
	category_id: Option<OpValsInt64>,
	description: Option<OpValsString>,

	#[modql(to_sea_value_fn = "time_to_sea_value")]
	published_date: Option<OpValsValue>,
	isbn: Option<OpValsString>,

	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

// endregion: --- Book Types
// region:    --- BookBmc
pub struct BookBmc {}

impl DbBmc for BookBmc {
	const TABLE: &'static str = "book";
}

impl BookBmc {
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		book_c: BookForCreate,
	) -> Result<i64> {
		base::create::<Self, _>(ctx, mm, book_c).await
	}

	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Book> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filter: Option<Vec<BookFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Book>> {
		base::list::<Self, _, _>(ctx, mm, filter, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		book_u: BookForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, book_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
