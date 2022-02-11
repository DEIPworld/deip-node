pub mod assets;
pub mod deip;
pub mod deip_dao;
pub mod deip_proposal;
#[cfg(feature = "octopus")]
pub mod octopus_appchain;
#[cfg(feature = "octopus")]
pub mod octopus_lpos;

// impl From<crate::runtime_api::api::runtime_types::appchain_deip_runtime::Call>
//     for node_template_runtime::Call
// {
//     fn from(call: crate::runtime_api::api::runtime_types::appchain_deip_runtime::Call) -> Self {
//         todo!()
//     }
// }
