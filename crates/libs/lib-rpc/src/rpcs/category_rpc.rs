use crate::rpcs::prelude::*;
use lib_core::model::category::{
	Category, CategoryBmc, CategoryFilter, CategoryForCreate, CategoryForUpdate,
};

pub fn rpc_router() -> RpcRouter {
	rpc_router!(
		// Same as RpcRouter::new().add...
		create_category,
		list_categorys,
		update_category,
		delete_category,
	)
}

generate_common_rpc_fns!(
	Bmc: CategoryBmc,
	Entity: Category,
	ForCreate: CategoryForCreate,
	ForUpdate: CategoryForUpdate,
	Filter: CategoryFilter,
	Suffix: category
);
