(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName&gt; AffineTransformation&lt;Point&lt;N, D&gt;&gt; for Rotation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D, D&gt; + Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AffineTransformation&lt;Point&lt;N, U3&gt;&gt; for UnitQuaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AffineTransformation&lt;Point&lt;N, U2&gt;&gt; for UnitComplex&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName&gt; AffineTransformation&lt;Point&lt;N, D&gt;&gt; for Translation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R&gt; AffineTransformation&lt;Point&lt;N, D&gt;&gt; for Isometry&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rotation&lt;Point&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R&gt; AffineTransformation&lt;Point&lt;N, D&gt;&gt; for Similarity&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rotation&lt;Point&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()