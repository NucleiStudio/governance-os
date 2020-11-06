initSidebarItems({"enum":[["BalanceStatus","Status of funds."],["ExistenceRequirement","Simple boolean for whether an account needs to be kept in existence."],["SignedImbalance","Either a positive or a negative imbalance."],["WithdrawReason","Reason for moving funds out of an account."]],"mod":[["schedule",""]],"struct":[["CallMetadata","The function and pallet name of the Call."],["ClearFilterGuard","Guard type for clearing all pushed constraints from a `FilterStack` and reinstating them when dropped."],["FilterStackGuard","Guard type for pushing a constraint to a `FilterStack` and popping when dropped."],["SplitTwoWays","Split an unbalanced amount two ways between a common divisor."],["StorageMapShim","A shim for placing around a storage item in order to use it as a `StoredValue`. Ideally this wouldn't be needed as `StorageValue`s should blanket implement `StoredValue`s, however this would break the ability to have custom impls of `StoredValue`. The other workaround is to implement it directly in the macro."],["TestRandomness","Provides an implementation of [`Randomness`] that should only be used in tests!"],["WithdrawReasons","Reasons for moving funds out of an account."]],"trait":[["ChangeMembers","Trait for type that can handle incremental changes to a set of account IDs."],["Contains","A trait for querying whether a type can be said to \"contain\" a value."],["ContainsLengthBound","A trait for querying bound for the length of an implementation of `Contains`"],["Currency","Abstraction over a fungible assets system."],["EnsureOrigin","Some sort of check on the origin is performed by this object."],["EstimateNextNewSession","Something that can estimate at which block the next `new_session` will be triggered. This must always be implemented by the session module."],["EstimateNextSessionRotation","Something that can estimate at which block the next session rotation will happen. This should be the same logical unit that dictates `ShouldEndSession` to the session module. No Assumptions are made about the scheduling of the sessions."],["Filter","Simple trait for providing a filter over a reference to some type."],["FilterStack","Trait to add a constraint onto the filter."],["FindAuthor","A trait for finding the author of a block header based on the `PreRuntime` digests contained within it."],["Get","A trait for querying a single value from a type."],["GetCallMetadata","Gets the metadata for the Call - function name and pallet name."],["GetCallName","Gets the function name of the Call."],["Happened","A simple, generic one-parameter event notifier/handler."],["Imbalance","A trait for a not-quite Linear Type that tracks an imbalance."],["InitializeMembers","Trait for type that can handle the initialization of account IDs at genesis."],["Instance","An instance of a pallet in the storage."],["InstanceFilter","Simple trait for providing a filter over a reference to some type, given an instance of itself."],["IntegrityTest","Type that provide some integrity tests."],["IsDeadAccount","Determiner to say whether a given account is unused."],["IsType","Trait to be used when types are exactly same."],["KeyOwnerProofSystem","Something which can compute and check proofs of a historical key owner and return full identification data of that key owner."],["Lateness","Trait to be used by block producing consensus engine modules to determine how late the current block is (e.g. in a slot-based proposal mechanism how many slots were skipped since the previous block)."],["Len","Anything that can have a `::len()` method."],["LockableCurrency","A currency whose accounts can have liquidity restrictions."],["OffchainWorker","Off-chain computation trait."],["OnFinalize","The block finalization trait. Implementing this lets you express what should happen for your module when the block is ending."],["OnInitialize","The block initialization trait. Implementing this lets you express what should happen for your module when the block is beginning (right before the first extrinsic is executed)."],["OnKilledAccount","The account with the given id was reaped."],["OnNewAccount","Handler for when a new account has been created."],["OnRuntimeUpgrade","The runtime upgrade trait."],["OnUnbalanced","Handler for when some currency \"account\" decreased in balance for some reason."],["OriginTrait","Methods available on `frame_system::Trait::Origin`."],["PalletInfo","Provides information about the pallet setup in the runtime."],["Randomness",""],["ReservableCurrency","A currency where funds can be reserved from the user."],["StoredMap","An abstraction of a value stored within storage, but possibly as part of a larger composite item."],["Time",""],["TryDrop","A type for which some values make sense to be able to drop without further consideration."],["UnfilteredDispatchable","Type that can be dispatched with an origin but without checking the origin filter."],["UnixTime","Trait to deal with unix time."],["ValidatorRegistration","Implementors of this trait provide information about whether or not some validator has been registered with them. The Session module is an implementor."],["VerifySeal","A trait for verifying the seal of a header and returning the author."],["VestingSchedule","A vesting schedule over a currency. This allows a particular currency to have vesting limits applied to it."]],"type":[["LockIdentifier","An identifier for a lock. Used for disambiguating different locks so that they can be individually replaced or removed."]]});