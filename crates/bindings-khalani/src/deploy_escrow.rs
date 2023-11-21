pub use deploy_escrow::*;
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
pub mod deploy_escrow {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("run"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("run"),
                            inputs: ::std::vec![],
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
    pub static DEPLOYESCROW_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x04\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0C\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa\x06\xA4\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xC0@b&\x14a\0;W\x80c\xF8\xCC\xBFG\x14a\0EW[`\0\x80\xFD[a\0Ca\0fV[\0[`\x0CTa\0R\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xD8W=`\0\x80>=`\0\xFD[PPPP`\0`@Qa\0\xEA\x90a\x02#V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\x06W=`\0\x80>=`\0\xFD[P\x90Pa\x01@`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01tDeployed Escrow at %s`X\x1B\x81RP\x82a\x01\xB9V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xB2W=`\0\x80>=`\0\xFD[PPPPPV[a\x01\xFE\x82\x82`@Q`$\x01a\x01\xCF\x92\x91\x90a\x020V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x02\x02V[PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[a\x03\xDE\x80a\x02\x91\x839\x01\x90V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x02^W` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x02AV[P`\0``\x82\x85\x01\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16` \x84\x01R`\x1F\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xBE\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0*W`\x005`\xE0\x1C\x80b\xE8:\xFD\x14a\0/W[`\0\x80\xFD[a\0Ba\0=6`\x04a\0\xEEV[a\0DV[\0[`\0a\0Wa\0R\x83a\x02.V[a\0\x96V[\x90P\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\0\x8A\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\0\xD1\x97\x96\x95\x94\x93\x92\x91\x90a\x02\xFBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x17W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x01*W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01kWa\x01ka\x011V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x01\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB9Wa\x01\xB9a\x011V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\xE1Wa\x01\xE1a\x011V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[`\0a\x01\0\x826\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02Ia\x01GV[a\x02R\x83a\x01qV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02oW`\0\x80\xFD[a\x02{6\x83\x87\x01a\x01\x8DV[` \x84\x01Ra\x02\x8C`@\x86\x01a\x02\x1AV[`@\x84\x01Ra\x02\x9D``\x86\x01a\x02\x1AV[``\x84\x01Ra\x02\xAE`\x80\x86\x01a\x01qV[`\x80\x84\x01Ra\x02\xBF`\xA0\x86\x01a\x01qV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xEF6\x82\x86\x01a\x01\x8DV[`\xE0\x83\x01RP\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Q`\0[\x81\x81\x10\x15a\x03qW` \x81\x86\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x03TV[P`\0\x92\x01`d\x01\x91\x82RP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x19;W3\xE1\xC1f(7by\xFB_\xD2bhru3\xA3\xDE\x90\xF4J\xB0g\x9C\xD7\x13\x16V\x14dsolcC\0\x08\x13\x003\xA2dipfsX\"\x12 \xC3\rs4P\xAD\x9D\xF8\x1Cj\xD9\xB5\x14\x01ZX\x85yn\x9Fz\x97(\x14\x1F\xA6\x96\x05\xF2\x05\xD3;dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DEPLOYESCROW_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\xC0@b&\x14a\0;W\x80c\xF8\xCC\xBFG\x14a\0EW[`\0\x80\xFD[a\0Ca\0fV[\0[`\x0CTa\0R\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xC4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\0\xD8W=`\0\x80>=`\0\xFD[PPPP`\0`@Qa\0\xEA\x90a\x02#V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x01\x06W=`\0\x80>=`\0\xFD[P\x90Pa\x01@`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01tDeployed Escrow at %s`X\x1B\x81RP\x82a\x01\xB9V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xB2W=`\0\x80>=`\0\xFD[PPPPPV[a\x01\xFE\x82\x82`@Q`$\x01a\x01\xCF\x92\x91\x90a\x020V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x02\x02V[PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[a\x03\xDE\x80a\x02\x91\x839\x01\x90V[`@\x81R`\0\x83Q\x80`@\x84\x01R`\0[\x81\x81\x10\x15a\x02^W` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a\x02AV[P`\0``\x82\x85\x01\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16` \x84\x01R`\x1F\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x92\x91PPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xBE\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0*W`\x005`\xE0\x1C\x80b\xE8:\xFD\x14a\0/W[`\0\x80\xFD[a\0Ba\0=6`\x04a\0\xEEV[a\0DV[\0[`\0a\0Wa\0R\x83a\x02.V[a\0\x96V[\x90P\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\0\x8A\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\0\xD1\x97\x96\x95\x94\x93\x92\x91\x90a\x02\xFBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x17W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x01*W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01kWa\x01ka\x011V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x01\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB9Wa\x01\xB9a\x011V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\xE1Wa\x01\xE1a\x011V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[`\0a\x01\0\x826\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02Ia\x01GV[a\x02R\x83a\x01qV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02oW`\0\x80\xFD[a\x02{6\x83\x87\x01a\x01\x8DV[` \x84\x01Ra\x02\x8C`@\x86\x01a\x02\x1AV[`@\x84\x01Ra\x02\x9D``\x86\x01a\x02\x1AV[``\x84\x01Ra\x02\xAE`\x80\x86\x01a\x01qV[`\x80\x84\x01Ra\x02\xBF`\xA0\x86\x01a\x01qV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xEF6\x82\x86\x01a\x01\x8DV[`\xE0\x83\x01RP\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Q`\0[\x81\x81\x10\x15a\x03qW` \x81\x86\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x03TV[P`\0\x92\x01`d\x01\x91\x82RP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x19;W3\xE1\xC1f(7by\xFB_\xD2bhru3\xA3\xDE\x90\xF4J\xB0g\x9C\xD7\x13\x16V\x14dsolcC\0\x08\x13\x003\xA2dipfsX\"\x12 \xC3\rs4P\xAD\x9D\xF8\x1Cj\xD9\xB5\x14\x01ZX\x85yn\x9Fz\x97(\x14\x1F\xA6\x96\x05\xF2\x05\xD3;dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DEPLOYESCROW_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DeployEscrow<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeployEscrow<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeployEscrow<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeployEscrow<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeployEscrow<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DeployEscrow))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployEscrow<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYESCROW_ABI.clone(),
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
                DEPLOYESCROW_ABI.clone(),
                DEPLOYESCROW_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xc0406226) function
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeployEscrow<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `run` function with signature `run()` and selector `0xc0406226`
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
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
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
    pub enum DeployEscrowCalls {
        IsScript(IsScriptCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployEscrowCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployEscrowCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DeployEscrowCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeployEscrowCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<RunCall> for DeployEscrowCalls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
}
