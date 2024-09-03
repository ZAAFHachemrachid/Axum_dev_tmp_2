use crate::rpcs::prelude::*;
use lib_core::model::book::{
	Book, BookBmc, BookFilter, BookForCreate, BookForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_book,
		list_books,
		update_book,
		delete_book,
	)
}

generate_common_rpc_fns!(
	Bmc: BookBmc,
	Entity: Book,
	ForCreate: BookForCreate,
	ForUpdate: BookForUpdate,
	Filter: BookFilter,
	Suffix: book
);
