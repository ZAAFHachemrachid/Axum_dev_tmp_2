#![allow(unused)] // For example code.

use anyhow::Result;
use serde_json::json;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8090")?;

	// hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo2",
			"pwd": "hello"
		}),
	);
	req_login.await?.print().await?;

	let req_create_project = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_project",
			"params": {
				"data": {
					"name": "project AAA"
				}
			}
		}),
	);
	let result = req_create_project.await?;
	result.print().await?;

	let project_id = result.json_value::<i64>("/result/data/id")?;

	let mut task_ids: Vec<i64> = Vec::new();
	for i in 1..=5 {
		let req_create_task = hc.do_post(
			"/api/rpc",
			json!({
				"id": 1,
				"method": "create_task",
				"params": {
					"data": {
						"project_id": project_id,
						"title": format!("task AAA {i}")
					}
				}
			}),
		);
		let result = req_create_task.await?;
		task_ids.push(result.json_value::<i64>("/result/data/id")?)
	}

	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": task_ids[0], // The first task created.
				"data": {
					"title": "task BB"
				}
			}
		}),
	);
	req_update_task.await?.print().await?;

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": task_ids[1] // The second task created.
			}
		}),
	);
	req_delete_task.await?.print().await?;

	let req_list_all_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks",
			"params": {
				"filters": {
					"project_id": project_id
				},
				"list_options": {
					"order_bys": "!title"
				}
			}
		}),
	);
	req_list_all_tasks.await?.print().await?;

	let req_list_b_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks",
			"params": {
				"filters": [
				{
					"project_id": project_id,
					"title": {"$contains": "BB"},
				},
				// Shows how to use other $in
				{

					"project_id": { "$in": [project_id] },
					"title": {"$in": ["task AAA 3", "task AAA 4"]}
				},
				// This won't match any projects, so, won't return anything.
				{
					"project_id": { "$in": [ 123, 124]},
					"title": {"$in": ["task AAA 2", "task AAA 5"]}
				}
				]
			}
		}),
	);
	req_list_b_tasks.await?.print().await?;
	let req_author_create = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_author",
			"params": {
				"data": {
					"name": "Author Name",
					"bio": "Author Bio"
				}
			}
		}),
	);

	req_author_create.await?.print().await?;
	let req_author_list = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_authors",
			"params": {
				"filters": {
					"name": {"$contains": "Author"}
				}
			}
		}),
	);
	req_author_list.await?.print().await?;
	let req_publisher_create = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_publisher",
			"params": {
				"data": {
					"name": "Publisher Name",
					"address": "Publisher Address"
				}
			}
		}),
	);
	req_publisher_create.await?.print().await?;
	let req_publisher_list = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_publishers",
			"params": {
				"filters": {
					"name": {"$contains": "Publisher"}
				}
			}
		}),
	);

	req_publisher_list.await?.print().await?;
	let req_category_create = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_category",
			"params": {
				"data": {
					"name": "Category Name",
		"description": "Category Type, Category Type 2",
				}
			}
		}),
	);
	req_category_create.await?.print().await?;
	let description = "Category Type, Category Type 2";
	let req_book_create = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_book",
			"params": {
				"data": {
					"title": "Book Title",
					"author_id": 1000,
					"publisher_id": 1000,
					"category_id": 1000,
					"description": "Category Type, Category Type 2",
					"isbn": "123-4567890123",
									"published_date": "2024-09-04T00:00:00Z" // RFC 3339 format with time // RFC 3339 format
				}
			}
		}),
	);
	req_book_create.await?.print().await?;
	let req_book_list = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_books",
			"params": {
				"filters": {
				}
			}
		}),
	);
	req_book_list.await?.print().await?;
	// let title = "Example Book Title";
	// let author_id = 123;
	// let publisher_id = 456;
	// let category_id = 789;
	// let isbn = "123-4567890123";
	// let req_add_book = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"id": 1,
	// 		"method": "create_book",
	// 		"params": {
	// 			"data": {
	// 	"title": title,
	// 			"author_id": author_id,
	// 			"publisher_id": publisher_id,
	// 			"category_id": category_id,
	// 			"isbn": isbn,
	// 			}}
	//
	// 	}),
	// );
	// req_add_book.await?.print().await?;
	//
	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	println!("req_logoff:");
	// req_logoff.await?.print().await?;

	Ok(())
}
