#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HPTXFSIZ_SPEC>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HPTXFSIZ_SPEC>;
#[doc = "Field `PTXFSTADDR` reader - Host Periodic TxFIFO Start Address"]
pub type PTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - Host Periodic TxFIFO Start Address"]
pub type PTXFSTADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `PTXFSIZE` reader - Host Periodic TxFIFO Depth"]
pub type PTXFSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - Host Periodic TxFIFO Depth"]
pub type PTXFSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfstaddr(&mut self) -> PTXFSTADDR_W<HPTXFSIZ_SPEC, 0> {
        PTXFSTADDR_W::new(self)
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsize(&mut self) -> PTXFSIZE_W<HPTXFSIZ_SPEC, 16> {
        PTXFSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0200_0400"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
