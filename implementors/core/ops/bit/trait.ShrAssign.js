(function() {var implementors = {};
implementors["bitvec"] = [{"text":"impl&lt;O, T&gt; ShrAssign&lt;usize&gt; for BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; ShrAssign&lt;usize&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; ShrAssign&lt;usize&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl ShrAssign&lt;usize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl ShrAssign&lt;usize&gt; for BigUint","synthetic":false,"types":[]}];
implementors["primitive_types"] = [{"text":"impl&lt;T&gt; ShrAssign&lt;T&gt; for U128 <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;U128&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; ShrAssign&lt;T&gt; for U256 <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;U256&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; ShrAssign&lt;T&gt; for U512 <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Into&lt;U512&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()