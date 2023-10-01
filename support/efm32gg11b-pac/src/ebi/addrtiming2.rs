#[doc = "Register `ADDRTIMING2` reader"]
pub type R = crate::R<ADDRTIMING2_SPEC>;
#[doc = "Register `ADDRTIMING2` writer"]
pub type W = crate::W<ADDRTIMING2_SPEC>;
#[doc = "Field `ADDRSETUP` reader - Address Setup Time"]
pub type ADDRSETUP_R = crate::FieldReader;
#[doc = "Field `ADDRSETUP` writer - Address Setup Time"]
pub type ADDRSETUP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADDRHOLD` reader - Address Hold Time"]
pub type ADDRHOLD_R = crate::FieldReader;
#[doc = "Field `ADDRHOLD` writer - Address Hold Time"]
pub type ADDRHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HALFALE` reader - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_R = crate::BitReader;
#[doc = "Field `HALFALE` writer - Half Cycle ALE Strobe Duration Enable"]
pub type HALFALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&self) -> ADDRSETUP_R {
        ADDRSETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&self) -> ADDRHOLD_R {
        ADDRHOLD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&self) -> HALFALE_R {
        HALFALE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrsetup(&mut self) -> ADDRSETUP_W<ADDRTIMING2_SPEC, 0> {
        ADDRSETUP_W::new(self)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn addrhold(&mut self) -> ADDRHOLD_W<ADDRTIMING2_SPEC, 8> {
        ADDRHOLD_W::new(self)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfale(&mut self) -> HALFALE_W<ADDRTIMING2_SPEC, 28> {
        HALFALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Address Timing Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addrtiming2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addrtiming2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDRTIMING2_SPEC;
impl crate::RegisterSpec for ADDRTIMING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrtiming2::R`](R) reader structure"]
impl crate::Readable for ADDRTIMING2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addrtiming2::W`](W) writer structure"]
impl crate::Writable for ADDRTIMING2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRTIMING2 to value 0x0707"]
impl crate::Resettable for ADDRTIMING2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
