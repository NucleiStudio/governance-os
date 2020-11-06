(function() {var implementors = {};
implementors["libp2p_core"] = [];
implementors["libp2p_mplex"] = [{"text":"impl&lt;C&gt; StreamMuxer for Multiplex&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: AsyncRead + AsyncWrite + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["libp2p_yamux"] = [{"text":"impl&lt;S&gt; StreamMuxer for Yamux&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Stream&lt;Item = Result&lt;Stream, YamuxError&gt;&gt; + Unpin,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()