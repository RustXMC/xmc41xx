#[doc = "Register `SDCR` reader"]
pub type R = crate::R<SDCR_SPEC>;
#[doc = "Register `SDCR` writer"]
pub type W = crate::W<SDCR_SPEC>;
#[doc = "Field `SDTRV` reader - Shadow dead time rising value"]
pub type SDTRV_R = crate::FieldReader<u16>;
#[doc = "Field `SDTRV` writer - Shadow dead time rising value"]
pub type SDTRV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&self) -> SDTRV_R {
        SDTRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrv(&mut self) -> SDTRV_W<SDCR_SPEC> {
        SDTRV_W::new(self, 0)
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
#[doc = "HRC shadow dead time rising\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcr::R`](R) reader structure"]
impl crate::Readable for SDCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdcr::W`](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDCR to value 0x01"]
impl crate::Resettable for SDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
