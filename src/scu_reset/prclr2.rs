#[doc = "Register `PRCLR2` writer"]
pub type W = crate::W<PRCLR2_SPEC>;
#[doc = "WDT Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<WDTRS_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` writer - WDT Reset Clear"]
pub type WDTRS_W<'a, REG> = crate::BitWriter<'a, REG, WDTRS_A>;
impl<'a, REG> WDTRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTRS_A::VALUE2)
    }
}
#[doc = "DMA0 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<DMA0RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` writer - DMA0 Reset Clear"]
pub type DMA0RS_W<'a, REG> = crate::BitWriter<'a, REG, DMA0RS_A>;
impl<'a, REG> DMA0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RS_A::VALUE2)
    }
}
#[doc = "FCE Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<FCERS_A> for bool {
    #[inline(always)]
    fn from(variant: FCERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` writer - FCE Reset Clear"]
pub type FCERS_W<'a, REG> = crate::BitWriter<'a, REG, FCERS_A>;
impl<'a, REG> FCERS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCERS_A::VALUE2)
    }
}
#[doc = "USB Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: De-assert reset"]
    VALUE2 = 1,
}
impl From<USBRS_A> for bool {
    #[inline(always)]
    fn from(variant: USBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` writer - USB Reset Clear"]
pub type USBRS_W<'a, REG> = crate::BitWriter<'a, REG, USBRS_A>;
impl<'a, REG> USBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_A::VALUE1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBRS_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Reset Clear"]
    #[inline(always)]
    pub fn wdtrs(&mut self) -> WDTRS_W<PRCLR2_SPEC> {
        WDTRS_W::new(self, 1)
    }
    #[doc = "Bit 4 - DMA0 Reset Clear"]
    #[inline(always)]
    pub fn dma0rs(&mut self) -> DMA0RS_W<PRCLR2_SPEC> {
        DMA0RS_W::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Reset Clear"]
    #[inline(always)]
    pub fn fcers(&mut self) -> FCERS_W<PRCLR2_SPEC> {
        FCERS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Reset Clear"]
    #[inline(always)]
    pub fn usbrs(&mut self) -> USBRS_W<PRCLR2_SPEC> {
        USBRS_W::new(self, 7)
    }
}
#[doc = "RCU Peripheral 2 Reset Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prclr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR2_SPEC;
impl crate::RegisterSpec for PRCLR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr2::W`](W) writer structure"]
impl crate::Writable for PRCLR2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR2 to value 0"]
impl crate::Resettable for PRCLR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
