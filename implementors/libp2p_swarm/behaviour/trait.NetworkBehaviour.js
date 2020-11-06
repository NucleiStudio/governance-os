(function() {var implementors = {};
implementors["libp2p_floodsub"] = [{"text":"impl NetworkBehaviour for Floodsub","synthetic":false,"types":[]}];
implementors["libp2p_gossipsub"] = [{"text":"impl NetworkBehaviour for Gossipsub","synthetic":false,"types":[]}];
implementors["libp2p_identify"] = [{"text":"impl NetworkBehaviour for Identify","synthetic":false,"types":[]}];
implementors["libp2p_kad"] = [{"text":"impl&lt;TStore&gt; NetworkBehaviour for Kademlia&lt;TStore&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TStore: RecordStore&lt;'a&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;TStore: Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_mdns"] = [{"text":"impl NetworkBehaviour for Mdns","synthetic":false,"types":[]}];
implementors["libp2p_ping"] = [{"text":"impl NetworkBehaviour for Ping","synthetic":false,"types":[]}];
implementors["libp2p_request_response"] = [{"text":"impl&lt;C&gt; NetworkBehaviour for Throttled&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: RequestResponseCodec + Send + Clone + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;C::Protocol: Sync,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;TCodec&gt; NetworkBehaviour for RequestResponse&lt;TCodec&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TCodec: RequestResponseCodec + Send + Clone + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()