(function() {var implementors = {};
implementors["sc_client_api"] = [];
implementors["sc_client_db"] = [{"text":"impl&lt;Block:&nbsp;BlockT&gt; BlockImportOperation&lt;Block&gt; for BlockImportOperation&lt;Block&gt;","synthetic":false,"types":[]}];
implementors["sc_light"] = [{"text":"impl&lt;S, Block&gt; BlockImportOperation&lt;Block&gt; for ImportOperation&lt;Block, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: BlockT,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BlockchainStorage&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block::Hash: Ord,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()