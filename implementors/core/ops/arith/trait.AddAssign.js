(function() {var implementors = {};
implementors["alga"] = [{"text":"impl AddAssign&lt;Id&lt;Additive&gt;&gt; for Id&lt;Additive&gt;","synthetic":false,"types":[]}];
implementors["bitvec"] = [{"text":"impl&lt;O, T, I&gt; AddAssign&lt;I&gt; for BitSlice&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: IntoIterator&lt;Item = bool&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;I::IntoIter: DoubleEndedIterator,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AddAssign&lt;BitBox&lt;O, T&gt;&gt; for BitBox&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;O, T&gt; AddAssign&lt;BitVec&lt;O, T&gt;&gt; for BitVec&lt;O, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;O: BitOrder,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: BitStore,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["chrono"] = [{"text":"impl AddAssign&lt;Duration&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Duration&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Duration&gt; for NaiveTime","synthetic":false,"types":[]}];
implementors["curve25519_dalek"] = [{"text":"impl&lt;'b&gt; AddAssign&lt;&amp;'b Scalar&gt; for Scalar","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Scalar&gt; for Scalar","synthetic":false,"types":[]},{"text":"impl&lt;'b&gt; AddAssign&lt;&amp;'b EdwardsPoint&gt; for EdwardsPoint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;EdwardsPoint&gt; for EdwardsPoint","synthetic":false,"types":[]},{"text":"impl&lt;'b&gt; AddAssign&lt;&amp;'b RistrettoPoint&gt; for RistrettoPoint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;RistrettoPoint&gt; for RistrettoPoint","synthetic":false,"types":[]}];
implementors["nalgebra"] = [{"text":"impl&lt;'b, N, R1, C1, R2, C2, SA, SB&gt; AddAssign&lt;&amp;'b Matrix&lt;N, R2, C2, SB&gt;&gt; for Matrix&lt;N, R1, C1, SA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R1: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;C1: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;R2: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;C2: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA: StorageMut&lt;N, R1, C1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: Storage&lt;N, R2, C2&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;R1, R2&gt; + SameNumberOfColumns&lt;C1, C2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, R1, C1, R2, C2, SA, SB&gt; AddAssign&lt;Matrix&lt;N, R2, C2, SB&gt;&gt; for Matrix&lt;N, R1, C1, SA&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R1: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;C1: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;R2: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;C2: Dim,<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;SA: StorageMut&lt;N, R1, C1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: Storage&lt;N, R2, C2&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;R1, R2&gt; + SameNumberOfColumns&lt;C1, C2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'b, N, D1:&nbsp;DimName, D2:&nbsp;Dim, SB&gt; AddAssign&lt;&amp;'b Matrix&lt;N, D2, U1, SB&gt;&gt; for Point&lt;N, D1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: Storage&lt;N, D2&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;D1, D2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N, D1:&nbsp;DimName, D2:&nbsp;Dim, SB&gt; AddAssign&lt;Matrix&lt;N, D2, U1, SB&gt;&gt; for Point&lt;N, D1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;N: Scalar + ClosedAdd,<br>&nbsp;&nbsp;&nbsp;&nbsp;SB: Storage&lt;N, D2&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, D1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShapeConstraint: SameNumberOfRows&lt;D1, D2&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'b, N:&nbsp;RealField&gt; AddAssign&lt;&amp;'b Quaternion&lt;N&gt;&gt; for Quaternion&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U4, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;N:&nbsp;RealField&gt; AddAssign&lt;Quaternion&lt;N&gt;&gt; for Quaternion&lt;N&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;DefaultAllocator: Allocator&lt;N, U4, U1&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl&lt;'a&gt; AddAssign&lt;&amp;'a BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;usize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;i8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;i16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;isize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;i32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;i64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;i128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; AddAssign&lt;&amp;'a BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u8&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u16&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;usize&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u32&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u64&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;u128&gt; for BigUint","synthetic":false,"types":[]}];
implementors["num_complex"] = [{"text":"impl&lt;T:&nbsp;Clone + NumAssign&gt; AddAssign&lt;Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone + NumAssign&gt; AddAssign&lt;T&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + NumAssign&gt; AddAssign&lt;&amp;'a Complex&lt;T&gt;&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + NumAssign&gt; AddAssign&lt;&amp;'a T&gt; for Complex&lt;T&gt;","synthetic":false,"types":[]}];
implementors["num_rational"] = [{"text":"impl&lt;T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;T&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;&amp;'a Ratio&lt;T&gt;&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T:&nbsp;Clone + Integer + NumAssign&gt; AddAssign&lt;&amp;'a T&gt; for Ratio&lt;T&gt;","synthetic":false,"types":[]}];
implementors["primitive_types"] = [{"text":"impl AddAssign&lt;U128&gt; for U128","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;U256&gt; for U256","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;U512&gt; for U512","synthetic":false,"types":[]}];
implementors["secp256k1"] = [{"text":"impl&lt;'a&gt; AddAssign&lt;&amp;'a Field&gt; for Field","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Field&gt; for Field","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; AddAssign&lt;&amp;'a Scalar&gt; for Scalar","synthetic":false,"types":[]},{"text":"impl AddAssign&lt;Scalar&gt; for Scalar","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl AddAssign&lt;Duration&gt; for Instant","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()