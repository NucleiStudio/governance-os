(function() {var implementors = {};
implementors["sc_transaction_pool"] = [{"text":"impl&lt;Client, Block&gt; ChainApi for FullChainApi&lt;Client, Block&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: BlockT,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: ProvideRuntimeApi&lt;Block&gt; + BlockBackend&lt;Block&gt; + BlockIdTo&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client::Api: TaggedTransactionQueue&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ApiErrorFor&lt;Client, Block&gt;: Send + Display,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Client, F, Block&gt; ChainApi for LightChainApi&lt;Client, F, Block&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: BlockT,<br>&nbsp;&nbsp;&nbsp;&nbsp;Client: HeaderBackend&lt;Block&gt; + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Fetcher&lt;Block&gt; + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()