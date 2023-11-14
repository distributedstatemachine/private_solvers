pub use mock_igp::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mock_igp {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("gasPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payForGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payForGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteGasPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteGasPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GasPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GasPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKIGP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03/\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c\x11\xBF,\x18\x14a\09W\x80c\xA6\x92\x97\x93\x14a\0NW\x80c\xFE\x17;\x97\x14a\0\x80W[`\0\x80\xFD[a\0La\0G6`\x04a\x02;V[a\0\x95V[\0[4\x80\x15a\0ZW`\0\x80\xFD[Pa\0na\0i6`\x04a\x02\x8FV[a\x02\x0CV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0n`\n\x81V[`\0a\0\xA1\x84\x84a\x02\x0CV[\x90P\x804\x10\x15a\x01\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finsufficient interchain gas paym`D\x82\x01Rb\x19[\x9D`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x01\x10\x824a\x02\xCFV[\x90P\x80\x15a\x01\xC9W`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01eW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01jV[``\x91P[PP\x90P\x80a\x01\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FInterchain gas payment refund fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\0\xFBV[P[`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x87\x91\x7F\xF7\x15\xE6m,\xD2\xA0\x17\x90i\xDC\xDA*A\xA4\xDA6\xA3\x0E\x1A\xAD\x12\x18\x7F\xBB|]\x10\n\xFDb\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x02\x19\x82`\na\x02\xE2V[\x90P[\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x026W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x02QW`\0\x80\xFD[\x845\x93Pa\x02a` \x86\x01a\x02\"V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x84W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xA2W`\0\x80\xFD[a\x02\xAB\x83a\x02\"V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x1CWa\x02\x1Ca\x02\xB9V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x1CWa\x02\x1Ca\x02\xB9V\xFE\xA2dipfsX\"\x12 \x98\x93\xA9\xFA\x8FeF\xFB\xC4>\xDB\xEE\xB2\xC4\x0C\xB6\xBB\x96\"#\xB0\xF6\x1F\x07\xC6M\x1C\x13U\x94\xC5MdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKIGP_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x004W`\x005`\xE0\x1C\x80c\x11\xBF,\x18\x14a\09W\x80c\xA6\x92\x97\x93\x14a\0NW\x80c\xFE\x17;\x97\x14a\0\x80W[`\0\x80\xFD[a\0La\0G6`\x04a\x02;V[a\0\x95V[\0[4\x80\x15a\0ZW`\0\x80\xFD[Pa\0na\0i6`\x04a\x02\x8FV[a\x02\x0CV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0n`\n\x81V[`\0a\0\xA1\x84\x84a\x02\x0CV[\x90P\x804\x10\x15a\x01\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finsufficient interchain gas paym`D\x82\x01Rb\x19[\x9D`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x01\x10\x824a\x02\xCFV[\x90P\x80\x15a\x01\xC9W`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x01eW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01jV[``\x91P[PP\x90P\x80a\x01\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FInterchain gas payment refund fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\0\xFBV[P[`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x87\x91\x7F\xF7\x15\xE6m,\xD2\xA0\x17\x90i\xDC\xDA*A\xA4\xDA6\xA3\x0E\x1A\xAD\x12\x18\x7F\xBB|]\x10\n\xFDb\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x02\x19\x82`\na\x02\xE2V[\x90P[\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x026W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x02QW`\0\x80\xFD[\x845\x93Pa\x02a` \x86\x01a\x02\"V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x84W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xA2W`\0\x80\xFD[a\x02\xAB\x83a\x02\"V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\x1CWa\x02\x1Ca\x02\xB9V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x1CWa\x02\x1Ca\x02\xB9V\xFE\xA2dipfsX\"\x12 \x98\x93\xA9\xFA\x8FeF\xFB\xC4>\xDB\xEE\xB2\xC4\x0C\xB6\xBB\x96\"#\xB0\xF6\x1F\x07\xC6M\x1C\x13U\x94\xC5MdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKIGP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockIgp<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockIgp<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockIgp<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockIgp<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockIgp<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockIgp)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockIgp<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKIGP_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOCKIGP_ABI.clone(),
                MOCKIGP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `gasPrice` (0xfe173b97) function
        pub fn gas_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 23, 59, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payForGas` (0x11bf2c18) function
        pub fn pay_for_gas(
            &self,
            message_id: [u8; 32],
            destination_domain: u32,
            gas_amount: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [17, 191, 44, 24],
                    (message_id, destination_domain, gas_amount, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteGasPayment` (0xa6929793) function
        pub fn quote_gas_payment(
            &self,
            destination_domain: u32,
            gas_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 146, 151, 147], (destination_domain, gas_amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GasPayment` event
        pub fn gas_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymentFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymentFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockIgp<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "GasPayment", abi = "GasPayment(bytes32,uint256,uint256)")]
    pub struct GasPaymentFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub gas_amount: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gasPrice` function with signature `gasPrice()` and selector `0xfe173b97`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gasPrice", abi = "gasPrice()")]
    pub struct GasPriceCall;
    ///Container type for all input parameters for the `payForGas` function with signature `payForGas(bytes32,uint32,uint256,address)` and selector `0x11bf2c18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "payForGas", abi = "payForGas(bytes32,uint32,uint256,address)")]
    pub struct PayForGasCall {
        pub message_id: [u8; 32],
        pub destination_domain: u32,
        pub gas_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quoteGasPayment` function with signature `quoteGasPayment(uint32,uint256)` and selector `0xa6929793`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quoteGasPayment", abi = "quoteGasPayment(uint32,uint256)")]
    pub struct QuoteGasPaymentCall {
        pub destination_domain: u32,
        pub gas_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MockIgpCalls {
        GasPrice(GasPriceCall),
        PayForGas(PayForGasCall),
        QuoteGasPayment(QuoteGasPaymentCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockIgpCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GasPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasPrice(decoded));
            }
            if let Ok(decoded) = <PayForGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayForGas(decoded));
            }
            if let Ok(decoded) = <QuoteGasPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteGasPayment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockIgpCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayForGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteGasPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockIgpCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayForGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteGasPayment(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GasPriceCall> for MockIgpCalls {
        fn from(value: GasPriceCall) -> Self {
            Self::GasPrice(value)
        }
    }
    impl ::core::convert::From<PayForGasCall> for MockIgpCalls {
        fn from(value: PayForGasCall) -> Self {
            Self::PayForGas(value)
        }
    }
    impl ::core::convert::From<QuoteGasPaymentCall> for MockIgpCalls {
        fn from(value: QuoteGasPaymentCall) -> Self {
            Self::QuoteGasPayment(value)
        }
    }
    ///Container type for all return fields from the `gasPrice` function with signature `gasPrice()` and selector `0xfe173b97`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GasPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quoteGasPayment` function with signature `quoteGasPayment(uint32,uint256)` and selector `0xa6929793`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuoteGasPaymentReturn(pub ::ethers::core::types::U256);
}
