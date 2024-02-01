#[doc = "Register `TRAPSET` writer"]
pub type W = crate::W<TRAPSET_SPEC>;
#[doc = "System OSC WDT Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOSCWDGT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_AW> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - System OSC WDT Trap Set"]
pub type SOSCWDGT_W<'a, REG> = crate::BitWriter<'a, REG, SOSCWDGT_AW>;
impl<'a, REG> SOSCWDGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SOSCWDGT_AW::VALUE2)
    }
}
#[doc = "System VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVCOLCKT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Set"]
pub type SVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, SVCOLCKT_AW>;
impl<'a, REG> SVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SVCOLCKT_AW::VALUE2)
    }
}
#[doc = "USB VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVCOLCKT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_AW> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Set"]
pub type UVCOLCKT_W<'a, REG> = crate::BitWriter<'a, REG, UVCOLCKT_AW>;
impl<'a, REG> UVCOLCKT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(UVCOLCKT_AW::VALUE2)
    }
}
#[doc = "Parity Error Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PET_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<PET_AW> for bool {
    #[inline(always)]
    fn from(variant: PET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Set"]
pub type PET_W<'a, REG> = crate::BitWriter<'a, REG, PET_AW>;
impl<'a, REG> PET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PET_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PET_AW::VALUE2)
    }
}
#[doc = "Brown Out Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRWNT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BRWNT_AW> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Set"]
pub type BRWNT_W<'a, REG> = crate::BitWriter<'a, REG, BRWNT_AW>;
impl<'a, REG> BRWNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BRWNT_AW::VALUE2)
    }
}
#[doc = "OSC_ULP WDG Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<ULPWDT_AW> for bool {
    #[inline(always)]
    fn from(variant: ULPWDT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDT` writer - OSC_ULP WDG Trap Set"]
pub type ULPWDT_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDT_AW>;
impl<'a, REG> ULPWDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDT_AW::VALUE2)
    }
}
#[doc = "Peripheral Bridge 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR0T_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BWERR0T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Set"]
pub type BWERR0T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR0T_AW>;
impl<'a, REG> BWERR0T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR0T_AW::VALUE2)
    }
}
#[doc = "Peripheral Bridge 1 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWERR1T_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<BWERR1T_AW> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Set"]
pub type BWERR1T_W<'a, REG> = crate::BitWriter<'a, REG, BWERR1T_AW>;
impl<'a, REG> BWERR1T_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BWERR1T_AW::VALUE2)
    }
}
#[doc = "Die Temperature Too High Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPHIT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<TEMPHIT_AW> for bool {
    #[inline(always)]
    fn from(variant: TEMPHIT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPHIT` writer - Die Temperature Too High Trap Set"]
pub type TEMPHIT_W<'a, REG> = crate::BitWriter<'a, REG, TEMPHIT_AW>;
impl<'a, REG> TEMPHIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPHIT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPHIT_AW::VALUE2)
    }
}
#[doc = "Die Temperature Too Low Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMPLOT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Set trap request"]
    VALUE2 = 1,
}
impl From<TEMPLOT_AW> for bool {
    #[inline(always)]
    fn from(variant: TEMPLOT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPLOT` writer - Die Temperature Too Low Trap Set"]
pub type TEMPLOT_W<'a, REG> = crate::BitWriter<'a, REG, TEMPLOT_AW>;
impl<'a, REG> TEMPLOT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPLOT_AW::VALUE1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TEMPLOT_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - System OSC WDT Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W<TRAPSET_SPEC> {
        SOSCWDGT_W::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W<TRAPSET_SPEC> {
        SVCOLCKT_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W<TRAPSET_SPEC> {
        UVCOLCKT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PET_W<TRAPSET_SPEC> {
        PET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BRWNT_W<TRAPSET_SPEC> {
        BRWNT_W::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP WDG Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdt(&mut self) -> ULPWDT_W<TRAPSET_SPEC> {
        ULPWDT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> BWERR0T_W<TRAPSET_SPEC> {
        BWERR0T_W::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> BWERR1T_W<TRAPSET_SPEC> {
        BWERR1T_W::new(self, 8)
    }
    #[doc = "Bit 12 - Die Temperature Too High Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn temphit(&mut self) -> TEMPHIT_W<TRAPSET_SPEC> {
        TEMPHIT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Die Temperature Too Low Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn templot(&mut self) -> TEMPLOT_W<TRAPSET_SPEC> {
        TEMPLOT_W::new(self, 13)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Trap Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAPSET_SPEC;
impl crate::RegisterSpec for TRAPSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapset::W`](W) writer structure"]
impl crate::Writable for TRAPSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAPSET to value 0"]
impl crate::Resettable for TRAPSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
