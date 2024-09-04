use crate::rpcs::prelude::*;
use lib_core::model::publisher::{
	Publisher, PublisherBmc, PublisherFilter, PublisherForCreate, PublisherForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_publisher,
		list_publishers,
		update_publisher,
		delete_publisher,
	)
}

generate_common_rpc_fns!(
	Bmc: PublisherBmc,
	Entity: Publisher,
	ForCreate: PublisherForCreate,
	ForUpdate: PublisherForUpdate,
	Filter: PublisherFilter,
	Suffix: publisher
);
