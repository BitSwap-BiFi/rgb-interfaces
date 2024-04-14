// RGB interfaces by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rgbstd::interface::{ContractIface, DataAllocation, Iface, IfaceId, OutpointFilter};
use rgbstd::stl::{bp_tx_stl, rgb_contract_stl, AssetSpec, AssetTerms};
use strict_types::stl::std_stl;
use strict_types::{CompileError, LibBuilder, TypeLib};

use super::iface::*;
use super::{
    Allocation, AttachmentType, EngravingData, Features, Issues, ItemsCount, TokenData,
    LIB_NAME_RGB21,
};
use crate::rgb20::iface::{named_asset, renameable, reservable};
use crate::IfaceWrapper;

pub const RGB21_UNIQUE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x2c, 0x7a, 0x73, 0x3e, 0xeb, 0x4e, 0x39, 0x30, 0x22, 0x0a, 0xaa, 0xb9, 0x46, 0x7a, 0x47, 0xb9,
    0x6c, 0x1c, 0x30, 0xe5, 0xbe, 0xc5, 0x4c, 0x0b, 0x85, 0x54, 0x93, 0x83, 0xe8, 0xe1, 0xa9, 0x71,
]);

pub const RGB21_IFACE_ID: IfaceId = IfaceId::from_array([
    0xb1, 0x26, 0xe2, 0x92, 0x90, 0x28, 0xad, 0xdb, 0x9a, 0x7f, 0x21, 0xc3, 0xce, 0x3c, 0x2c, 0xce,
    0x73, 0x0c, 0xcf, 0xaf, 0x57, 0x20, 0xfa, 0xde, 0xdb, 0xea, 0x47, 0x3b, 0x44, 0x58, 0x53, 0x27,
]);

fn _rgb21_stl() -> Result<TypeLib, CompileError> {
    LibBuilder::new(libname!(LIB_NAME_RGB21), tiny_bset! {
        std_stl().to_dependency(),
        bp_tx_stl().to_dependency(),
        rgb_contract_stl().to_dependency()
    })
    .transpile::<TokenData>()
    .transpile::<EngravingData>()
    .transpile::<ItemsCount>()
    .transpile::<Allocation>()
    .transpile::<AttachmentType>()
    .compile()
}

/// Generates strict type library providing data types for RGB21 interface.
fn rgb21_stl() -> TypeLib { _rgb21_stl().expect("invalid strict type RGB21 library") }

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb21(ContractIface);

impl From<ContractIface> for Rgb21 {
    fn from(iface: ContractIface) -> Self {
        if !Rgb21::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB21 interface");
        }
        Self(iface)
    }
}

impl IfaceWrapper for Rgb21 {
    const IFACE_NAME: &'static str = LIB_NAME_RGB21;
    const IFACE_IDS: &'static [IfaceId] = &[RGB21_UNIQUE_IFACE_ID, RGB21_IFACE_ID];

    type Features = Features;

    fn iface(features: Self::Features) -> Iface {
        let mut iface = named_asset().expect_extended(nft(), "RGB21Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB21Renameable");
        }
        if features.engraving {
            iface = iface.expect_extended(engravable(), "RGB21Engravable");
        }
        iface = match features.issues {
            Issues::Unique => iface.expect_extended(unique(), "RGB21Unique"),
            Issues::Limited => iface.expect_extended(limited(), "RGB21Limited"),
            Issues::MultiIssue => iface.expect_extended(issuable(), "RGB21Issuable"),
        };
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB21Reservable");
        }
        if features == Features::ALL {
            iface.name = Self::IFACE_NAME.into();
        }
        iface
    }
    fn stl() -> TypeLib { rgb21_stl() }
}

impl Rgb21 {
    pub fn spec(&self) -> AssetSpec {
        let strict_val = &self
            .0
            .global("spec")
            .expect("RGB21 interface requires global `spec`")[0];
        AssetSpec::from_strict_val_unchecked(strict_val)
    }

    pub fn contract_terms(&self) -> AssetTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB21 interface requires global `terms`")[0];
        AssetTerms::from_strict_val_unchecked(strict_val)
    }

    pub fn token_data(&self) -> TokenData {
        let strict_val = &self
            .0
            .global("tokens")
            .expect("RGB21 interface requires global `tokens`")[0];
        TokenData::from_strict_val_unchecked(strict_val)
    }

    pub fn engarving_data(&self) -> EngravingData {
        let strict_val = &self
            .0
            .global("engravings")
            .expect("RGB21 interface requires global state `engravings`")[0];
        EngravingData::from_strict_val_unchecked(strict_val)
    }

    pub fn allocations<'c>(
        &'c self,
        filter: impl OutpointFilter + 'c,
    ) -> impl Iterator<Item = DataAllocation> + 'c {
        self.0
            .data("assetOwner", filter)
            .expect("RGB21 interface requires `assetOwner` state")
    }
}
