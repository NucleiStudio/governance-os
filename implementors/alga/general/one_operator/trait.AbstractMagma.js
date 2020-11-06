(function() {var implementors = {};
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;DimName, C:&nbsp;DimName&gt; AbstractMagma&lt;Additive&gt; for MatrixMN&lt;N, R, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, D:&nbsp;DimName&gt; AbstractMagma&lt;Multiplicative&gt; for MatrixN&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + Zero + One + ClosedAdd + ClosedMul,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName&gt; AbstractMagma&lt;Multiplicative&gt; for Rotation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AbstractMagma&lt;Multiplicative&gt; for Quaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AbstractMagma&lt;Additive&gt; for Quaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AbstractMagma&lt;Multiplicative&gt; for UnitQuaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AbstractMagma&lt;Multiplicative&gt; for UnitComplex&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName&gt; AbstractMagma&lt;Multiplicative&gt; for Translation&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R&gt; AbstractMagma&lt;Multiplicative&gt; for Isometry&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rotation&lt;Point&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimName, R&gt; AbstractMagma&lt;Multiplicative&gt; for Similarity&lt;N, D, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Rotation&lt;Point&lt;N, D&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField, D:&nbsp;DimNameAdd&lt;U1&gt;, C&gt; AbstractMagma&lt;Multiplicative&gt; for Transform&lt;N, D, C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: TCategory,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, DimNameSum&lt;D, U1&gt;, DimNameSum&lt;D, U1&gt;&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()