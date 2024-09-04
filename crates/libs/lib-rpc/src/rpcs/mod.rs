pub mod author_rpc;
pub mod book_rpc;
pub mod category_rpc;
mod macro_utils;
mod prelude;
pub mod project_rpc;
pub mod publisher_rpc;
pub mod task_rpc;
use crate::router::RpcRouter;
pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(task_rpc::rpc_router())
		.extend(project_rpc::rpc_router())
		.extend(author_rpc::rpc_router())
		.extend(book_rpc::rpc_router())
		.extend(category_rpc::rpc_router())
		.extend(publisher_rpc::rpc_router())
}
