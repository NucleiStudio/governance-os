initSidebarItems({"enum":[["NegotiationError","Error that can happen when negotiating a protocol with the remote."],["ProtocolError","A protocol error."],["Version","Supported multistream-select protocol versions."]],"fn":[["dialer_select_proto","Returns a `Future` that negotiates a protocol on the given I/O stream for a peer acting as the dialer (or initiator)."],["listener_select_proto","Returns a `Future` that negotiates a protocol on the given I/O stream for a peer acting as the listener (or responder)."]],"struct":[["ListenerSelectFuture","The `Future` returned by [`listener_select_proto`] that performs a multistream-select protocol negotiation on an underlying I/O stream."],["Negotiated","An I/O stream that has settled on an (application-layer) protocol to use."],["NegotiatedComplete","A `Future` that waits on the completion of protocol negotiation."]],"type":[["DialerSelectFuture","Future, returned by `dialer_select_proto`, which selects a protocol and dialer either trying protocols in-order, or by requesting all protocols supported by the remote upfront, from which the first protocol found in the dialer's list of protocols is selected."]]});