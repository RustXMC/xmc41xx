#[doc = "Register `CSGCLRG` writer"]
pub struct W(crate::W<CSGCLRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCLRG_SPEC>;
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
impl From<crate::W<CSGCLRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCLRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD0R` writer - DAC0 run bit clear"]
pub struct CD0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CD0R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CC0R` writer - CMP0 run bit clear"]
pub struct CC0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CC0P` writer - CMP0 passive level clear"]
pub struct CC0P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0P_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CD1R` writer - DAC1 run bit clear"]
pub struct CD1R_W<'a> {
    w: &'a mut W,
}
impl<'a> CD1R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CC1R` writer - CMP1 run bit clear"]
pub struct CC1R_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CC1P` writer - CMP1 passive level clear"]
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CD2R` writer - DAC2 run bit clear"]
pub struct CD2R_W<'a> {
    w: &'a mut W,
}
impl<'a> CD2R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CC2R` writer - CMP2 run bit clear"]
pub struct CC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2R_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CC2P` writer - CMP2 passive level clear"]
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 run bit clear"]
    #[inline(always)]
    pub fn cd0r(&mut self) -> CD0R_W {
        CD0R_W { w: self }
    }
    #[doc = "Bit 1 - CMP0 run bit clear"]
    #[inline(always)]
    pub fn cc0r(&mut self) -> CC0R_W {
        CC0R_W { w: self }
    }
    #[doc = "Bit 2 - CMP0 passive level clear"]
    #[inline(always)]
    pub fn cc0p(&mut self) -> CC0P_W {
        CC0P_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 run bit clear"]
    #[inline(always)]
    pub fn cd1r(&mut self) -> CD1R_W {
        CD1R_W { w: self }
    }
    #[doc = "Bit 5 - CMP1 run bit clear"]
    #[inline(always)]
    pub fn cc1r(&mut self) -> CC1R_W {
        CC1R_W { w: self }
    }
    #[doc = "Bit 6 - CMP1 passive level clear"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 run bit clear"]
    #[inline(always)]
    pub fn cd2r(&mut self) -> CD2R_W {
        CD2R_W { w: self }
    }
    #[doc = "Bit 9 - CMP2 run bit clear"]
    #[inline(always)]
    pub fn cc2r(&mut self) -> CC2R_W {
        CC2R_W { w: self }
    }
    #[doc = "Bit 10 - CMP2 passive level clear"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG run bit clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgclrg](index.html) module"]
pub struct CSGCLRG_SPEC;
impl crate::RegisterSpec for CSGCLRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgclrg::W](W) writer structure"]
impl crate::Writable for CSGCLRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCLRG to value 0"]
impl crate::Resettable for CSGCLRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
