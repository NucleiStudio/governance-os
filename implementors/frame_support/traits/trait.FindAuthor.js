(function() {var implementors = {};
implementors["frame_support"] = [];
implementors["pallet_aura"] = [{"text":"impl&lt;T:&nbsp;Trait&gt; FindAuthor&lt;u32&gt; for Module&lt;T&gt;","synthetic":false,"types":[]}];
implementors["pallet_session"] = [{"text":"impl&lt;T:&nbsp;Trait, Inner:&nbsp;FindAuthor&lt;u32&gt;&gt; FindAuthor&lt;&lt;T as Trait&gt;::ValidatorId&gt; for FindAccountFromAuthorIndex&lt;T, Inner&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()