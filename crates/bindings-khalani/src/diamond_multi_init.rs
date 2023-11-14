pub use diamond_multi_init::*;
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
pub mod diamond_multi_init {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("multiInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multiInit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InitializationFunctionReverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InitializationFunctionReverted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initializationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoBytecodeAtAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoBytecodeAtAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DIAMONDMULTIINIT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x04\xB1\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cn\x02\xFA<\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\x9BV[a\0EV[\0[\x82\x81\x14a\0\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAddress and calldata length do n`D\x82\x01Rg\r\xEE\x84\r\xAC.\x8Cm`\xC3\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x01OWa\x01=\x85\x85\x83\x81\x81\x10a\0\xCAWa\0\xCAa\x03\x07V[\x90P` \x02\x01` \x81\x01\x90a\0\xDF\x91\x90a\x03\x1DV[\x84\x84\x84\x81\x81\x10a\0\xF1Wa\0\xF1a\x03\x07V[\x90P` \x02\x81\x01\x90a\x01\x03\x91\x90a\x03MV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x01V\x92PPPV[\x80a\x01G\x81a\x03\x94V[\x91PPa\0\xADV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01hWPPV[a\x01\x8A\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x04T`(\x919a\x02\"V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x01\xA5\x91\x90a\x03\xDFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xE0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xE5V[``\x91P[P\x91P\x91P\x81a\x02\x1CW\x80Q\x15a\x01\xFFW\x80Q\x80\x82` \x01\xFD[\x83\x83`@Qc\x19!\x05\xD7`\xE0\x1B\x81R`\x04\x01a\0\xA1\x92\x91\x90a\x04'V[PPPPV[\x81;`\0\x81\x90\x03a\x02JW\x82\x82`@Qc\x91\x984\xB9`\xE0\x1B\x81R`\x04\x01a\0\xA1\x92\x91\x90a\x04'V[PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x02aW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02yW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x02\x94W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02\xB1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xC9W`\0\x80\xFD[a\x02\xD5\x88\x83\x89\x01a\x02OV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02\xEEW`\0\x80\xFD[Pa\x02\xFB\x87\x82\x88\x01a\x02OV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03/W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03FW`\0\x80\xFD[\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x03dW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x7FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x02\x94W`\0\x80\xFD[`\0`\x01\x82\x01a\x03\xB4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x03\xD6W\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xBEV[PP`\0\x91\x01RV[`\0\x82Qa\x03\xF1\x81\x84` \x87\x01a\x03\xBBV[\x91\x90\x91\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x04\x13\x81` \x86\x01` \x86\x01a\x03\xBBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x04K\x90\x83\x01\x84a\x03\xFBV[\x94\x93PPPPV\xFELibDiamondCut: _init address has no code\xA2dipfsX\"\x12 \xAAY[\xE4\xBB\xD4\xDFS\xEB\xD0\xE4\xF5Qd\\*\x8A\x1D\xE1\x9E\xC1\x93o\xF69\x1F\x88EI\xEC\xF2\x13dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DIAMONDMULTIINIT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cn\x02\xFA<\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\x9BV[a\0EV[\0[\x82\x81\x14a\0\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FAddress and calldata length do n`D\x82\x01Rg\r\xEE\x84\r\xAC.\x8Cm`\xC3\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x83\x81\x10\x15a\x01OWa\x01=\x85\x85\x83\x81\x81\x10a\0\xCAWa\0\xCAa\x03\x07V[\x90P` \x02\x01` \x81\x01\x90a\0\xDF\x91\x90a\x03\x1DV[\x84\x84\x84\x81\x81\x10a\0\xF1Wa\0\xF1a\x03\x07V[\x90P` \x02\x81\x01\x90a\x01\x03\x91\x90a\x03MV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x01V\x92PPPV[\x80a\x01G\x81a\x03\x94V[\x91PPa\0\xADV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x01hWPPV[a\x01\x8A\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x04T`(\x919a\x02\"V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x01\xA5\x91\x90a\x03\xDFV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01\xE0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\xE5V[``\x91P[P\x91P\x91P\x81a\x02\x1CW\x80Q\x15a\x01\xFFW\x80Q\x80\x82` \x01\xFD[\x83\x83`@Qc\x19!\x05\xD7`\xE0\x1B\x81R`\x04\x01a\0\xA1\x92\x91\x90a\x04'V[PPPPV[\x81;`\0\x81\x90\x03a\x02JW\x82\x82`@Qc\x91\x984\xB9`\xE0\x1B\x81R`\x04\x01a\0\xA1\x92\x91\x90a\x04'V[PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x02aW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02yW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x02\x94W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x02\xB1W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xC9W`\0\x80\xFD[a\x02\xD5\x88\x83\x89\x01a\x02OV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x02\xEEW`\0\x80\xFD[Pa\x02\xFB\x87\x82\x88\x01a\x02OV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03/W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03FW`\0\x80\xFD[\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x03dW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x7FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x02\x94W`\0\x80\xFD[`\0`\x01\x82\x01a\x03\xB4WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x03\xD6W\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xBEV[PP`\0\x91\x01RV[`\0\x82Qa\x03\xF1\x81\x84` \x87\x01a\x03\xBBV[\x91\x90\x91\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x04\x13\x81` \x86\x01` \x86\x01a\x03\xBBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x04K\x90\x83\x01\x84a\x03\xFBV[\x94\x93PPPPV\xFELibDiamondCut: _init address has no code\xA2dipfsX\"\x12 \xAAY[\xE4\xBB\xD4\xDFS\xEB\xD0\xE4\xF5Qd\\*\x8A\x1D\xE1\x9E\xC1\x93o\xF69\x1F\x88EI\xEC\xF2\x13dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DIAMONDMULTIINIT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DiamondMultiInit<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DiamondMultiInit<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DiamondMultiInit<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DiamondMultiInit<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DiamondMultiInit<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DiamondMultiInit))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DiamondMultiInit<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DIAMONDMULTIINIT_ABI.clone(),
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
                DIAMONDMULTIINIT_ABI.clone(),
                DIAMONDMULTIINIT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `multiInit` (0x6e02fa3c) function
        pub fn multi_init(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            calldata: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 2, 250, 60], (addresses, calldata))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DiamondMultiInit<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InitializationFunctionReverted` with signature `InitializationFunctionReverted(address,bytes)` and selector `0x192105d7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InitializationFunctionReverted",
        abi = "InitializationFunctionReverted(address,bytes)"
    )]
    pub struct InitializationFunctionReverted {
        pub initialization_contract_address: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `NoBytecodeAtAddress` with signature `NoBytecodeAtAddress(address,string)` and selector `0x919834b9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "NoBytecodeAtAddress",
        abi = "NoBytecodeAtAddress(address,string)"
    )]
    pub struct NoBytecodeAtAddress {
        pub contract_address: ::ethers::core::types::Address,
        pub message: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
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
    pub enum DiamondMultiInitErrors {
        InitializationFunctionReverted(InitializationFunctionReverted),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DiamondMultiInitErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InitializationFunctionReverted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializationFunctionReverted(decoded));
            }
            if let Ok(decoded) = <NoBytecodeAtAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoBytecodeAtAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DiamondMultiInitErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InitializationFunctionReverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DiamondMultiInitErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InitializationFunctionReverted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoBytecodeAtAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DiamondMultiInitErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DiamondMultiInitErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted>
    for DiamondMultiInitErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for DiamondMultiInitErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    ///Container type for all input parameters for the `multiInit` function with signature `multiInit(address[],bytes[])` and selector `0x6e02fa3c`
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
    #[ethcall(name = "multiInit", abi = "multiInit(address[],bytes[])")]
    pub struct MultiInitCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub calldata: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
}
