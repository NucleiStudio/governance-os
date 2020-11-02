// governance_os_pallet_bylaws
pub trait WeightInfo {
	fn grant_role(b: u32, ) -> Weight;
	fn revoke_role(b: u32, ) -> Weight;
}
