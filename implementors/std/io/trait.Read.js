(function() {var implementors = {};
implementors["base64"] = [{"text":"impl&lt;'a, R:&nbsp;Read&gt; Read for DecoderReader&lt;'a, R&gt;","synthetic":false,"types":[]}];
implementors["bincode"] = [{"text":"impl&lt;'storage&gt; Read for SliceReader&lt;'storage&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for IoReader&lt;R&gt;","synthetic":false,"types":[]}];
implementors["bytes"] = [{"text":"impl&lt;B:&nbsp;Buf + Sized&gt; Read for Reader&lt;B&gt;","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R&gt; Read for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Read,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Read,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["flate2"] = [{"text":"impl&lt;R:&nbsp;Read&gt; Read for CrcReader&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for DeflateEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for DeflateDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for DeflateEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for DeflateDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Read for DeflateEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Read for DeflateDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for GzEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for GzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for MultiGzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for GzEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for GzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for MultiGzDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read + Write&gt; Read for GzEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Read for GzDecoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for ZlibEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for ZlibDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for ZlibEncoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;Read&gt; Read for ZlibDecoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Read for ZlibEncoder&lt;W&gt;","synthetic":false,"types":[]},{"text":"impl&lt;W:&nbsp;Read + Write&gt; Read for ZlibDecoder&lt;W&gt;","synthetic":false,"types":[]}];
implementors["futures_lite"] = [{"text":"impl&lt;T:&nbsp;AsyncRead + Unpin&gt; Read for BlockOn&lt;T&gt;","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;T&gt; Read for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Read,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["hyper"] = [{"text":"impl Read for AddrStream","synthetic":false,"types":[]},{"text":"impl Read for Upgraded","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Read for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a TcpStream","synthetic":false,"types":[]}];
implementors["mio_uds"] = [{"text":"impl Read for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a UnixStream","synthetic":false,"types":[]}];
implementors["parity_tokio_ipc"] = [{"text":"impl Read for IpcConnection","synthetic":false,"types":[]}];
implementors["rand_core"] = [{"text":"impl Read for dyn RngCore","synthetic":false,"types":[]}];
implementors["rustls"] = [{"text":"impl&lt;'a, S, T&gt; Read for Stream&lt;'a, S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: 'a + Session,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Read + Write,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;S, T&gt; Read for StreamOwned&lt;S, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Session,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Read + Write,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Read for ServerSession","synthetic":false,"types":[]},{"text":"impl Read for ClientSession","synthetic":false,"types":[]}];
implementors["sha3"] = [{"text":"impl Read for Sha3XofReader","synthetic":false,"types":[]}];
implementors["socket2"] = [{"text":"impl Read for Socket","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a Socket","synthetic":false,"types":[]}];
implementors["tempfile"] = [{"text":"impl Read for NamedTempFile","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a NamedTempFile","synthetic":false,"types":[]},{"text":"impl Read for SpooledTempFile","synthetic":false,"types":[]}];
implementors["tokio_fs"] = [{"text":"impl Read for File","synthetic":false,"types":[]},{"text":"impl Read for Stdin","synthetic":false,"types":[]}];
implementors["tokio_io"] = [{"text":"impl&lt;T&gt; Read for AllowStdIo&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Read,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;AsyncRead&gt; Read for ReadHalf&lt;T&gt;","synthetic":false,"types":[]}];
implementors["tokio_reactor"] = [{"text":"impl&lt;E&gt; Read for PollEvented&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Evented + Read,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, E&gt; Read for &amp;'a PollEvented&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Evented,<br>&nbsp;&nbsp;&nbsp;&nbsp;&amp;'a E: Read,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["tokio_tcp"] = [{"text":"impl Read for TcpStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a TcpStream","synthetic":false,"types":[]}];
implementors["tokio_uds"] = [{"text":"impl Read for UnixStream","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Read for &amp;'a UnixStream","synthetic":false,"types":[]}];
implementors["zstd"] = [{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for Decoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R:&nbsp;BufRead&gt; Read for Encoder&lt;R&gt;","synthetic":false,"types":[]},{"text":"impl&lt;R, D&gt; Read for Reader&lt;R, D&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: BufRead,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Operation,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()