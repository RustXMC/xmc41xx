#[doc = "Register `GNPTXFSIZ` reader"]
pub type R = crate::R<GNPTXFSIZ_SPEC>;
#[doc = "Register `GNPTXFSIZ` writer"]
pub type W = crate::W<GNPTXFSIZ_SPEC>;
#[doc = "Field `INEPTxF0StAddr` reader - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type INEPTX_F0ST_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTxF0StAddr` writer - IN Endpoint FIFO0 Transmit RAM Start Address"]
pub type INEPTX_F0ST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTxF0Dep` reader - IN Endpoint TxFIFO 0 Depth"]
pub type INEPTX_F0DEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTxF0Dep` writer - IN Endpoint TxFIFO 0 Depth"]
pub type INEPTX_F0DEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&self) -> INEPTX_F0ST_ADDR_R {
        INEPTX_F0ST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&self) -> INEPTX_F0DEP_R {
        INEPTX_F0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFO0 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn ineptx_f0st_addr(&mut self) -> INEPTX_F0ST_ADDR_W<GNPTXFSIZ_SPEC> {
        INEPTX_F0ST_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn ineptx_f0dep(&mut self) -> INEPTX_F0DEP_W<GNPTXFSIZ_SPEC> {
        INEPTX_F0DEP_W::new(self, 16)
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0010_011a"]
impl crate::Resettable for GNPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0010_011a;
}
