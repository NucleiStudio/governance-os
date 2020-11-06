(function() {var implementors = {};
implementors["sp_authorship"] = [{"text":"impl&lt;F, H:&nbsp;Encode + Debug&gt; ProvideInherentData for InherentDataProvider&lt;F, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Fn() -&gt; Vec&lt;H&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sp_consensus_aura"] = [{"text":"impl ProvideInherentData for InherentDataProvider","synthetic":false,"types":[]}];
implementors["sp_finality_tracker"] = [{"text":"impl&lt;F, N:&nbsp;Encode&gt; ProvideInherentData for InherentDataProvider&lt;F, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Fn() -&gt; Result&lt;N, Error&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sp_timestamp"] = [{"text":"impl ProvideInherentData for InherentDataProvider","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()