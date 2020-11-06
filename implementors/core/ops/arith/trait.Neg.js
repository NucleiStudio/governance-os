(function() {var implementors = {};
implementors["bitvec"] = [{"text":"impl&lt;'a, O, T&gt; Neg for &amp;'a mut BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; Neg for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; Neg for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["curve25519_dalek"] = [{"text":"impl&lt;'a&gt; Neg for &amp;'a Scalar","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for Scalar","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for &amp;'a EdwardsPoint","synthetic":false,"types":[]},{"text":"impl Neg for EdwardsPoint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for &amp;'a RistrettoPoint","synthetic":false,"types":[]},{"text":"impl Neg for RistrettoPoint","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; Neg for Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedNeg,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N, R:&nbsp;Dim, C:&nbsp;Dim, S&gt; Neg for &amp;'a Matrix&lt;N, R, C, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedNeg,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Storage&lt;N, R, C&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, R, C&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Neg&gt; Neg for Unit&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;Scalar + ClosedNeg, D:&nbsp;DimName&gt; Neg for Point&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, N:&nbsp;Scalar + ClosedNeg, D:&nbsp;DimName&gt; Neg for &amp;'a Point&lt;N, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; Neg for Quaternion&lt;N&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, N:&nbsp;RealField&gt; Neg for &amp;'a Quaternion&lt;N&gt;","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Neg for TimeSpec","synthetic":false,"types":[]},{"text":"impl Neg for TimeVal","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl Neg for Sign","synthetic":false,"types":[]},{"text":"impl Neg for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl Neg for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for &amp;'a BigUint","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Clone + Num + Neg&lt;Output = T&gt;&gt; Neg for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Num + Neg&lt;Output = T&gt;&gt; Neg for &amp;'a Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T&gt; Neg for Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer + Neg&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Neg for &amp;'a Ratio&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Clone + Integer + Neg&lt;Output = T&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["secp256k1"] = [{"text":"impl Neg for Scalar","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Neg for &amp;'a Scalar","synthetic":false,"types":[]}];
implementors["sp_arithmetic"] = [{"text":"impl Neg for FixedI64","synthetic":false,"types":[]},{"text":"impl Neg for FixedI128","synthetic":false,"types":[]},{"text":"impl Neg for FixedU128","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Neg for Duration","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Neg for Z0","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Neg for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Neg for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl Neg for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;V, A&gt; Neg for TArr&lt;V, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Neg,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Neg,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["wasmi"] = [{"text":"impl Neg for F32","synthetic":false,"types":[]},{"text":"impl Neg for F64","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()