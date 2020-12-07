(function() {var implementors = {};
implementors["futures_codec"] = [{"text":"impl&lt;T, U&gt; PinnedDrop for Framed&lt;T, U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, E&gt; PinnedDrop for FramedWrite&lt;T, E&gt;","synthetic":false,"types":[]}];
implementors["futures_diagnose"] = [{"text":"impl&lt;T&gt; PinnedDrop for DiagnoseFuture&lt;T&gt;","synthetic":false,"types":[]}];
implementors["libp2p"] = [{"text":"impl&lt;TInner&gt; PinnedDrop for BandwidthListener&lt;TInner&gt;","synthetic":false,"types":[]},{"text":"impl&lt;TInner&gt; PinnedDrop for BandwidthFuture&lt;TInner&gt;","synthetic":false,"types":[]},{"text":"impl&lt;TInner&gt; PinnedDrop for BandwidthConnecLogging&lt;TInner&gt;","synthetic":false,"types":[]}];
implementors["libp2p_core"] = [{"text":"impl&lt;A, B&gt; PinnedDrop for EitherOutput&lt;A, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A, B&gt; PinnedDrop for EitherListenStream&lt;A, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A, B&gt; PinnedDrop for EitherFuture&lt;A, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A, B&gt; PinnedDrop for EitherFuture2&lt;A, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;TListener, TMap&gt; PinnedDrop for AndThenStream&lt;TListener, TMap&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, F&gt; PinnedDrop for MapStream&lt;T, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T, F&gt; PinnedDrop for MapFuture&lt;T, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Transport, F&gt; PinnedDrop for MapErrListener&lt;T, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Transport, F&gt; PinnedDrop for MapErrListenerUpgrade&lt;T, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Transport, F&gt; PinnedDrop for MapErrDial&lt;T, F&gt;","synthetic":false,"types":[]},{"text":"impl&lt;InnerStream&gt; PinnedDrop for TimeoutListener&lt;InnerStream&gt;","synthetic":false,"types":[]},{"text":"impl&lt;InnerFut&gt; PinnedDrop for Timeout&lt;InnerFut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;C, U&gt; PinnedDrop for Authenticate&lt;C, U&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: InboundUpgrade&lt;Negotiated&lt;C&gt;&gt; + OutboundUpgrade&lt;Negotiated&lt;C&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;C, U, I&gt; PinnedDrop for Multiplex&lt;C, U, I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: InboundUpgrade&lt;Negotiated&lt;C&gt;&gt; + OutboundUpgrade&lt;Negotiated&lt;C&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_pnet"] = [{"text":"impl&lt;S&gt; PinnedDrop for PnetOutput&lt;S&gt;","synthetic":false,"types":[]}];
implementors["rw_stream_sink"] = [{"text":"impl&lt;S:&nbsp;TryStream&gt; PinnedDrop for RwStreamSink&lt;S&gt;","synthetic":false,"types":[]}];
implementors["tracing_futures"] = [{"text":"impl&lt;T&gt; PinnedDrop for Instrumented&lt;T&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()