mod macro_utils;
mod prelude;
pub mod project_rpc;

pub mod book_rpc;
pub mod task_rpc;
use crate::router::RpcRouter;
pub fn all_rpc_router() -> RpcRouter {
	RpcRouter::new()
		.extend(task_rpc::rpc_router())
		.extend(project_rpc::rpc_router())
		.extend(book_rpc::rpc_router())
}
