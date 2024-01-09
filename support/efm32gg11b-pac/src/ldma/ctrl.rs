#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SYNCPRSSETEN` reader - Synchronization PRS Set Enable"]
pub type SYNCPRSSETEN_R = crate::FieldReader;
#[doc = "Field `SYNCPRSSETEN` writer - Synchronization PRS Set Enable"]
pub type SYNCPRSSETEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCPRSCLREN` reader - Synchronization PRS Clear Enable"]
pub type SYNCPRSCLREN_R = crate::FieldReader;
#[doc = "Field `SYNCPRSCLREN` writer - Synchronization PRS Clear Enable"]
pub type SYNCPRSCLREN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUMFIXED` reader - Number of Fixed Priority Channels"]
pub type NUMFIXED_R = crate::FieldReader;
#[doc = "Field `NUMFIXED` writer - Number of Fixed Priority Channels"]
pub type NUMFIXED_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&self) -> SYNCPRSSETEN_R {
        SYNCPRSSETEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&self) -> SYNCPRSCLREN_R {
        SYNCPRSCLREN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&self) -> NUMFIXED_R {
        NUMFIXED_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncprsseten(&mut self) -> SYNCPRSSETEN_W<CTRL_SPEC> {
        SYNCPRSSETEN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncprsclren(&mut self) -> SYNCPRSCLREN_W<CTRL_SPEC> {
        SYNCPRSCLREN_W::new(self, 8)
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    #[must_use]
    pub fn numfixed(&mut self) -> NUMFIXED_W<CTRL_SPEC> {
        NUMFIXED_W::new(self, 24)
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
#[doc = "DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x1700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1700_0000;
}
