pub use mock_counter::*;
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
pub mod mock_counter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_caller"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("a"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKCOUNTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\xBF8\x03\x80a\x02\xBF\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xCDV[a\08\x81a\0]V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\xFDV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01Ra\0\xA9\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c\x16\x17e\xE1`\xE1\x1B\x17\x90\x91Ra\0\xAC\x16V[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0` \x82\x84\x03\x12\x15a\0\xDFW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xF6W`\0\x80\xFD[\x93\x92PPPV[a\x01\xB3\x80a\x01\x0C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cF\xD4\xAD\xF2\x14a\0;W\x80c\xA8}\x94,\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01=V[a\0eV[\0[`\0T`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0n3a\0\xD1V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm4\xB7;0\xB64\xB2\x101\xB0\xB662\xB9`\x91\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0Ta\0\xCB\x91\x90a\x01VV[`\0UPV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01Ra\x01\x19\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x16\x17e\xE1`\xE1\x1B\x17\x90Ra\x01\x1CV[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0` \x82\x84\x03\x12\x15a\x01OW`\0\x80\xFD[P5\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x01wWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCB\x9B\xA9\x96i\x87\x04\xAA\xADh\xD9U\xCE\x89CeG\x16\x81\x93N\x84\xA5\xED*\xD7\xDCI\x03\x0C\xD47dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKCOUNTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cF\xD4\xAD\xF2\x14a\0;W\x80c\xA8}\x94,\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01=V[a\0eV[\0[`\0T`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0n3a\0\xD1V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\0\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm4\xB7;0\xB64\xB2\x101\xB0\xB662\xB9`\x91\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x80`\0Ta\0\xCB\x91\x90a\x01VV[`\0UPV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01Ra\x01\x19\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x16\x17e\xE1`\xE1\x1B\x17\x90Ra\x01\x1CV[PV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\0` \x82\x84\x03\x12\x15a\x01OW`\0\x80\xFD[P5\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x01wWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCB\x9B\xA9\x96i\x87\x04\xAA\xADh\xD9U\xCE\x89CeG\x16\x81\x93N\x84\xA5\xED*\xD7\xDCI\x03\x0C\xD47dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKCOUNTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockCounter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockCounter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockCounter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockCounter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockCounter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockCounter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockCounter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKCOUNTER_ABI.clone(),
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
                MOCKCOUNTER_ABI.clone(),
                MOCKCOUNTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getCount` (0xa87d942c) function
        pub fn get_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 125, 148, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseCount` (0x46d4adf2) function
        pub fn increase_count(
            &self,
            a: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 212, 173, 242], a)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockCounter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCount` function with signature `getCount()` and selector `0xa87d942c`
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
    #[ethcall(name = "getCount", abi = "getCount()")]
    pub struct GetCountCall;
    ///Container type for all input parameters for the `increaseCount` function with signature `increaseCount(uint256)` and selector `0x46d4adf2`
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
    #[ethcall(name = "increaseCount", abi = "increaseCount(uint256)")]
    pub struct IncreaseCountCall {
        pub a: ::ethers::core::types::U256,
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
    pub enum MockCounterCalls {
        GetCount(GetCountCall),
        IncreaseCount(IncreaseCountCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockCounterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCount(decoded));
            }
            if let Ok(decoded) = <IncreaseCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseCount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockCounterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockCounterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseCount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCountCall> for MockCounterCalls {
        fn from(value: GetCountCall) -> Self {
            Self::GetCount(value)
        }
    }
    impl ::core::convert::From<IncreaseCountCall> for MockCounterCalls {
        fn from(value: IncreaseCountCall) -> Self {
            Self::IncreaseCount(value)
        }
    }
    ///Container type for all return fields from the `getCount` function with signature `getCount()` and selector `0xa87d942c`
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
    pub struct GetCountReturn(pub ::ethers::core::types::U256);
}
