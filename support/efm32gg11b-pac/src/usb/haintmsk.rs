#[doc = "Register `HAINTMSK` reader"]
pub type R = crate::R<HAINTMSK_SPEC>;
#[doc = "Register `HAINTMSK` writer"]
pub type W = crate::W<HAINTMSK_SPEC>;
#[doc = "Field `HAINTMSK` reader - Channel Interrupt Mask for channel 0 - 13"]
pub type HAINTMSK_R = crate::FieldReader<u16>;
#[doc = "Field `HAINTMSK` writer - Channel Interrupt Mask for channel 0 - 13"]
pub type HAINTMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Interrupt Mask for channel 0 - 13"]
    #[inline(always)]
    pub fn haintmsk(&self) -> HAINTMSK_R {
        HAINTMSK_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Channel Interrupt Mask for channel 0 - 13"]
    #[inline(always)]
    #[must_use]
    pub fn haintmsk(&mut self) -> HAINTMSK_W<HAINTMSK_SPEC> {
        HAINTMSK_W::new(self, 0)
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
#[doc = "Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haintmsk::R`](R) reader structure"]
impl crate::Readable for HAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure"]
impl crate::Writable for HAINTMSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HAINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
