/*
 * Copyright 2020 Nuclei Studio OÜ
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    chain_spec,
    cli::{Cli, Subcommand},
    executor::Executor,
    helpers::{core_org, set_default_ss58_version},
    service::{new_full, new_light, new_partial},
};
use governance_os_runtime::Block;
use log::info;
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::{ChainSpec as ServiceChainSpec, PartialComponents};
use sp_api::HashT;
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::BlakeTwo256;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Governance OS Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/NucleiStudio/governance-os/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn ServiceChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "dummy" => Box::new(chain_spec::dummy_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &governance_os_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    set_default_ss58_version();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;

                runner.sync_run(|config| cmd.run::<Block, Executor>(config))
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                    .into())
            }
        }
        Some(Subcommand::GeneratePlcrVotes(args)) => {
            let hashed = BlakeTwo256::hash_of(&(args.power, args.support, args.salt));
            println!("Commit: {:#x}", hashed);
            println!("Reveal: {}, {}, {}", args.power, args.support, args.salt);

            Ok(())
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                {
                    info!(
                        r#"

                     __/>^^^;:,
        __  __      /-.       :,/|/|
       /  \/  \  __/ ^         :,/ \__
      |        |(~             ;/ /  /
      \        / `-'--._       / / ,<  ___
       \      /,__.   /=\     /  _/  >|_'.
        \    /  `_ `--------'    __ / ',\ \
         \  / ,_// ,---_____,   ,_  \_  ,| |
          \/   `--' |=|           \._/ ,/  |
                     \=\            `,,/   |
                      \=\            ||    /
                       \=\____       |\    \
                      / \/    `     <__)    \
                      | |                    |
                    ,__\,\                   /
                   ,--____>    /\.         ./
                   '-__________>  \.______/
                "#
                    );

                    info!(
                        "✨ Core organization account id: {}",
                        core_org().to_ss58check(),
                    );

                    match config.role {
                        Role::Light => new_light(config),
                        _ => new_full(config),
                    }
                    .map_err(sc_cli::Error::Service)
                }
            })
        }
    }
}
