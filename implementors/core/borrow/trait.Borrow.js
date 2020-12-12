(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A:&nbsp;Array&lt;Item = u8&gt;&gt; Borrow&lt;str&gt; for ArrayString&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["async_std"] = [{"text":"impl Borrow&lt;Path&gt; for PathBuf","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;O, T&gt; Borrow&lt;BitSlice&lt;O, T&gt;&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; Borrow&lt;BitSlice&lt;O, T&gt;&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["bstr"] = [{"text":"impl Borrow&lt;BStr&gt; for BString","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl Borrow&lt;[u8]&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl Borrow&lt;[u8]&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["crossbeam_epoch"] = [{"text":"impl&lt;T&gt; Borrow&lt;T&gt; for Owned&lt;T&gt;","synthetic":false,"types":[]}];
implementors["generic_array"] = [{"text":"impl&lt;T, N&gt; Borrow&lt;[T]&gt; for GenericArray&lt;T, N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: ArrayLength&lt;T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl Borrow&lt;str&gt; for HeaderName","synthetic":false,"types":[]}];
implementors["libp2p_core"] = [{"text":"impl Borrow&lt;[u8]&gt; for PeerId","synthetic":false,"types":[]}];
implementors["libp2p_kad"] = [{"text":"impl Borrow&lt;[u8]&gt; for Key","synthetic":false,"types":[]}];
implementors["log"] = [{"text":"impl&lt;'k&gt; Borrow&lt;str&gt; for Key&lt;'k&gt;","synthetic":false,"types":[]}];
implementors["multihash"] = [{"text":"impl&lt;T:&nbsp;TryFrom&lt;u64&gt;&gt; Borrow&lt;[u8]&gt; for MultihashGeneric&lt;T&gt;","synthetic":false,"types":[]}];
implementors["owning_ref"] = [{"text":"impl&lt;O, T:&nbsp;?Sized&gt; Borrow&lt;T&gt; for OwningRef&lt;O, T&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["string"] = [{"text":"impl&lt;T&gt; Borrow&lt;str&gt; for String&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AsRef&lt;[u8]&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'s, T&gt; Borrow&lt;[T]&gt; for SliceVec&lt;'s, T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Borrow&lt;[&lt;A as Array&gt;::Item]&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()