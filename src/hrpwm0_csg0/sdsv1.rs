#[doc = "Register `SDSV1` reader"]
pub struct R(crate::R<SDSV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDSV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDSV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDSV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDSV1` writer"]
pub struct W(crate::W<SDSV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDSV1_SPEC>;
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
impl From<crate::W<SDSV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDSV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDSV1` reader - Shadow DAC reference value 1"]
pub struct SDSV1_R(crate::FieldReader<u16, u16>);
impl SDSV1_R {
    pub(crate) fn new(bits: u16) -> Self {
        SDSV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDSV1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDSV1` writer - Shadow DAC reference value 1"]
pub struct SDSV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDSV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    pub fn sdsv1(&self) -> SDSV1_R {
        SDSV1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    pub fn sdsv1(&mut self) -> SDSV1_W {
        SDSV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsv1](index.html) module"]
pub struct SDSV1_SPEC;
impl crate::RegisterSpec for SDSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdsv1::R](R) reader structure"]
impl crate::Readable for SDSV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdsv1::W](W) writer structure"]
impl crate::Writable for SDSV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDSV1 to value 0"]
impl crate::Resettable for SDSV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
