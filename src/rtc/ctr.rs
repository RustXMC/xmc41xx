#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENB` reader - RTC Module Enable"]
pub struct ENB_R(crate::FieldReader<bool, bool>);
impl ENB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENB` writer - RTC Module Enable"]
pub struct ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENB_W<'a> {
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
#[doc = "Field `TAE` reader - Timer Alarm Enable for Hibernation Wake-up"]
pub struct TAE_R(crate::FieldReader<bool, bool>);
impl TAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAE` writer - Timer Alarm Enable for Hibernation Wake-up"]
pub struct TAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAE_W<'a> {
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
#[doc = "Field `ESEC` reader - Enable Seconds Comparison for Hibernation Wake-up"]
pub struct ESEC_R(crate::FieldReader<bool, bool>);
impl ESEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESEC` writer - Enable Seconds Comparison for Hibernation Wake-up"]
pub struct ESEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESEC_W<'a> {
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
#[doc = "Field `EMIC` reader - Enable Minutes Comparison for Hibernation Wake-up"]
pub struct EMIC_R(crate::FieldReader<bool, bool>);
impl EMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMIC` writer - Enable Minutes Comparison for Hibernation Wake-up"]
pub struct EMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMIC_W<'a> {
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
#[doc = "Field `EHOC` reader - Enable Hours Comparison for Hibernation Wake-up"]
pub struct EHOC_R(crate::FieldReader<bool, bool>);
impl EHOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EHOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHOC` writer - Enable Hours Comparison for Hibernation Wake-up"]
pub struct EHOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EHOC_W<'a> {
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
#[doc = "Field `EDAC` reader - Enable Days Comparison for Hibernation Wake-up"]
pub struct EDAC_R(crate::FieldReader<bool, bool>);
impl EDAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDAC` writer - Enable Days Comparison for Hibernation Wake-up"]
pub struct EDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> EDAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EMOC` reader - Enable Months Comparison for Hibernation Wake-up"]
pub struct EMOC_R(crate::FieldReader<bool, bool>);
impl EMOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMOC` writer - Enable Months Comparison for Hibernation Wake-up"]
pub struct EMOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `EYEC` reader - Enable Years Comparison for Hibernation Wake-up"]
pub struct EYEC_R(crate::FieldReader<bool, bool>);
impl EYEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EYEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EYEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EYEC` writer - Enable Years Comparison for Hibernation Wake-up"]
pub struct EYEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EYEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DIV` reader - RTC Clock Divider Value"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - RTC Clock Divider Value"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&self) -> TAE_R {
        TAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&self) -> ESEC_R {
        ESEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&self) -> EMIC_R {
        EMIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&self) -> EHOC_R {
        EHOC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&self) -> EDAC_R {
        EDAC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&self) -> EMOC_R {
        EMOC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&self) -> EYEC_R {
        EYEC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Module Enable"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W { w: self }
    }
    #[doc = "Bit 2 - Timer Alarm Enable for Hibernation Wake-up"]
    #[inline(always)]
    pub fn tae(&mut self) -> TAE_W {
        TAE_W { w: self }
    }
    #[doc = "Bit 8 - Enable Seconds Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn esec(&mut self) -> ESEC_W {
        ESEC_W { w: self }
    }
    #[doc = "Bit 9 - Enable Minutes Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emic(&mut self) -> EMIC_W {
        EMIC_W { w: self }
    }
    #[doc = "Bit 10 - Enable Hours Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn ehoc(&mut self) -> EHOC_W {
        EHOC_W { w: self }
    }
    #[doc = "Bit 11 - Enable Days Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn edac(&mut self) -> EDAC_W {
        EDAC_W { w: self }
    }
    #[doc = "Bit 13 - Enable Months Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn emoc(&mut self) -> EMOC_W {
        EMOC_W { w: self }
    }
    #[doc = "Bit 14 - Enable Years Comparison for Hibernation Wake-up"]
    #[inline(always)]
    pub fn eyec(&mut self) -> EYEC_W {
        EYEC_W { w: self }
    }
    #[doc = "Bits 16:31 - RTC Clock Divider Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR to value 0x7fff_0000"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_0000
    }
}
