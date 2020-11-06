(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; FusedStream for Receiver&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; FusedStream for UnboundedReceiver&lt;T&gt;","synthetic":false,"types":[]}];
implementors["futures_core"] = [];
implementors["libp2p_swarm"] = [{"text":"impl&lt;TBehaviour, TInEvent, TOutEvent, THandler, TConnInfo&gt; FusedStream for ExpandedSwarm&lt;TBehaviour, TInEvent, TOutEvent, THandler, TConnInfo&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TBehaviour: NetworkBehaviour&lt;ProtocolsHandler = THandler&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;THandler: IntoProtocolsHandler + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;TInEvent: Clone + Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;TOutEvent: Send + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;THandler::Handler: ProtocolsHandler&lt;InEvent = TInEvent, OutEvent = TOutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;TConnInfo: ConnectionInfo&lt;PeerId = PeerId&gt; + Debug + Clone + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["sp_utils"] = [{"text":"impl&lt;T&gt; FusedStream for TracingUnboundedReceiver&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()