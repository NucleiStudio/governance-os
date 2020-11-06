(function() {var implementors = {};
implementors["futures"] = [];
implementors["futures_util"] = [{"text":"impl&lt;T, Item&gt; Sink for CompatSink&lt;T, Item&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sink03&lt;Item&gt; + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Sink for Sender","synthetic":false,"types":[]}];
implementors["jsonrpc_client_transports"] = [{"text":"impl&lt;TMetadata, THandler, TMiddleware&gt; Sink for LocalRpc&lt;THandler, TMetadata&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TMetadata: Metadata,<br>&nbsp;&nbsp;&nbsp;&nbsp;TMiddleware: Middleware&lt;TMetadata&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;THandler: Deref&lt;Target = MetaIoHandler&lt;TMetadata, TMiddleware&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["jsonrpc_pubsub"] = [{"text":"impl Sink for Sink","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Serialize, E:&nbsp;Serialize&gt; Sink for Sink&lt;T, E&gt;","synthetic":false,"types":[]}];
implementors["tokio_sync"] = [{"text":"impl&lt;T&gt; Sink for Sender&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Sink for UnboundedSender&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Sink for Sender&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_udp"] = [{"text":"impl&lt;C:&nbsp;Encoder&gt; Sink for UdpFramed&lt;C&gt;","synthetic":false,"types":[]}];
implementors["tokio_uds"] = [{"text":"impl&lt;A:&nbsp;AsRef&lt;Path&gt;, C:&nbsp;Encoder&gt; Sink for UnixDatagramFramed&lt;A, C&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()