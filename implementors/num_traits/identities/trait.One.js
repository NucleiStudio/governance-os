(function() {var implementors = {};
implementors["alga"] = [{"text":"impl One for Id","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, D:&nbsp;DimName&gt; One for MatrixN&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + Zero + One + ClosedMul + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, D:&nbsp;DimName&gt; One for Rotation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + Zero + One + ClosedAdd + ClosedMul,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; One for Quaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; One for UnitQuaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; One for UnitComplex&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar + Zero + ClosedAdd, D:&nbsp;DimName&gt; One for Translation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R:&nbsp;AlgaRotation&lt;Point&lt;N, D&gt;&gt;&gt; One for Isometry&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R&gt; One for Similarity&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: AlgaRotation&lt;Point&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimNameAdd&lt;U1&gt;, C:&nbsp;TCategory&gt; One for Transform&lt;N, D, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, DimNameSum&lt;D, U1&gt;, DimNameSum&lt;D, U1&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl One for BigInt","synthetic":false,"types":[]},{"text":"impl One for BigUint","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Clone + Num&gt; One for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer&gt; One for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_traits"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()