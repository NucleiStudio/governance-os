(function() {var implementors = {};
implementors["bincode"] = [{"text":"impl&lt;'a, W:&nbsp;Write, O:&nbsp;Options&gt; Serializer for &amp;'a mut Serializer&lt;W, O&gt;","synthetic":false,"types":[]}];
implementors["erased_serde"] = [{"text":"impl&lt;'a&gt; Serializer for &amp;'a mut dyn Serializer","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Serializer for &amp;'a mut (dyn Serializer + Send)","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Serializer for &amp;'a mut (dyn Serializer + Sync)","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Serializer for &amp;'a mut (dyn Serializer + Send + Sync)","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl&lt;'a, W, F&gt; Serializer for &amp;'a mut Serializer&lt;W, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Write,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Formatter,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Serializer for Serializer","synthetic":false,"types":[]}];
implementors["toml"] = [{"text":"impl&lt;'a, 'b&gt; Serializer for &amp;'b mut Serializer&lt;'a&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()