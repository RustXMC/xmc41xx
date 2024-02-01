#[doc = "Register `CGATSTAT2` reader"]
pub type R = crate::R<CGATSTAT2_SPEC>;
#[doc = "Field `WDT` reader - WDT Gating Status"]
pub type WDT_R = crate::BitReader<WDT_A>;
#[doc = "WDT Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::VALUE1,
            true => WDT_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDT_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDT_A::VALUE2
    }
}
#[doc = "Field `DMA0` reader - DMA0 Gating Status"]
pub type DMA0_R = crate::BitReader<DMA0_A>;
#[doc = "DMA0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0_A {
        match self.bits {
            false => DMA0_A::VALUE1,
            true => DMA0_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA0_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA0_A::VALUE2
    }
}
#[doc = "Field `FCE` reader - FCE Gating Status"]
pub type FCE_R = crate::BitReader<FCE_A>;
#[doc = "FCE Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<FCE_A> for bool {
    #[inline(always)]
    fn from(variant: FCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCE_A {
        match self.bits {
            false => FCE_A::VALUE1,
            true => FCE_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCE_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCE_A::VALUE2
    }
}
#[doc = "Field `USB` reader - USB Gating Status"]
pub type USB_R = crate::BitReader<USB_A>;
#[doc = "USB Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
impl USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::VALUE1,
            true => USB_A::VALUE2,
        }
    }
    #[doc = "Gating de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USB_A::VALUE1
    }
    #[doc = "Gating asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 1 - WDT Gating Status"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA0 Gating Status"]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - FCE Gating Status"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Gating Status"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Peripheral 2 Clock Gating Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgatstat2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSTAT2_SPEC;
impl crate::RegisterSpec for CGATSTAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgatstat2::R`](R) reader structure"]
impl crate::Readable for CGATSTAT2_SPEC {}
#[doc = "`reset()` method sets CGATSTAT2 to value 0"]
impl crate::Resettable for CGATSTAT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
