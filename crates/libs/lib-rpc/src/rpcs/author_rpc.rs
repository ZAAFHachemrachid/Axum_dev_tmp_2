use crate::rpcs::prelude::*;
use lib_core::model::author::{
	Author, AuthorBmc, AuthorFilter, AuthorForCreate, AuthorForUpdate,
};
pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_author,
		list_authors,
		update_author,
		delete_author,
	)
}

generate_common_rpc_fns!(
	Bmc: AuthorBmc,
	Entity: Author,
	ForCreate: AuthorForCreate,
	ForUpdate: AuthorForUpdate,
	Filter: AuthorFilter,
	Suffix: author
);
