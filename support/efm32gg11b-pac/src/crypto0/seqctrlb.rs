#[doc = "Register `SEQCTRLB` reader"]
pub type R = crate::R<SEQCTRLB_SPEC>;
#[doc = "Register `SEQCTRLB` writer"]
pub type W = crate::W<SEQCTRLB_SPEC>;
#[doc = "Field `LENGTHB` reader - Buffer Length B in Bytes"]
pub type LENGTHB_R = crate::FieldReader<u16>;
#[doc = "Field `LENGTHB` writer - Buffer Length B in Bytes"]
pub type LENGTHB_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `DMA0PRESB` reader - DMA0 Preserve B"]
pub type DMA0PRESB_R = crate::BitReader;
#[doc = "Field `DMA0PRESB` writer - DMA0 Preserve B"]
pub type DMA0PRESB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1PRESB` reader - DMA1 Preserve B"]
pub type DMA1PRESB_R = crate::BitReader;
#[doc = "Field `DMA1PRESB` writer - DMA1 Preserve B"]
pub type DMA1PRESB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&self) -> LENGTHB_R {
        LENGTHB_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&self) -> DMA0PRESB_R {
        DMA0PRESB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&self) -> DMA1PRESB_R {
        DMA1PRESB_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn lengthb(&mut self) -> LENGTHB_W<SEQCTRLB_SPEC> {
        LENGTHB_W::new(self, 0)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    #[must_use]
    pub fn dma0presb(&mut self) -> DMA0PRESB_W<SEQCTRLB_SPEC> {
        DMA0PRESB_W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    #[must_use]
    pub fn dma1presb(&mut self) -> DMA1PRESB_W<SEQCTRLB_SPEC> {
        DMA1PRESB_W::new(self, 29)
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
#[doc = "Sequence Control B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrlb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrlb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQCTRLB_SPEC;
impl crate::RegisterSpec for SEQCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrlb::R`](R) reader structure"]
impl crate::Readable for SEQCTRLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seqctrlb::W`](W) writer structure"]
impl crate::Writable for SEQCTRLB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQCTRLB to value 0"]
impl crate::Resettable for SEQCTRLB_SPEC {
    const RESET_VALUE: u32 = 0;
}
