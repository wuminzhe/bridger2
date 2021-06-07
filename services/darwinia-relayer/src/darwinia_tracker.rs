use crate::{Result, Error};
use darwinia::Darwinia;
use std::time::Duration;
use substrate_subxt::sp_runtime::generic::Header;
use substrate_subxt::sp_runtime::traits::BlakeTwo256;
use async_std::task;

/// DarwiniaTracker
pub struct DarwiniaBlockTracker {
	darwinia: Darwinia,
	next_block: u32,
}

impl DarwiniaBlockTracker {
	/// new
	pub fn new(darwinia: Darwinia, scan_from: u32) -> Self {
		Self {
			darwinia,
			next_block: scan_from,
		}
	}

	/// get next block
	pub async fn next_block(&mut self) -> Result<Header<u32, BlakeTwo256>> {
		loop {
			match self.get_next_block().await {
				Ok(result) => {
					if let Some(header) = result {
						return Ok(header);
					} else {
						task::sleep(Duration::from_secs(6)).await;
					}
				}
				Err(err) => {
					error!("An error occurred while tracking next darwinia block: {:#?}", err);
					let err_msg = format!("{:?}", err);
					if err_msg.contains("restart") {
						return Err(Error::RestartNeeded);
					} else {
						task::sleep(Duration::from_secs(30)).await;
					}
				}
			}
		}
	}

	async fn get_next_block(&mut self) -> anyhow::Result<Option<Header<u32, BlakeTwo256>>> {
		let finalized_block_hash = self.darwinia.finalized_head().await?;
		match self
			.darwinia
			.get_block_number_by_hash(finalized_block_hash)
			.await?
		{
			Some(finalized_block_number) => {
				if self.next_block > finalized_block_number {
					Ok(None)
				} else {
					let header = self.darwinia.get_block_by_number(self.next_block).await?;
					self.next_block += 1;
					Ok(Some(header))
				}
			}
			None => Ok(None),
		}
	}
}
