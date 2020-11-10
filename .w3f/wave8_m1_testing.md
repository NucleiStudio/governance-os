# Milestone 1 Testing

Milestone 1 covered the development of two pallets: `tokens` and `bylaws`; let's see how they can be tested.

## Requirements
You will need a few things first:
1. A running local node, the easiest way to create one is to use this command: `docker run --rm -p 9944:9944 -it eteissonniere/governance-os --dev --tmp --ws-external`.
2. A correctly configured [Polkadot JS UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), you may need the types which you can find [here](../types.json).

## `tokens` pallet
This pallet is used to implement a multi currency system. It maintains compatibility with the `Currency` and `ReservableCurrency` traits, support `Imbalances` and even relies on the system's `AccountData` just like the traditional `pallet_balances`.

### Creating tokens
There are two ways to create tokens: at genesis and on the fly. For creating tokens at genesis please refer to our [example](../node/src/chain_spec.rs). Let's test by creating a new currency on the fly:
1. Go ahead and submit an extrinsic made of:
   1. Call: `tokens.create`
   2. `currency_id` parameter: the runtime defines its own currency ids. In our case we have two `Native` (used for the genesis instantiated currency which is used to pay for fees etc) and a customizable `Custom(u32)`. In our case just select `Custom(0)`
   3. `transferable` parameter: wether the token can be transfered by its holders or not, set it to `Yes` for now.
2. Submit it, you should see an event being logged: `tokens.CurrencyCreated`.

And that's it! We should have created a new currency inside our chain. Under the hood, the pallet also created some bylaw roles to allow our account to "manage" the currency and its token holders to transfer units of currency.

### Using tokens
#### Burning and Minting
Let's grant some tokens to `Bob`:
1. Submit the extrinsic `tokens.mint(Custom(0), Bob, 100)`.
2. An event `tokens.CurrencyMinted` is logged.
3. Checking the chain state `tokens.totalIssuances(Custom(0))` and `system.account(Bob)` should both show a balance of `100` units for our custom currency. Note how we use the `System` pallet and colocated balance informations with other account data.

We can also burn some tokens by using the call `tokens.burn`, try it!

> By default, only the creator (the "owner") of a currency can burn and mint tokens. One could use the `bylaws` pallet to grant other people such an access as well.

#### Exchanging tokens
Let's submit an extrinsic coming from `Bob`:
1. Call: `tokens.transfer(Custom(0), Alice, 50)`.
2. The event `tokens.CurrencyTransferred` should have been logged.
3. Balances of both `Alice` and `Bob` should now be of `50` units for our custom currency.

#### Making tokens non transferable
If `Alice` wants to prevent token holders from making token transfers she could do the following:
1. Submit call `tokens.updateDetails` with the parameters:
   1. `currency_id` set to `Custom(0)`.
   2. Keep `owner` to `Alice`, but you could use this to make somebody else the owner of the token (ownershoip transfer).
   3. `transferable` to `No`.
2. The event `tokens.CurrencyDetailsChanged` should be logged.
3. Try to transfer tokens from `Bob` to `Alice`, you should now have an error.

> `Alice` also happens to have been granted the `Root` role in our genesis block (more on that later), this means that she should still be able to transfer tokens anyways; hence why we are trying with `Bob`.

### Usage with traditional, single currency pallets
In the runtime code, we provide an adapted named `NativeCurrencyAdapter` which can be used to interface a chosen currency with a native substrate pallet (for instance the `staking` one). For example, if we wanted to pass the `Native` currency to the `Currency` field of a specific trait we could simply give it `NativeCurrencyAdapter<Runtime, NativeCurrencyId>`. Please see the Rust documentation for more details 😉.

## `bylaws` pallet
This pallet is used to implement a role based permissioning system. The runtime develop is in charge of creating all the roles they want to expose. In our case we added some in our [example primitives](../primitives/src/lib.rs). We also added a `WeightInfo` inspired pattern for specifying custom pallet roles.

### Role model
A role can be granted to either:
- one or many specific accounts.
- everybody; for instance, when we create a transferable currency we give the associated `TransferCurrency(CurrencyId)` role to everybody.

### Root and management roles
We have also created a few specific roles:
- `Root` which is set via `governance_os_pallet_bylaws::RoleBuilder::root` as the main root role. Anybody with this specific permission can do anything and "bypass" any access control.
- `ManageRoles` which is set via `governance_os_pallet_bylaws::RoleBuilder::manage_roles` which can be used to give sombody the permission to revoke or grant roles. (Also they'd likely be able to grant themselves the `Root` role with this).

### Granting and revoking roles
By default, everybody can create new currencies thanks to the `CreateCurrency` role being granted to everybody in our genesis block. Let's restrict this to `Bob` (remember that `Alice` is root so she can do anything she wants):
1. Revoke the `CreateCurrency` permission being granted to everybody via the extrinsic `bylaws.revokeRole(None, CreateCurrencies)`.
2. The event `bylaws.RoleRevoked` is triggered.
3. Let's grant the role to `Bob` now via `bylaws.grantRole(Some(Bob), CreateCurrencies)`.
4. The event `bylaws.RoleGranted` is triggered.
5. Querying the storage `bylaws.role(Some(Bob))` shows that `Bob` has the role `CreateCurrencies`.

Now, `Alice` (she is root) and `Bob` can create new currencies, however no other account can do it. Feel free to try! For instance you could send some `Native` currency to `Charlie` and see what happens when you try to use her account to create a new currency.

### Usage with other pallets
The `bylaws` pallet implements the trait `RoleManager` which lets other pallets use it to configure bylaws for themselves. Examples of such pallets would be our `tokens` pallet which uses bylaws to manage a currency's ownership and parameters. Another one (which would be simpler) is the [`compat` pallet](../pallets/compat).

### Bonus: `compat` pallet
We wanted to make our solution even more compatible with the standard substrate permissioning model and also demo the capabilities of our system; so we create a pallet to trigger system calls if one has the `Root` role.

We consider a system call any call that would need to be triggered by the `sudo` pallet. We didn't want to rely on `sudo` in our runtime as it would defeat the purpose of having a new role based permissioning model. Instead, consider `compat` an implementation of `sudo` but with our own role system.