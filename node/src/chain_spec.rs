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

use governance_os_pallet_tokens::CurrencyDetails;
use governance_os_primitives::{AccountId, CurrencyId, Role, Signature};
use governance_os_runtime::{
    AuraConfig, AuraId, BylawsConfig, GenesisConfig, GrandpaConfig, GrandpaId, NativeCurrencyId,
    OrganizationsConfig, SystemConfig, TokensConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    endowed_accounts: Vec<AccountId>,
    currencies: Option<Vec<(CurrencyId, CurrencyDetails<AccountId>)>>,
    roles: Option<Vec<(Role, Option<AccountId>)>>,
) -> GenesisConfig {
    let chain_currencies = currencies.unwrap_or(vec![(
        NativeCurrencyId::get(),
        CurrencyDetails {
            owner: get_account_id_from_seed::<sr25519::Public>("Alice"),
            transferable: true,
        },
    )]);
    let chain_roles = roles.unwrap_or(vec![
        (
            Role::Root,
            Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
        ),
        (Role::CreateCurrencies, None),
        (Role::CreateOrganizations, None),
    ]);

    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        governance_os_pallet_tokens: Some(TokensConfig {
            endowed_accounts: endowed_accounts
                .iter()
                .cloned()
                .map(|account_id| (NativeCurrencyId::get(), account_id, 1 << 60))
                .collect::<Vec<_>>(),
            currency_details: chain_currencies,
        }),
        governance_os_pallet_bylaws: Some(BylawsConfig { roles: chain_roles }),
        governance_os_pallet_organizations: Some(OrganizationsConfig {
            organizations: vec![],
        }),
    }
}

// TODO: we'd like the native currency to be owned by a demo dOrg

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                vec![authority_keys_from_seed("Alice")],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                ],
                None,
                None,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                None,
                None,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    ))
}

/// The dummy chain spec is a chain with no balances, currencies or dOrgs. It is
/// mostly used for benchmarking purposes.
pub fn dummy_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Dummy",
        "dummy",
        ChainType::Custom("dummy".to_string()),
        move || {
            testnet_genesis(
                wasm_binary,
                vec![authority_keys_from_seed("Alice")],
                vec![],
                Some(vec![]),
                None,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    ))
}
