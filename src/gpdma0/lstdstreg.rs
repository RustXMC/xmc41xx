#[doc = "Register `LSTDSTREG` reader"]
pub struct R(crate::R<LSTDSTREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSTDSTREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSTDSTREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSTDSTREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSTDSTREG` writer"]
pub struct W(crate::W<LSTDSTREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSTDSTREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LSTDSTREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSTDSTREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Destination last request for channel 0"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Destination last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::VALUE1,
            true => CH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH0_A::VALUE2
    }
}
#[doc = "Field `CH0` writer - Destination last request for channel 0"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH0_A, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH0_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH0_A::VALUE2)
    }
}
#[doc = "Field `CH1` reader - Destination last request for channel 1"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Destination last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::VALUE1,
            true => CH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH1_A::VALUE2
    }
}
#[doc = "Field `CH1` writer - Destination last request for channel 1"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH1_A, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH1_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH1_A::VALUE2)
    }
}
#[doc = "Field `CH2` reader - Destination last request for channel 2"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Destination last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::VALUE1,
            true => CH2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH2_A::VALUE2
    }
}
#[doc = "Field `CH2` writer - Destination last request for channel 2"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH2_A, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH2_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH2_A::VALUE2)
    }
}
#[doc = "Field `CH3` reader - Destination last request for channel 3"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Destination last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::VALUE1,
            true => CH3_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH3_A::VALUE2
    }
}
#[doc = "Field `CH3` writer - Destination last request for channel 3"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH3_A, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH3_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH3_A::VALUE2)
    }
}
#[doc = "Field `CH4` reader - Destination last request for channel 4"]
pub type CH4_R = crate::BitReader<CH4_A>;
#[doc = "Destination last request for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::VALUE1,
            true => CH4_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH4_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH4_A::VALUE2
    }
}
#[doc = "Field `CH4` writer - Destination last request for channel 4"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH4_A, O>;
impl<'a, const O: u8> CH4_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH4_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH4_A::VALUE2)
    }
}
#[doc = "Field `CH5` reader - Destination last request for channel 5"]
pub type CH5_R = crate::BitReader<CH5_A>;
#[doc = "Destination last request for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::VALUE1,
            true => CH5_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH5_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH5_A::VALUE2
    }
}
#[doc = "Field `CH5` writer - Destination last request for channel 5"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH5_A, O>;
impl<'a, const O: u8> CH5_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH5_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH5_A::VALUE2)
    }
}
#[doc = "Field `CH6` reader - Destination last request for channel 6"]
pub type CH6_R = crate::BitReader<CH6_A>;
#[doc = "Destination last request for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::VALUE1,
            true => CH6_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH6_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH6_A::VALUE2
    }
}
#[doc = "Field `CH6` writer - Destination last request for channel 6"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH6_A, O>;
impl<'a, const O: u8> CH6_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH6_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH6_A::VALUE2)
    }
}
#[doc = "Field `CH7` reader - Destination last request for channel 7"]
pub type CH7_R = crate::BitReader<CH7_A>;
#[doc = "Destination last request for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7_A {
    #[doc = "0: Not last transaction in current block"]
    VALUE1 = 0,
    #[doc = "1: Last transaction in current block"]
    VALUE2 = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::VALUE1,
            true => CH7_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH7_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH7_A::VALUE2
    }
}
#[doc = "Field `CH7` writer - Destination last request for channel 7"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, CH7_A, O>;
impl<'a, const O: u8> CH7_W<'a, O> {
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH7_A::VALUE1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH7_A::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH0_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Destination last transaction request write enable for channel 0"]
pub type WE_CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH0_AW, O>;
impl<'a, const O: u8> WE_CH0_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH0_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH1_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Destination last transaction request write enable for channel 1"]
pub type WE_CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH1_AW, O>;
impl<'a, const O: u8> WE_CH1_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH1_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH2_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Destination last transaction request write enable for channel 2"]
pub type WE_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH2_AW, O>;
impl<'a, const O: u8> WE_CH2_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH2_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH3_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Destination last transaction request write enable for channel 3"]
pub type WE_CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH3_AW, O>;
impl<'a, const O: u8> WE_CH3_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH3_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH4_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH4` writer - Destination last transaction request write enable for channel 4"]
pub type WE_CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH4_AW, O>;
impl<'a, const O: u8> WE_CH4_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH4_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH4_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH5_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH5` writer - Destination last transaction request write enable for channel 5"]
pub type WE_CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH5_AW, O>;
impl<'a, const O: u8> WE_CH5_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH5_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH5_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH6_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH6` writer - Destination last transaction request write enable for channel 6"]
pub type WE_CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH6_AW, O>;
impl<'a, const O: u8> WE_CH6_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH6_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH6_AW::VALUE2)
    }
}
#[doc = "Destination last transaction request write enable for channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_CH7_AW {
    #[doc = "0: write disabled"]
    VALUE1 = 0,
    #[doc = "1: write enabled"]
    VALUE2 = 1,
}
impl From<WE_CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: WE_CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH7` writer - Destination last transaction request write enable for channel 7"]
pub type WE_CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LSTDSTREG_SPEC, WE_CH7_AW, O>;
impl<'a, const O: u8> WE_CH7_W<'a, O> {
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WE_CH7_AW::VALUE1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WE_CH7_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination last request for channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination last request for channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination last request for channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination last request for channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Destination last request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Destination last request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Destination last request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Destination last request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Destination last request for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Destination last request for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Destination last request for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Destination last request for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Destination last transaction request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WE_CH0_W<8> {
        WE_CH0_W::new(self)
    }
    #[doc = "Bit 9 - Destination last transaction request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WE_CH1_W<9> {
        WE_CH1_W::new(self)
    }
    #[doc = "Bit 10 - Destination last transaction request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WE_CH2_W<10> {
        WE_CH2_W::new(self)
    }
    #[doc = "Bit 11 - Destination last transaction request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WE_CH3_W<11> {
        WE_CH3_W::new(self)
    }
    #[doc = "Bit 12 - Destination last transaction request write enable for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch4(&mut self) -> WE_CH4_W<12> {
        WE_CH4_W::new(self)
    }
    #[doc = "Bit 13 - Destination last transaction request write enable for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch5(&mut self) -> WE_CH5_W<13> {
        WE_CH5_W::new(self)
    }
    #[doc = "Bit 14 - Destination last transaction request write enable for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch6(&mut self) -> WE_CH6_W<14> {
        WE_CH6_W::new(self)
    }
    #[doc = "Bit 15 - Destination last transaction request write enable for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch7(&mut self) -> WE_CH7_W<15> {
        WE_CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last Destination Transaction Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lstdstreg](index.html) module"]
pub struct LSTDSTREG_SPEC;
impl crate::RegisterSpec for LSTDSTREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lstdstreg::R](R) reader structure"]
impl crate::Readable for LSTDSTREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lstdstreg::W](W) writer structure"]
impl crate::Writable for LSTDSTREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSTDSTREG to value 0"]
impl crate::Resettable for LSTDSTREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
