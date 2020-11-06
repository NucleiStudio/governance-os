(function() {var implementors = {};
implementors["frame_benchmarking"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn Benchmark&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["frame_system_rpc_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;BlockT, AccountId, Index, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn AccountNonceApi&lt;Block, AccountId, Index, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["pallet_transaction_payment_rpc_runtime_api"] = [{"text":"impl&lt;Block:&nbsp;BlockT, Balance, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn TransactionPaymentApi&lt;Block, Balance, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_api"] = [];
implementors["sp_block_builder"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn BlockBuilder&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_consensus_aura"] = [{"text":"impl&lt;Block:&nbsp;BlockT, AuthorityId:&nbsp;Codec, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn AuraApi&lt;Block, AuthorityId, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_finality_grandpa"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn GrandpaApi&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_offchain"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn OffchainWorkerApi&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_session"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn SessionKeys&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
implementors["sp_transaction_pool"] = [{"text":"impl&lt;Block:&nbsp;BlockT, __Sr_Api_Error__&gt; RuntimeApiInfo for dyn TaggedTransactionQueue&lt;Block, Error = __Sr_Api_Error__&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()