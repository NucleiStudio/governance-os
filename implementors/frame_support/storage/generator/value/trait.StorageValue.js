(function() {var implementors = {};
implementors["frame_system"] = [{"text":"impl StorageValue&lt;LastRuntimeUpgradeInfo&gt; for LastRuntimeUpgrade","synthetic":false,"types":[]}];
implementors["pallet_aura"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; StorageValue&lt;Vec&lt;&lt;T as Trait&gt;::AuthorityId&gt;&gt; for Authorities&lt;T&gt;","synthetic":false,"types":[]}];
implementors["pallet_timestamp"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; StorageValue&lt;&lt;T as Trait&gt;::Moment&gt; for Now&lt;T&gt;","synthetic":false,"types":[]}];
implementors["pallet_transaction_payment"] = [{"text":"impl StorageValue&lt;FixedU128&gt; for NextFeeMultiplier","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()